#!/bin/bash

# Source the logging functions
source ./logging_functions.sh

# Source the prerequisites check
source ./install_prerequisites.sh

function get_latest_release() {
    # Check if the required arguments are provided
    if [[ $# -ne 2 ]]; then
        log_error "Usage: get_latest_release <repo_owner> <repo_name>"
        return 1
    fi

    REPO_OWNER=$1
    REPO_NAME=$2

    API_URL="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"

    # Fetch the release information
    RESPONSE=$(curl -s "$API_URL")

    # Check if there's a rate-limiting or other issue
    if echo "$RESPONSE" | jq -e '.message' &>/dev/null; then
        MESSAGE=$(echo "$RESPONSE" | jq -r '.message')
        log_error "$MESSAGE"
        return 1
    fi

    # Extract tag_name and published_at using jq
    NAME=$(echo "$RESPONSE" | jq -r '.name')
    TAG_NAME=$(echo "$RESPONSE" | jq -r '.tag_name')
    PUBLISHED_AT=$(echo "$RESPONSE" | jq -r '.published_at')

    # Print the result
    echo "Name: $NAME"
}
