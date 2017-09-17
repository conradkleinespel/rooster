FROM alpine:latest

# install compile dependencies
RUN apk add --no-cache libxmu-dev libx11-dev pkgconf rust cargo python3 make

# compile
ADD . /var/rooster
RUN cd /var/rooster && cargo build --release

FROM alpine:latest

# install runtime dependencies
RUN apk add --no-cache xclip libxmu libx11 rust-stdlib jq

# run as non-root
RUN adduser -S -u 1000 rooster
WORKDIR /home/rooster
USER rooster
ENTRYPOINT ["/usr/bin/rooster"]

# keep only the compiled binary from the build
COPY --from=0 /var/rooster/target/release/rooster /usr/bin/rooster
