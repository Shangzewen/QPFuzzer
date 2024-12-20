
#!/bin/bash

# Stop the script on errors
set -e

echo "Updating package lists..."
sudo apt update

echo "Installing Rust toolchain..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

echo "Installing build dependencies..."
sudo apt install -y build-essential libssl-dev pkg-config curl libglib2.0-dev clang cargo

echo "Installing Ninja build system..."
sudo apt install -y ninja-build

echo "Installing Pixman library..."
sudo apt install -y libpixman-1-dev

echo "Installing Python 3.12 development headers..."
sudo apt install -y python3.12-dev

echo "Installing Patchelf..."
sudo apt install -y patchelf

echo "Installing LLVM linker (lld)..."
sudo apt install -y lld

echo "Installing Python virtual environment module..."
sudo apt install -y python3.12-venv

echo "Setting up Python virtual environment..."
python3.12 -m venv venv
source ./venv/bin/activate

echo "Installing Python libraries in virtual environment..."
pip install --upgrade pip
pip install colorama scapy
pip3 install setuptools

echo "Setup completed successfully!"