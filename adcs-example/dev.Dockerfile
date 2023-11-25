# Create Builder image
FROM --platform=$BUILDPLATFORM rust:1.74.0-alpine3.18

# Setup timezone
ARG tz=Europe/Paris

# Install required dependencies
RUN apk add --update alpine-sdk \
  musl-dev \
  g++ \
  make \
  libpq-dev \
  build-base \
  tzdata \
  libgcc \
  libpq \
  && rm -rf /var/cache/apk/* \
  && rm -rf /tmp/* \
  && rm -rf /var/log/* \
  && rm -rf /var/tmp/*

RUN cargo install cargo-watch --locked

# Create project directory
RUN mkdir -p /project
WORKDIR /project

ENV TZ=${tz}
ENV RUSTFLAGS="-C target-feature=-crt-static"

ENTRYPOINT ["cargo"]
