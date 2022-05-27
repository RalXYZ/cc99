package service_gen

import (
	"bytes"
	"cc99-backend/biz/model/model_gen"
	"cc99-backend/define"
	"cc99-backend/utils/Rand"
	"cc99-backend/utils/response"
	"fmt"
	"github.com/gin-gonic/gin"
	"io/ioutil"
	"os/exec"
)

func GenCode(c *gin.Context, data model_gen.GenReq) response.Response {
	codeFile, err := ioutil.TempFile("", "cc99.*.c")
	if err != nil {
		return response.JSONStWithMsg(define.StIOErr, err.Error())
	}
	defer codeFile.Close()

	_, err = codeFile.WriteString(data.Code)
	if err != nil {
		return response.JSONStWithMsg(define.StIOErr, err.Error())
	}
	fmt.Println()
	_ = codeFile.Sync()
	outputFile := Rand.RandomString(10)
	cmd := exec.Command(define.CC99Bin, "-o", fmt.Sprintf("runtime/%s", outputFile), codeFile.Name())

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	err = cmd.Start()
	if err != nil {
		return response.JSONStWithMsg(define.StIOErr, err.Error())
	}
	err = cmd.Wait()

	if err != nil {
		retCode := err.(*exec.ExitError).ExitCode()
		return response.JSONData(model_gen.GenResp{ExitCode: retCode, File: outputFile, Stdout: stdout.String(), Stderr: stderr.String()})
	}
	return response.JSONData(model_gen.GenResp{ExitCode: 0, File: outputFile, Stdout: stdout.String(), Stderr: stderr.String()})
}
