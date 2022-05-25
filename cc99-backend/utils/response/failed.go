package response

// Failed 用于异常状态 用于主动设置HTTP状态码 比如下载失败需要返回HTTP Status Code = 500时使用
func Failed(code int) Response {
	return Response{
		Type:       TypeFailed,
		FailedCode: code,
	}
}
