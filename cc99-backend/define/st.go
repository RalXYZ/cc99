package define

import "fmt"

type St int64

const (
	StOk St = 0 // 正常

	StParamErr         St = 10001 // 参数错误
	StNoLoginUser      St = 10002 // 未登录
	StPermissionErr    St = 10003 // 无权限
	StLoginErr         St = 10004 // 账户或密码错误
	StDuplicateJoinErr St = 10005 // 账户已注册
	StNoUser           St = 10006 // 账户不存在
	StTokenExpired     St = 10007 // token过期
	StInvalidUserParam St = 10008 // 无效的用户注册参数
	StRPCErr           St = 20001 // RPC失败
	StServerErr        St = 20002 // 服务器错误

	StIOErr       St = 20000 //文件相关错误
	StDBErr       St = 30000 // 数据库错误
	StUrlErr      St = 40000 //URL错误
	StNoImage     St = 40001 //没有图片内容
	StNetworkErr  St = 50000 //网络错误
	StReadBodyErr St = 50001 //读取body错误
)

func (s St) String() string {
	switch s {
	case StOk:
		return ""
	case StInvalidUserParam:
		return "用户注册参数无效"
	case StParamErr:
		return "参数错误"
	case StNoLoginUser:
		return "未登录"
	case StPermissionErr:
		return "无权限"
	case StLoginErr:
		return "账户或密码错误"
	case StDuplicateJoinErr:
		return "账户已注册"
	case StNoUser:
		return "账户不存在"
	case StTokenExpired:
		return "token过期"
	case StRPCErr:
		return "服务器远程调用失败"
	case StServerErr:
		return "服务器错误"
	case StDBErr:
		return "数据库错误"
	}
	panic(fmt.Errorf("unknown St:%d", s))
}
