package model_gen

type GenReq struct {
	CompileOptions string `json:"compileOptions" form:"compileOptions"`
	Code           string `json:"code" form:"code" binding:"required"`
}

type GenResp struct {
	ExitCode int    `json:"exitCode"`
	File     string `json:"file"`
	Stdout   string `json:"stdout"`
	Stderr   string `json:"stderr"`
}
