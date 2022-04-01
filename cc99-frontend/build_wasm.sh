wasm-pack build .. -- --features web --no-default-features
wasm-opt -Os pkg/gnc_vis_lib_bg.wasm -o pkg/gnc_vis_lib_bg.wasm

