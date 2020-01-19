FROM matrix1986/rust:rustc-stable
RUN apt-get install clang llvm libclang-dev -y
WORKDIR /build
COPY . .
RUN mkdir -p /app/plugin
RUN export PATH="$HOME/.cargo/bin:$PATH" \
    && cargo update \
    && cargo build --release \
    && find target/release/ -maxdepth 1 -type f -name "*.so" -exec mv "{}" /app/plugin \; \
    && find target/release/ -maxdepth 1 -type f -perm /+x -exec mv "{}" /app \; \
    && cargo clean
WORKDIR /app
RUN mkdir excel
CMD ["/app/demo"]
