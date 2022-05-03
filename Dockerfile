FROM registry.cn-hangzhou.aliyuncs.com/raynor/rust-npm:1.0.0

WORKDIR /app

COPY . .
RUN cd cc99-frontend && chmod +x build_wasm.sh && ./build_wasm.sh && npm install && npm run build && mv build /srv && mv /srv/build /srv/cc99


