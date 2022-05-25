package ping

import "github.com/gin-gonic/gin"

func Pong(c *gin.Context) {
	c.JSON(200, gin.H{"st": 0, "msg": "pong!"})
}
