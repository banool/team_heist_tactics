# UI stage of the build
FROM node:14 as builder
WORKDIR /npm
COPY ui/ .
COPY src/types.proto .

# Get protoc
ENV protocversion=3.12.3
RUN apt-get update
RUN apt-get install autoconf automake libtool curl make g++ unzip
RUN wget -q https://github.com/protocolbuffers/protobuf/releases/download/v${protocversion}/protobuf-all-${protocversion}.tar.gz
RUN tar -xzf protobuf-all-${protocversion}.tar.gz
WORKDIR protobuf-${protocversion}
RUN ./configure
RUN make
RUN make install
RUN ldconfig
WORKDIR ..

# Generate types and build UI
RUN npm ci --only=production
RUN ls
RUN pwd
RUN ./generate_types.sh
RUN npm run prodbuild

# Server stage of the build
# FROM rust:1.44-alpine3.11 as build
FROM rust:1.44 as build

ENV app=tht
WORKDIR /${app}

# Use nightly
RUN rustup default nightly-2020-06-11 

# Copy in npm artifacts from previous iamge
COPY --from=builder /npm/dist/index.html /${app}/templates/
COPY --from=builder /npm/dist/static /${app}/templates/static

# Files listing dependencies
COPY Cargo.toml Cargo.lock ./

# Compile dependencies
RUN set -x\
 && mkdir src\
 && echo "fn main() {println!(\"broken\")}" > src/main.rs\
 && cargo build --release

# Copy source and rebuild
COPY src/ src/
COPY build.rs .
RUN set -x\
  && cargo build --release

# Finally run it all
COPY prod_run.sh .
EXPOSE 19996
ENV THT_IP_ADDRESS=0.0.0.0
ENV THT_PORT=19996
ENV THT_DEPLOYMENT_MODE=prod
ENV RUST_LOG=debug
ENV RUST_LOG_STYLE=always
CMD ["./prod_run.sh"]
