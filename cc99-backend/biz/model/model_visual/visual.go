package model_visual

type VisualReq struct {
	Code string `json:"code" form:"code"`
}

type VisualResp struct {
	Res string `json:"res" form:"res"`
}
