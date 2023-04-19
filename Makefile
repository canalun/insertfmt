gen-release-bin-on-m1-mac:
	CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target aarch64-apple-darwin -v
	tar -zcvf aarch64-apple-darwin-v1.0.0.tar.gz ./target/aarch64-apple-darwin/release/insertfmt
	CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target x86_64-pc-windows-gnu -v
	tar -zcvf x86_64-pc-windows-gnu-v1.0.0.tar.gz ./target/x86_64-pc-windows-gnu/release/insertfmt.exe
	CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target x86_64-unknown-linux-gnu -v
	tar -zcvf x86_64-unknown-linux-gnu-v1.0.0.tar.gz ./target/x86_64-unknown-linux-gnu/release/insertfmt
	CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target x86_64-unknown-linux-musl -v
	tar -zcvf x86_64-unknown-linux-musl-v1.0.0.tar.gz ./target/x86_64-unknown-linux-musl/release/insertfmt

gen-wasm-for-extension:
	rm -rf ./vscode_extension/node_modules/insertfmt
	wasm-pack build --target bundler --out-dir ./vscode_extension/node_modules/insertfmt --release