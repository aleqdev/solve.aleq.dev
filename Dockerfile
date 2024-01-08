FROM rust:1.75-alpine as builder

USER root

RUN apk --no-cache add ca-certificates libpq-dev musl-dev

RUN cargo install wasm-pack

RUN mkdir /server
WORKDIR /server

# setup cache
COPY ./server/Cargo.lock ./server/Cargo.toml ./

RUN mkdir -p auth/src && echo "pub fn x(){}" >> ./auth/src/lib.rs
COPY ./server/auth/Cargo.toml ./auth/Cargo.toml

RUN mkdir -p db/src && echo "pub fn x(){}" >> ./db/src/lib.rs
COPY ./server/db/Cargo.toml ./db/Cargo.toml

RUN mkdir -p language_module/src && echo "pub fn x(){}" >> ./language_module/src/lib.rs
COPY ./server/language_module/Cargo.toml ./language_module/Cargo.toml

RUN mkdir -p language_module_cpp/src && echo "pub fn x(){}" >> ./language_module_cpp/src/lib.rs
COPY ./server/language_module_cpp/Cargo.toml ./language_module_cpp/Cargo.toml

RUN mkdir -p language_module_python/src && echo "pub fn x(){}" >> ./language_module_python/src/lib.rs
COPY ./server/language_module_python/Cargo.toml ./language_module_python/Cargo.toml

RUN mkdir -p launcher/src && echo "pub fn x(){}" >> ./launcher/src/lib.rs
COPY ./server/launcher/Cargo.toml ./launcher/Cargo.toml

RUN mkdir -p web/src && echo "pub fn x(){}" >> ./web/src/lib.rs
COPY ./server/web/Cargo.toml ./web/Cargo.toml

RUN cargo build --release

# build wasm auth
COPY ./server/auth/ ./auth/
RUN touch ./auth/src/lib.rs;
RUN wasm-pack build --target web ./auth

# build
COPY ./server/ ./
RUN for DIR in $(ls -d */); \
    do touch $DIR/src/lib.rs; \
    done
RUN cargo build --release

RUN mkdir /program
RUN cp -R /server/web/static /program
COPY docker-config/debug/.env /program/.env
RUN cp /server/target/release/launcher /program/
RUN cp /server/auth/pkg/auth.js /program/static/auth.js
RUN cp /server/auth/pkg/auth_bg.wasm /program/static/auth_bg.wasm

FROM alpine:latest

RUN apk --no-cache add ca-certificates libpq

COPY --from=builder /program /program
WORKDIR /program/
CMD ./launcher
