# Get pre-compiled binary for protoc separately
FROM curlimages/curl:7.77.0 as protoc_build

WORKDIR /tmp
ENV protocversion=3.17.3
RUN curl -L https://github.com/protocolbuffers/protobuf/releases/download/v${protocversion}/protoc-${protocversion}-linux-x86_64.zip --output protoc.zip
RUN mkdir protoc_dl
RUN unzip protoc.zip -d protoc_dl

# UI stage of the build
FROM node:16 as frontend_build

# Copy in protoc
COPY --from=protoc_build /tmp/protoc_dl/bin/protoc /usr/bin

WORKDIR /src
COPY src/types.proto .

WORKDIR /npm
COPY ui/ .

# Generate types and build UI
RUN yarn install --frozen-lockfile
RUN ./generate_types.sh
RUN yarn run prodbuild

# Server stage of the build
FROM rust:1.62.1 as backend_build

ENV app=tht
WORKDIR /${app}

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

# Final stage
FROM gcr.io/distroless/cc:debug

# Copy out the built binary into the distroless build
COPY --from=backend_build /tht/target/release/team_heist_tactics /

# Copy in templates, data, static content, and more
COPY templates templates
COPY data data
COPY static/images static/images
COPY static/audio static/audio
COPY prod_run.sh .
COPY ui/src/components/main.css static/main.css

# Copy the frontend in from the frontend build stage
COPY --from=frontend_build /npm/dist/index.html /templates/play.html
COPY --from=frontend_build /npm/dist/static/* /static/

# Finally run it all
EXPOSE 19996
ENV THT_IP_ADDRESS=0.0.0.0
ENV THT_PORT=19996
ENV THT_DEPLOYMENT_MODE=prod
ENV RUST_LOG=debug
ENV RUST_LOG_STYLE=always
ENV HANDLES_FILE="data/handles.txt"
CMD ["./prod_run.sh"]
