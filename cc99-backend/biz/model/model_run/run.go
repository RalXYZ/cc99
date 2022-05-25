package model_run

type RunReq struct {
	File  string `json:"file" binding:"required"`
	Stdin string `json:"stdin"`
}

type RunResp struct {
}
