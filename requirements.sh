#!/usr/bin/env bash

if ! which cargo > /dev/null; then
		echo "cargo not found, installing now..."
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	else
		echo "cargo found!"
fi

if [ ! -d "venv" ]; then
	echo "venv not found, creating one..."
	python3 -m venv venv
fi

cargo install evcxr_jupyter
evcxr_jupyter --install

source venv/bin/activate
echo "installing requirements to python3 virtual environment..."
pip3 install -r scripts/requirements.txt