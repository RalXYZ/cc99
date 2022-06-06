package mw

import (
	"cc99-backend/define"
	"cc99-backend/utils/response"
	"log"

	"github.com/gin-gonic/gin"
)

func ResponseMiddleware(c *gin.Context) {
	c.Next()
	value, exists := c.Get(define.CC99Response)
	if !exists {
		log.Println("[ResponseMiddleware] response not set!")
		return
	}
	resp, ok := value.(response.Response)
	if !ok {
		log.Println("[ResponseMiddleware] response type invalid!")
		return
	}
	resp.Write(c)
}
