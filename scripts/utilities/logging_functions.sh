#!/bin/bash

# Logging functions with color coding
log_debug() {
    echo -e "\e[36m[DEBUG]\e[0m $1"  # Cyan color
}

log_info() {
    echo -e "\e[34m[INFO]\e[0m $1"   # Blue color
}

log_success() {
    echo -e "\e[32m[SUCCESS]\e[0m $1" # Green color
}

log_warn() {
    echo -e "\e[33m[WARNING]\e[0m $1" # Yellow color
}

log_error() {
    echo -e "\e[31m[ERROR]\e[0m $1"   # Red color
}

log_fatal() {
    echo -e "\e[91m[FATAL]\e[0m $1"   # Bright Red color
}

log_notice() {
    echo -e "\e[35m[NOTICE]\e[0m $1"  # Magenta color
}
