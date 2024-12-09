cp ./index.html target/wasm32-unknown-emscripten/release/

pushd target/wasm32-unknown-emscripten/release
ln -sf deps/*.wasm ./

python3 -m http.server 8000

popd