#!/bin/bash

# Source the logging functions
source ./utilities/logging_functions.sh

# Function to check if a command exists
command_exists() {
  command -v "$@" >/dev/null 2>&1
}

# Check if the operating system is Ubuntu 20.04
os_name=$(lsb_release -si)
os_version=$(lsb_release -sr)

if [ "$os_name" != "Ubuntu" ] || [ "$os_version" != "20.04" ]; then
    log_error "This script requires Ubuntu 20.04 to run."
    exit 1
else
    log_success "Running on $os_name version $os_version."
fi

# Check for SGX1 or SGX2 support
if ! (grep -q sgx /proc/cpuinfo); then
  log_error "Your CPU does not support SGX1 or SGX2. Exiting."
  exit 1
else
  log_success "CPU supports SGX1/SGX2."
fi

# Check for FLC support
if ! (grep -q sgx_lc /proc/cpuinfo); then
  log_error "Your CPU does not support Flexible Launch Control (FLC). Exiting."
  exit 1
else
  log_success "CPU supports Flexible Launch Control (FLC)."
fi

# Check Linux Kernel version
kernel_version=$(uname -r | cut -d'-' -f1)
required_version="5.10"

if [[ "$(printf '%s\n' "$required_version" "$kernel_version" | sort -V | head -n1)" != "$required_version" ]]; then
  log_error "Your kernel version ($kernel_version) is below the required version ($required_version)."

  # Update the Kernel
  log_info "Updating the Linux Kernel..."
  sudo apt-get update
  sudo apt-get dist-upgrade -y
  log_info "Kernel upgraded. Please reboot your system for changes to take effect."
else
  log_success "Kernel version ($kernel_version) is up-to-date."
fi

# Check if Docker is installed
if ! command_exists docker; then
  log_info "Docker not found. Installing..."

  # Assuming Ubuntu-like system
  sudo apt-get update

  # Install packages to allow apt to use a repository over HTTPS
  sudo apt-get install \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg \
    lsb-release

  # Add Docker's official GPG key
  curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -

  # Set up the stable repository
  sudo add-apt-repository \
    "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) \
  stable"

  # Install Docker CE
  sudo apt-get update
  sudo apt-get install docker-ce docker-ce-cli containerd.io

  log_success "Docker installed successfully!"
else
  log_notice "Docker already installed!"
fi

# Ensure Docker daemon is running
if ! sudo systemctl is-active --quiet docker; then
  log_info "Starting Docker..."
  sudo systemctl start docker

  if ! sudo systemctl is-active --quiet docker; then
    log_error "Failed to start Docker. Please check the system logs or configurations."
    exit 1
  else
    log_success "Docker started successfully!"
  fi
else
  log_notice "Docker is already running!"
fi

# Check if the docker group exists and if user is part of it
if ! grep -q "^docker:" /etc/group; then
  sudo groupadd docker
  log_info "Docker group created."
fi

if ! id -nG "$USER" | grep -qw "docker"; then
  # Add user to the docker group
  sudo usermod -aG docker $USER
  log_info "Added $USER to the docker group. Please log out and log back in for the changes to take effect, or restart your session."
else
  log_notice "$USER is already in the docker group."
fi

# Check if SGX packages are already installed
if ! dpkg-query -W -f='${Status}' libsgx-epid 2>/dev/null | grep -q "ok installed"; then
  # If not installed, add SGX repo and install the packages
  log_info "SGX packages not found. Installing..."

  echo 'deb [arch=amd64] https://download.01.org/intel-sgx/sgx_repo/ubuntu focal main' | sudo tee /etc/apt/sources.list.d/intel-sgx.list
  wget -qO - https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key | sudo apt-key add -

  sudo apt update
  sudo apt install -y libsgx-epid libsgx-quote-ex libsgx-dcap-ql libsgx-urts libsgx-uae-service libsgx-dcap-default-qpl

  log_success "SGX packages installed successfully!"
else
  log_notice "SGX packages are already installed."
fi

# Verify SGX installation
aesmd_status=$(service aesmd status)

if echo "$aesmd_status" | grep -q "active (running)"; then
  log_success "SGX service is running!"
else
  log_warn "SGX service is not running. Please check its configuration."
fi