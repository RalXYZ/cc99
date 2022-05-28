package response

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"net/url"
	"runtime"
	"strings"

	"github.com/gin-gonic/gin"
	jsoniter "github.com/json-iterator/go"
)

// Response 定义Response的结构
type Response struct {
	Type         Type         // 返回类型
	Json         JSONResponse // JSON数据
	File         []byte       // 文件数据
	FileName     string       // 文件名
	FailedCode   int          // 出错时的http状态码
	RedirectURL  string       // 重定向url
	RedirectCode int          // 重定向code
}

// Write 将Response结构体写入HTTP Response
func (r *Response) Write(c *gin.Context) {
	switch r.Type {
	case TypeJSON:
		marshal, _ := jsoniter.ConfigCompatibleWithStandardLibrary.Marshal(r.Json)
		c.JSON(http.StatusOK, json.RawMessage(marshal))
	case TypeFile:
		escape := url.QueryEscape(r.FileName)
		c.Header("Content-Disposition", fmt.Sprintf("attachment; filename=\"%s\"", escape))
		c.Data(http.StatusOK, "application/octet-stream", r.File)
	case TypeRedirect:
		c.Redirect(r.RedirectCode, r.RedirectURL)
	case TypeFailed:
		c.Status(r.FailedCode)
	case TypeImage:
		c.File(r.FileName)
	}
}

// ToJSON 用于调试， 将response的json data 转换成JSON字符串，方便打印
func (r *Response) ToJSON() string {
	marshal, _ := jsoniter.ConfigCompatibleWithStandardLibrary.Marshal(r.Json)
	return string(marshal)
}

// ToLocalFile 用于调试，将文件保存在本地
func (r *Response) ToLocalFile() string {
	_, file, _, _ := runtime.Caller(1)
	var values = strings.Split(file, "aitour")
	var path = values[0] + "aitour" + "/"
	var targetPath = path + r.FileName

	switch r.Type {
	case TypeFile:
		if err := ioutil.WriteFile(targetPath, r.File, 0644); err != nil {
			panic("write local file failed")
		}

	default:
		err := fmt.Errorf("to local file only support TypeFile, but is:%v", r.Type)
		panic(err)
	}

	return "写入本地文件成功：" + targetPath
}

// String 将response的概要信息转为字符串 用于打日志
func (r *Response) String() string {
	switch r.Type {
	case TypeJSON: // JSON
		return fmt.Sprintf("%+v", struct {
			Type Type   // 类型
			Json string // JSON数据
		}{r.Type, r.ToJSON()})

	case TypeFile:
		return fmt.Sprintf("%+v", struct {
			Type     Type   // 类型
			File     string // 文件数据（只显示大小）
			FileName string // 文件名
		}{r.Type, fmt.Sprintf("<%d byte>", len(r.File)), r.FileName})

	default:
		return "<unknown>"
	}
}
