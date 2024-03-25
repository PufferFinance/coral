#!/bin/bash

LATEST_VALIDATOR_VERSION=1.1.3

# Source the logging functions
source ./utilities/logging_functions.sh

# Function to check if a command exists
command_exists() {
  command -v "$@" >/dev/null 2>&1
}

volume_name="Puffer-Validator-Backup"

# Check if the Docker volume already exists
if docker volume inspect "$volume_name" >/dev/null 2>&1; then
  log_warn "Docker volume $volume_name already exists."
  read -p "Do you want to create another volume? (yes/no) " user_choice

  if [[ $user_choice == "yes" ]]; then
    read -p "Enter the name of the new Docker volume (default $volume_name): " new_volume_name
    new_volume_name=${new_volume_name:-$volume_name}

    if [ "$new_volume_name" == "$volume_name" ]; then
      read -p "You have chosen to keep the existing volume. Are you sure? (yes/no) " confirm_choice
      if [[ $confirm_choice != "yes" ]]; then
        log_info "New volume creation cancelled by the user."
      fi
    fi

    # Create the new Docker volume
    log_info "Creating Docker volume $new_volume_name..."
    if ! docker volume create "$new_volume_name" >/dev/null 2>&1; then
      log_error "Failed to create Docker volume $new_volume_name."
    fi
  else
    log_info "User chose not to create another volume."
  fi
else
  # The volume does not exist, create it
  log_info "Docker volume $volume_name does not exist. Creating it now..."
  if docker volume create "$volume_name" >/dev/null 2>&1; then
    log_success "Docker volume $volume_name created successfully!"
  else
    log_error "Failed to create Docker volume $volume_name."
  fi
fi

# Verify and inspect the volume
log_info "Verifying the existence of $volume_name volume..."
if docker volume inspect "$volume_name" >/dev/null 2>&1; then
  log_success "$volume_name volume exists!"
  log_info "Inspecting $volume_name volume..."
  if volume_info=$(docker volume inspect "$volume_name"); then
    log_info "Volume details:"
    echo "$volume_info"
  else
    log_warn "Failed to inspect $volume_name volume."
  fi
else
  log_error "$volume_name volume does not exist."
fi

# Prompt the user for the version of the Docker image
read -p "Enter the version of the Puffer validator image you want to use (default ${LATEST_VALIDATOR_VERSION}): " version

# Set the default version if no input is provided
version=${version:-${LATEST_VALIDATOR_VERSION}}

# Attempt to pull the specified Docker image
if docker pull pufferfinance/puffer_validator:"$version"; then
  log_success "Docker image puffer_validator:$version pulled successfully!"
else
  log_error "Failed to pull the Docker image version $version!"
  exit 1
fi

# Run the container
if docker run -itd --network host \
    --mount type=volume,source=Puffer-Validator-Backup,destination=/Validator \
    -v /var/run/aesmd:/var/run/aesmd \
    --device /dev/sgx/enclave \
    --device /dev/sgx/provision \
    --name puffer_secure_signer_container \
    pufferfinance/puffer_validator:$version; then
  log_success "Container deployed successfully!"
  sleep 5 # Wait for a few seconds to ensure the container has time to start its process
else
  log_error "Failed to start the Docker container. Please check Docker logs or configurations."
  exit 1
fi

if ! docker ps | grep -q "puffer_secure_signer_container"; then
  log_error "Container puffer_secure_signer_container exited unexpectedly. Displaying its logs for diagnosis:"

  # Displaying the last logs from the container
  docker logs puffer_secure_signer_container
  exit 1
else
  log_success "Container puffer_secure_signer_container is running successfully!"
fi
