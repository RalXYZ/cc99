package response

// File 用于文件下载
func File(fileName string, fileData []byte) Response {
	return Response{
		Type:     TypeFile,
		File:     fileData,
		FileName: fileName,
	}
}
