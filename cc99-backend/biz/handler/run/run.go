package run

import (
	"cc99-backend/biz/model/model_run"
	"cc99-backend/biz/service/service_run"
	"cc99-backend/define"
	"cc99-backend/utils/response"
	"github.com/gin-gonic/gin"
)

func Run(c *gin.Context) {
	var req model_run.RunReq
	err := c.ShouldBind(&req)
	if err != nil {
		c.Set(define.CC99Response, response.JSONSt(define.StParamErr))
		return
	}
	c.Set(define.CC99Response, service_run.Run(c, req))
}
