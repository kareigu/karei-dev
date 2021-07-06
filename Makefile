go-build:
	go build -v -o bin/main main.go

go-run:
	go run main.go

go-clean:
	go clean

trunk-install:
	cargo install trunk
	cargo install wasm-bindgen-cli

front:
	yarn css:build
	trunk build

docker:
	docker build -t mxr/mxrr-dev:new .