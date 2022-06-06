package response

// Redirect 用于重定向
func Redirect(code int, url string) Response {
	return Response{
		Type:         TypeRedirect,
		RedirectURL:  url,
		RedirectCode: code,
	}
}
