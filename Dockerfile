# UI stage of the build
FROM node:14 as builder
WORKDIR /npm
COPY ui/ .
COPY src/types.proto .

# Get pre-compiled binary for protoc
ENV protocversion=3.12.3
RUN wget -q https://github.com/protocolbuffers/protobuf/releases/download/v${protocversion}/protoc-${protocversion}-linux-x86_64.zip
RUN mkdir protoc_dl
RUN unzip protoc-${protocversion}-linux-x86_64.zip -d protoc_dl
RUN cp protoc_dl/bin/protoc /usr/bin

# Generate types and build UI
RUN npm ci --only=production
RUN ./generate_types.sh
RUN npm run prodbuild

# Server stage of the build
# FROM rust:1.44-alpine3.11 as build
FROM rust:1.44 as build

ENV app=tht
WORKDIR /${app}

# Use nightly
RUN rustup default nightly-2020-06-11 

# Copy in HTML templates
COPY templates templates

# Copy in npm artifacts from previous iamge
COPY --from=builder /npm/dist/index.html /${app}/templates/play.html
COPY --from=builder /npm/dist/static /${app}/static

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
RUN set -x && cargo build --release
COPY prod_run.sh .

# Copy in images
COPY static/images /${app}/static/images

# Copy out the built binary into the distroless build
FROM gcr.io/distroless/cc:debug
COPY --from=build /tht/target/release/team_heist_tactics /
COPY --from=build /tht/templates /templates
COPY --from=build /tht/static /static
COPY --from=build /tht/prod_run.sh /

# Copy in data
COPY data data

# Finally run it all
EXPOSE 19996
ENV THT_IP_ADDRESS=0.0.0.0
ENV THT_PORT=19996
ENV THT_DEPLOYMENT_MODE=prod
ENV RUST_LOG=debug
ENV RUST_LOG_STYLE=always
ENV HANDLES_FILE="data/handles.txt"
CMD ["./prod_run.sh"]
