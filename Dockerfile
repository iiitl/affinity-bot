# Build stage
FROM rust

WORKDIR /app
COPY . .

# Install build dependencies
RUN apt-get update && apt-get install -y \
  build-essential \
  && rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt-get install -y \
  wget \
  gnupg2 \
  ca-certificates \
  && wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb \
  && apt-get install -y ./google-chrome-stable_current_amd64.deb \
  && rm google-chrome-stable_current_amd64.deb \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-shuttle
# Set environment variables
ENV RUST_LOG=info
ENV CHROME_PATH=/usr/bin/google-chrome

# Runtime configuration
EXPOSE 8000
CMD ["shuttle","run"]
