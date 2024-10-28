FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y

WORKDIR /workspace
COPY . .

RUN cargo build --release
CMD ["cargo", "test"]
