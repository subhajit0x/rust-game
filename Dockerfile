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
# This doesnt work as excpected bauce `cargo build` requires the main file to be there
# For now let's just not use docker layers

# ADD Cargo.toml Cargo.lock /app/
# RUN cargo build

# Install the rest of the application
ADD . /app/
RUN cargo build

CMD ["./bin/entrypoint.sh"]
