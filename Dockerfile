FROM rust:latest as builder

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y libclang-dev cmake ffmpeg

WORKDIR /home/audio-to-text
COPY . .

RUN cargo install --path .

FROM rust:latest

WORKDIR /app
RUN mkdir ./upload

COPY --from=builder /home/audio-to-text/models ./models
COPY --from=builder /home/audio-to-text/static ./static
COPY --from=builder /home/audio-to-text/target/release .

ENTRYPOINT ["/app/audio-to-text"]

EXPOSE 2894
