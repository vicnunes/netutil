#!/bin/bash
# Build Linux binary using Docker on macOS

set -e

echo "Building netutil for Linux x86_64 using Docker..."

# Build the Docker image and compile for x86_64
docker build --platform linux/amd64 -f Dockerfile.build -t netutil-builder .

# Create a temporary container and copy the binary out
docker create --name netutil-temp netutil-builder
docker cp netutil-temp:/build/target/release/netutil-tui ./netutil-linux-x86_64
docker rm netutil-temp

echo "✓ Linux binary created: netutil-linux-x86_64"
echo ""
echo "To install on Ubuntu server:"
echo "  scp netutil-linux-x86_64 user@server:~/"
echo "  ssh user@server 'sudo mv netutil-linux-x86_64 /usr/local/bin/netutil && sudo chmod +x /usr/local/bin/netutil'"
