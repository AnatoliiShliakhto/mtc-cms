FROM rust AS chef
USER root
RUN cargo install cargo-chef
RUN apt-get update && apt-get install -y libclang-dev librocksdb-dev llvm
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-gnu --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM alpine AS runtime
RUN addgroup -S seiko && adduser -S seiko -G seiko
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/mtc-cms /usr/local/bin/
USER seiko
CMD ["/usr/local/bin/mtc-cms"]
