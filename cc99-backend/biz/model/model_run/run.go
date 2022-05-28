package model_run

type RunReq struct {
	File     string `json:"file" binding:"required"`
	ExecArgs string `json:"execArgs" form:"execArgs"`
	Stdin    string `json:"stdin"`
}

type RunResp struct {
	ExitCode int    `json:"exitCode"`
	Stdout   string `json:"stdout"`
	Stderr   string `json:"stderr"`
}
