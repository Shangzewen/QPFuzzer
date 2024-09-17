#!/usr/bin/env bash

sudo apt install -y clang lld meson patchelf libpixman-1-dev

if ! which cargo > /dev/null; then
		echo "cargo not found, installing now..."
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
		source ~/$HOME/.bashrc
	else
		echo "cargo found!"
fi


if ! which evcxr_jupyter > /dev/null; then
		# Optional
		echo "evcxr_jupyter not found, installing now..."
		cargo install evcxr_jupyter
		source ~/$HOME/.bashrc
		evcxr_jupyter --install
	else
		echo "evcxr_jupyter found!"
fi

# Install python3 virtual environment
if [ ! -d "venv" ]; then
	echo "venv not found, creating one..."
	python3 -m venv venv
fi

# Install python3 packages
source venv/bin/activate
echo "installing requirements to python3 virtual environment..."
pip3 install -r scripts/requirements.txt