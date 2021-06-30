go-build:
	go build -v -o bin/main main.go

go-run:
	go run main.go

go-clean:
	go clean

trunk-build:
	trunk build

trunk-serve:
	trunk serve

trunk-install:
	cargo install trunk
	cargo install wasm-bindgen-cli