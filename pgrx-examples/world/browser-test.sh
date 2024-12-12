cp ./index.html ./defs.js ./easywasi.js target/wasm32-unknown-emscripten/release/

mkdir -p target/wasm32-unknown-emscripten/release/bin

cp ../hello-world/target/wasm32-unknown-unknown/release/helloworld.wasm target/wasm32-unknown-emscripten/release/bin

pushd target/wasm32-unknown-emscripten/release
ln -sf deps/*.wasm ./

python3 -m http.server 8001

popd