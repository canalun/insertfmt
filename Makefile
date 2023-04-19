TAG=$(shell git describe --tags)

gen-release-bin-on-m1-mac:
	rm -rf ./target
	rm -rf ./dist
	mkdir ./dist
	CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target aarch64-apple-darwin -v
	tar -zcvf ./dist/aarch64-apple-darwin-${TAG}.tar.gz ./target/aarch64-apple-darwin/release/insertfmt
	CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target x86_64-pc-windows-gnu -v
	tar -zcvf ./dist/x86_64-pc-windows-gnu-${TAG}.tar.gz ./target/x86_64-pc-windows-gnu/release/insertfmt.exe
	CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target x86_64-unknown-linux-musl -v
	tar -zcvf ./dist/x86_64-unknown-linux-musl-${TAG}.tar.gz ./target/x86_64-unknown-linux-musl/release/insertfmt
# exclude the following targets because wasm_bindgen seems not to be supported
# CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target x86_64-unknown-linux-gnu -v
# tar -zcvf ./dist/x86_64-unknown-linux-gnu-v1.0.0.tar.gz ./target/x86_64-unknown-linux-gnu/release/insertfmt


gen-wasm-for-extension:
	rm -rf ./vscode_extension/insertfmt_core
	wasm-pack build --target bundler --out-dir ./vscode_extension/insertfmt_core --release