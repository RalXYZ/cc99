package visual

import (
	"cc99-backend/biz/model/model_visual"
	"cc99-backend/biz/service/service_visual"
	"cc99-backend/define"
	"cc99-backend/utils/response"
	"github.com/gin-gonic/gin"
)

func Visual(c *gin.Context) {
	var req model_visual.VisualReq
	err := c.ShouldBind(&req)
	if err != nil {
		c.Set(define.CC99Response, response.JSONSt(define.StParamErr))
		return
	}
	c.Set(define.CC99Response, service_visual.VisualCode(c, req))
}
