FROM matrix1986/rust:rustc-stable as builder
RUN apt-get install clang llvm libclang-dev -y
WORKDIR /build
COPY . .
RUN mkdir -p /app/plugin
RUN . /root/.cargo/env \
    && cargo update \
    && cargo build --release \
    && find target/release/ -maxdepth 1 -type f -name "*.so" -exec mv "{}" /app/plugin \; \
    && find target/release/ -maxdepth 1 -type f -perm /+x -exec mv "{}" /app \; \
    && cargo clean

FROM matrix1986/rust:ubuntu as demo
WORKDIR /app
RUN mkdir excel
COPY --from=builder /app .
CMD ["/app/demo"]
