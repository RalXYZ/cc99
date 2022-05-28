package biz

import (
	"cc99-backend/biz/handler/gen"
	"cc99-backend/biz/handler/ping"
	"cc99-backend/biz/handler/run"
	"cc99-backend/mw"
	"github.com/gin-gonic/gin"
)

func InitRouter() *gin.Engine {
	r := gin.New()
	r.Use(gin.Logger())
	r.Use(mw.RecoverMiddleware)
	r.Use(mw.CorsMiddleware) // CORS中间件  	必须在路由前配置
	api := r.Group("/api",
		mw.ResponseMiddleware, // response middleware
	)
	api.GET("/ping", ping.Pong) // ping
	api.POST("/gen", gen.Gen)
	api.POST("/run", run.Run)
	return r

}
