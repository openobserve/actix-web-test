# syntax=docker/dockerfile:1

# FROM public.ecr.aws/docker/library/rust:bullseye as builder
FROM public.ecr.aws/zinclabs/rust:bullseye-sccache as builder

# RUN rustup toolchain install nightly-2023-03-28
# RUN rustup default nightly-2023-03-28
# RUN rustup target add x86_64-unknown-linux-gnu
# RUN diff -u <(rustc --print cfg) <(rustc -C target-cpu=native --print cfg)
RUN rustc --version && sccache --version

WORKDIR /app
COPY . /app

RUN cargo build --release

# FROM gcr.io/distroless/cc as runtime
FROM public.ecr.aws/debian/debian:bullseye-slim as runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates
RUN apt-get install -y curl htop iftop sysstat procps lsof net-tools
RUN update-ca-certificates
COPY --from=builder /app/target/release/actix-web-test /zincobserve
CMD ["/zincobserve"]
