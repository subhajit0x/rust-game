FROM rust:1.54.0

WORKDIR /app

ENV DEBIAN_FRONTEND=noninteractive

# Install host dependencies
RUN apt-get update -qy \
    && apt-get install -y \
    libasound2-dev \
    libudev-dev \
    # Cleaning cache:
    && apt-get purge -y --auto-remove -o APT::AutoRemove::RecommendsImportant=false \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Install dependencies
ADD Cargo.toml Cargo.lock /app/
RUN cargo build

# Install the rest of the application
ADD . /app/

CMD ["./bin/entrypoint.sh"]
