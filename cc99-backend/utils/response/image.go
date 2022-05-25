package response

func Image(fileName string) Response {
	return Response{
		Type:     TypeImage,
		FileName: fileName,
	}
}
