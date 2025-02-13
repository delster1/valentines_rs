FROM rust:1.84 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Use Debian instead of Alpine
FROM debian:bookworm-slim
# Install timezone package
RUN apt-get update && apt-get install -y tzdata

# Set timezone (change "America/Chicago" to your preferred timezone)
ENV TZ=America/Chicago
RUN ln -fs /usr/share/zoneinfo/$TZ /etc/localtime && dpkg-reconfigure -f noninteractive tzdata
COPY --from=builder /app/target/release/valentines_rs /usr/local/bin/valentines_rs

# Make sure the binary is executable
RUN chmod +x /usr/local/bin/valentines_rs

ENTRYPOINT ["/usr/local/bin/valentines_rs"]

