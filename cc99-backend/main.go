package main

import (
	"cc99-backend/biz"
	"cc99-backend/define"
	"log"
	"net/http"
	"os/exec"
	"time"
)

func init() {
	cmd := exec.Command(define.CC99Bin, "--version")
	if err := cmd.Start(); err != nil {
		log.Println("未找到cc99")
		log.Fatal(err)
	}
	err := cmd.Wait()
	if err != nil {
		log.Println("获取version失败，请手动重试")
		log.Fatal(err)
	}
	log.Println("已正确加载cc99")
}

func main() {
	router := biz.InitRouter()
	log.Println("[server] running on 5001")

	s := &http.Server{
		Addr:           ":5001",
		Handler:        router,
		ReadTimeout:    60 * time.Second,
		WriteTimeout:   60 * time.Second,
		MaxHeaderBytes: 1 << 24,
	}
	s.ListenAndServe()
}
