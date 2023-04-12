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
	wasm-pack build --target=nodejs --out-dir ./vscode_extension/insertfmt_core_wasm --release