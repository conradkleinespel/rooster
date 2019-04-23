FROM alpine:latest

# install compile dependencies
RUN apk add --no-cache libxmu-dev libx11-dev pkgconf rust cargo python3 make openssl-dev libsodium-dev xclip jq

# run as non-root
RUN adduser --system -u 1000 rooster
WORKDIR /home/rooster
ENTRYPOINT ["/usr/bin/rooster"]

# compile
ADD . /home/rooster/src
RUN cargo install --path /home/rooster/src --root /usr

# run as non-root
USER rooster
