#!/usr/bin/env bash
wasm-pack build .. --target web -- --features web --no-default-features    
#npx wasm-opt -Os ../pkg/cc99_visual_lib_bg.wasm -o ../pkg/cc99_visual_lib_bg.wasm

