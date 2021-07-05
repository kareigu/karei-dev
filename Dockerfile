FROM node:alpine as css_builder

WORKDIR /usr/src/mxrr-dev

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./static ./static
COPY ./styles ./styles
COPY ./index.html ./index.html
COPY ./favicon.ico ./favicon.ico
COPY ./package.json ./package.json
COPY ./postcss.config.js ./postcss.config.js
COPY ./tailwind.config.js ./tailwind.config.js
COPY ./yarn.lock ./yarn.lock

RUN yarn
RUN yarn css:build


FROM rust as rust_builder

WORKDIR /usr/src

RUN USER=root cargo new --bin mxrr-dev

WORKDIR /usr/src/mxrr-dev

RUN cargo install trunk

RUN cargo install wasm-bindgen-cli

RUN rustup target add wasm32-unknown-unknown

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN rm src/*.rs
COPY ./src ./src
COPY ./static ./static
COPY ./styles ./styles
COPY ./index.html ./index.html
COPY ./favicon.ico ./favicon.ico
COPY ./package.json ./package.json
COPY ./postcss.config.js ./postcss.config.js
COPY ./tailwind.config.js ./tailwind.config.js
COPY ./yarn.lock ./yarn.lock

COPY --from=css_builder /usr/src/mxrr-dev/styles ./styles

RUN trunk build --release

FROM golang:alpine

WORKDIR /go/src/mxrr-dev

COPY ./main.go ./main.go
COPY go.* ./
COPY ./utils ./utils
COPY ./apiv1 ./apiv1
COPY ./Makefile ./Makefile
COPY ./certs ./certs

RUN go get -d -v ./...
RUN apk add --update make
RUN make go-build

COPY --from=rust_builder /usr/src/mxrr-dev/dist ./dist

CMD ./bin/main
