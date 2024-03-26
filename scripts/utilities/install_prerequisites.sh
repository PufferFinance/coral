#!/bin/bash

# Source the logging functions
source ./logging_functions.sh

# Ensure prerequisites are installed
ensure_prerequisites() {
    for cmd in "$@"; do
        if ! command -v "$cmd" &>/dev/null; then
            log_warn "$cmd is required but not installed. Trying to install..."

            # Install using apt-get (for Debian/Ubuntu)
            sudo apt-get update && sudo apt-get install -y "$cmd"

            # Check again
            if ! command -v "$cmd" &>/dev/null; then
                log_fatal "$cmd couldn't be installed. Please install manually."
                exit 1
            else
                log_success "$cmd successfully installed."
            fi
        else
            log_info "$cmd is already installed."
        fi
    done
}
