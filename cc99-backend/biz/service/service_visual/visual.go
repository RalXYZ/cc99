package service_visual

import (
	"bytes"
	"cc99-backend/biz/model/model_visual"
	"cc99-backend/define"
	"cc99-backend/utils/response"
	"github.com/gin-gonic/gin"
	"os/exec"
	"strings"
)

func VisualCode(c *gin.Context, data model_visual.VisualReq) response.Response {
	cmd := exec.Command(define.CC99Bin, "-V", "-")
	var stdout bytes.Buffer
	cmd.Stdin = strings.NewReader(data.Code)
	cmd.Stdout = &stdout
	err := cmd.Start()
	if err != nil {
		return response.JSONStWithMsg(define.StIOErr, err.Error())
	}
	_ = cmd.Wait()

	return response.JSONData(model_visual.VisualResp{Res: stdout.String()})
}
