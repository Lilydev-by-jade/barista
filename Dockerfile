##
# Adapted from this GitHub answer: https://github.com/emk/rust-musl-builder/issues/151#issuecomment-1515373012
##

# Chef
FROM messense/rust-musl-cross:x86_64-musl as chef

RUN cargo install cargo-chef
WORKDIR /barista

# Planner
FROM chef as planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder
FROM chef as builder 

RUN apt-get update -y
RUN apt-get install -y pkg-config libssl-dev

COPY --from=planner /barista/recipe.json recipe.json

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .
RUN ls

RUN cargo build --release --target x86_64-unknown-linux-musl

# Run :)
FROM scratch

COPY --from=builder /barista/target/x86_64-unknown-linux-musl/release/barista /usr/local/bin/barista


ENTRYPOINT [ "barista" ]
