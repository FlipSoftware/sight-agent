FROM rust AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update
RUN apt install -y musl-tools musl-dev
RUN apt install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu

WORKDIR /app

COPY ./ .

RUN cargo build --target x86_64-unknow-linux-musl --release

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-kb-center ./
COPY --from=builder /app/.env ./

CMD [ "/app/rust-kb-center", "--db-name postgres --db-user postgres --db-user-password 123123" ]
