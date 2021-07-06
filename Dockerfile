FROM node:alpine as css_builder

WORKDIR /usr/src/mxrr-dev

COPY ./src ./src
COPY ./styles ./styles
COPY ./index.html ./index.html
COPY ./package.json ./package.json
COPY ./postcss.config.js ./postcss.config.js
COPY ./tailwind.config.js ./tailwind.config.js
COPY ./yarn.lock ./yarn.lock

RUN yarn
RUN yarn css:build


FROM rust:bullseye as rust_builder

WORKDIR /usr/src

RUN USER=root cargo new --bin mxrr-dev

WORKDIR /usr/src/mxrr-dev

RUN bash -cl "wget -qO- https://github.com/thedodd/trunk/releases/download/v0.11.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-"
RUN bash -cl "mv ./trunk /usr/bin/"

RUN bash -cl "wget -O wasm-bindgen.tar.gz https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.74/wasm-bindgen-0.2.74-x86_64-unknown-linux-musl.tar.gz \
    && tar -xf wasm-bindgen.tar.gz \
    && mv wasm-bindgen-0.2.74-x86_64-unknown-linux-musl wasm-bindgen"
RUN bash -cl "mv ./wasm-bindgen/wasm-bindgen /usr/bin/"
RUN bash -cl "mv ./wasm-bindgen/wasm-bindgen-test-runner /usr/bin/"
RUN bash -cl "rm -r ./wasm-bindgen/"

RUN rustup target add wasm32-unknown-unknown

RUN apt-get update && apt-get install build-essential -y

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN rm src/*.rs
COPY ./src ./src
COPY ./static ./static
COPY ./index.html ./index.html
COPY ./favicon.ico ./favicon.ico

COPY --from=css_builder /usr/src/mxrr-dev/styles ./styles

RUN trunk build --release

FROM golang:alpine

WORKDIR /go/src/mxrr-dev

COPY ./main.go ./main.go
COPY go.* ./
COPY ./utils ./utils
COPY ./api ./api
COPY ./Makefile ./Makefile

RUN go get -d -v ./...
RUN apk add --update make
RUN make go-build

COPY --from=rust_builder /usr/src/mxrr-dev/dist ./dist

CMD ./bin/main
