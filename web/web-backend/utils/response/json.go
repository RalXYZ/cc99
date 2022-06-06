package response

import "cc99-backend/define"

// JSONData 用于正常情况 参数为body 如果没有body可以填nil
func JSONData(data interface{}) Response {
	return Response{
		Type: TypeJSON,
		Json: JSONResponse{
			St:   define.StOk,
			Msg:  "",
			Data: data,
		},
	}
}

// JSONSt 用于异常情况 参数为st状态码
func JSONSt(st define.St) Response {
	return Response{
		Type: TypeJSON,
		Json: JSONResponse{
			St:   st,
			Msg:  st.String(),
			Data: nil,
		},
	}
}

// JSONStWithMsg 用于异常情况 参数为st状态码和msg字符串
func JSONStWithMsg(st define.St, msg string) Response {
	return Response{
		Type: TypeJSON,
		Json: JSONResponse{
			St:   st,
			Msg:  msg,
			Data: nil,
		},
	}
}

// JSONResponse 定义JSONResponse的结构
type JSONResponse struct {
	St   define.St   `json:"st"`
	Msg  string      `json:"msg"`
	Data interface{} `json:"data"`
}

func (r *JSONResponse) SetSt(st define.St) {
	r.St = st
}
