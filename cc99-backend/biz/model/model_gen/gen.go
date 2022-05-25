package model_gen

type GenReq struct {
	Code string `json:"code" form:"code" binding:"required"`
}

type GenResp struct {
	Status string `json:"status"`
	File   string `json:"file"`
	Stdout string `json:"stdout"`
	Stderr string `json:"stderr"`
}
