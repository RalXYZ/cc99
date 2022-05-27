package service_run

import (
	"bytes"
	"cc99-backend/biz/model/model_gen"
	"cc99-backend/biz/model/model_run"
	"cc99-backend/define"
	"cc99-backend/utils/response"
	"fmt"
	"github.com/gin-gonic/gin"
	"os"
	"os/exec"
	"strings"
)

func Run(c *gin.Context, data model_run.RunReq) response.Response {
	_, err := os.Stat(fmt.Sprintf("runtime/%s", data.File))
	if err != nil {
		return response.JSONStWithMsg(define.StIOErr, "don't have a file named "+data.File)
	}
	cmd := exec.Command("time", "10", fmt.Sprintf("runtime/%s", data.File))

	var stdout, stderr bytes.Buffer
	cmd.Stdin = strings.NewReader(data.Stdin)
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	err = cmd.Start()
	if err != nil {
		return response.JSONStWithMsg(define.StIOErr, err.Error())
	}
	err = cmd.Wait()
	if err != nil {
		retCode := err.(*exec.ExitError).ExitCode()
		return response.JSONData(model_gen.GenResp{ExitCode: retCode, Stdout: stdout.String(), Stderr: stderr.String()})
	}
	return response.JSONData(model_gen.GenResp{ExitCode: 0, Stdout: stdout.String(), Stderr: stderr.String()})
}
