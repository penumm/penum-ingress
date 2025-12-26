#!/bin/bash

# Script to run Foundry tests for penum-ingress
# This script should be run in an environment where Foundry is installed

set -e  # Exit immediately if a command exits with a non-zero status

echo "Setting up Foundry test environment..."

# Check if forge is installed
if ! command -v forge &> /dev/null; then
    echo "Error: forge is not installed."
    echo "Please install Foundry first:"
    echo "curl -L https://foundry.paradigm.xyz | bash"
    echo "source ~/.bashrc  # or restart your shell"
    echo "foundryup"
    exit 1
fi

echo "Foundry is installed. Version: $(forge --version)"

# Navigate to the foundry-test directory
cd "$(dirname "$0")"

echo "Running Foundry tests..."
forge test

echo "Tests completed successfully!"