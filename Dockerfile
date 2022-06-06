FROM registry.cn-hangzhou.aliyuncs.com/raynor/rust-npm:1.0.0 as builder

WORKDIR /app
COPY . .
ENV LLVM_SYS_130_PREFIX /usr
RUN apt install libz-dev -y
RUN cargo build --package cc99 --bin cc99 --release
RUN cd web/web-frontend && npm install && npm run build && mv build /srv && mv /srv/build /srv/cc99


FROM golang:1.18-bullseye as prod
EXPOSE 5001
RUN mkdir /backend && mkdir /app
WORKDIR /backend
RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list
RUN apt update
# cache deps before building and copying source so that we don't need to re-download as muchw
# and so that source changes don't invalidate our downloaded layer
ENV GO111MODULE=on \
    GOPROXY=https://goproxy.cn,direct
COPY ./web/web-backend/go.mod go.mod
COPY ./web/web-backend/go.sum go.sum
RUN go mod download
RUN go mod tidy
# src code
COPY ./web/web-backend .
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64  go build -o cc99-backend .
RUN chmod +x cc99-backend


#copy frontend and cc99 and header file
WORKDIR /app
COPY --from=builder /srv/cc99 /srv/cc99
COPY --from=builder /app/target/release/cc99 .
COPY --from=builder /app/include ./include

WORKDIR /backend
RUN mv /app/cc99 /backend
RUN mv /app/include /backend
RUN apt install -y clang-11
RUN ln -s /usr/bin/clang-11 /usr/bin/clang
ENV PATH "$PATH:/backend"
ENV TZ=Asia/Shanghai
ENTRYPOINT ["/backend/cc99-backend"]
