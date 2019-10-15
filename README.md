# iv

Rust语言智能于视频库。


#pragma once

#include "rule_type.hpp"
#include <iv/app/node_type.hpp>
#include <iv/app/group_type.hpp>

namespace maa {

	struct AlarmInfo;
	using AlarmId = cx::StrongId<int64_t, AlarmInfo>;

	/// 报警类型
	enum class AlarmType : int
	{
		NONE = 0,

		//通用报警
		VIDEO_LOSS = 101,			///< 视频检查：信号丢失
		CAMERA_BLOCKED = 102,		///< 视频检查：摄像机遮挡
		CAMERA_MOVED = 103,			///< 视频检查：摄像机移位
		UNATTENDED = 121,			///< 物品：遗留物
		REMOVAL = 122,				///< 物品：移除

		//定制报警：ATM机
		ATM_SLOT = 401,				///< 插卡口异常
		ATM_KEYBOARD = 402,			///< 键盘异常
		ATM_DAMAGE = 411,			///< 破坏ATM
		ATM_FUZZY_FACE = 421,		///< 脸部特征不清
		ATM_FACE = 422,				///< 脸部出现

		//定制报警：防护舱
		CABIN_TAILGATING = 501,		///< 尾随进入
		CABIN_THRUST = 502,			///< 强行推入

		//定制报警：加钞间
		BACKROOM_NUM_LIMIT = 601,	///< 加钞间人数限制
		BACKROOM_SQUAT = 611,		///< 加钞间下蹲

		//定制报警：自助银行大厅
		HALL_LOITERING = 701,		///< 徘徊
		HALL_RETENTION = 702,		///< 滞留
		HALL_FIGHTING = 711,		///< 打斗
		HALL_SEIZING = 712,			///< 挟持

		//定制报警：无特定区域
		DAMAGE = 801,				///< 破坏行为(设备等)
	};
	cx_enum_20v(AlarmType, NONE, VIDEO_LOSS, CAMERA_BLOCKED, CAMERA_MOVED, UNATTENDED, REMOVAL
		, ATM_SLOT, ATM_KEYBOARD, ATM_DAMAGE, ATM_FUZZY_FACE, ATM_FACE
		, CABIN_TAILGATING, CABIN_THRUST
		, BACKROOM_NUM_LIMIT, BACKROOM_SQUAT
		, HALL_LOITERING, HALL_RETENTION, HALL_FIGHTING, HALL_SEIZING
		, DAMAGE);

	/// 获取本地名
	string local_name(AlarmType type);

	/// 来源信息
	struct FromInfo
	{
		RuleId rule_id;			///< 规则ID
		DeviceId device_id;		///< 报警设备ID
		GroupId group_id;		///< 报警设备所在组ID
		NodeId node_id;			///< 所在网点ID
	};
	cx_struct_4f(FromInfo, rule_id, device_id, group_id, node_id);


	/// 内部保留信息
	struct ReservedInfo
	{
		floats probs;		///< 分类器输出
	};
	cx_struct_1f(ReservedInfo, probs);

	/// 报警信息
	struct AlarmInfo
	{
		using Id = AlarmId;

		Id id;					///< 报警事件ID
		AlarmType type;			///< 报警类型

		TimePoint time;			///< 绝对时间，格式:"2011-11-01 13:45:23.120"
		strings images;			///< 截图信息(部分图像可能不存在)
		string ico_file;		///< 关注区域图标
		string record;			///< 录像文件
		Duration timestamp;		///< 相对录像开始时间的相对时标

		int confidence{};		///< 置信度[0, 100]

        FromInfo from;          ///< 报警来源信息

		ReservedInfo reserved;		///< 内部保留信息，不需要呈现

	public:
		/// 获取描述字符串
		string to_string() const;

		/// 获取描述字符串
		string to_local_string() const;

		/// 替换路径前缀
		AlarmInfo replace_path(string_view src_prefix, string_view dst_prefix) const;
	};
	cx_struct_10f(AlarmInfo, id, type, time, images, ico_file, record, timestamp, confidence, from, reserved);

	/// 报警信息集合
	using AlarmInfos = vector<AlarmInfo>;

}
