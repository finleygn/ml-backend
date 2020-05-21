FROM rust:1.43.0 as build

# create new rust project
RUN USER=root cargo new --bin mser
WORKDIR /mser

# copy cargo info
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# odd way to cache deps
RUN cargo build --release
RUN rm src/*.rs

# build
COPY ./src ./src
RUN rm ./target/release/deps/mser*
RUN cargo build --release



FROM rust:1.43.0

COPY --from=build /mser/target/release/mser .

CMD ["./mser"]