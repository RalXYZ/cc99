package gen

import (
	"cc99-backend/biz/model/model_gen"
	"cc99-backend/biz/service/service_gen"
	"cc99-backend/define"
	"cc99-backend/utils/response"
	"github.com/gin-gonic/gin"
)

func Gen(c *gin.Context) {
	var req model_gen.GenReq
	err := c.ShouldBind(&req)
	if err != nil {
		c.Set(define.CC99Response, response.JSONSt(define.StParamErr))
		return
	}
	c.Set(define.CC99Response, service_gen.GenCode(c, req))
}
