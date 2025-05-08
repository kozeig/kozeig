#!/bin/bash
set -e

# Colors for neat little output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Lut Language Builder${NC}"
echo "=========================="

# Detect the user's OS
OS="unknown"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    OS="windows"
fi

echo -e "Detected OS: ${YELLOW}$OS${NC}"

# Check for LLVM (16 specifically)
LLVM_VERSION="16"
LLVM_FOUND=false
LLVM_PATH=""

check_llvm() {
    if [[ "$OS" == "macos" ]]; then
        # Check Homebrew for LLVM
        if brew list --versions "llvm@$LLVM_VERSION" &>/dev/null; then
            LLVM_PATH=$(brew --prefix "llvm@$LLVM_VERSION")
            if [[ -d "$LLVM_PATH" ]]; then
                LLVM_FOUND=true
                return 0
            fi
        fi
    elif [[ "$OS" == "linux" ]]; then
        # Check the standard locations on Linux
        if [[ -d "/usr/lib/llvm-$LLVM_VERSION" ]]; then
            LLVM_PATH="/usr/lib/llvm-$LLVM_VERSION"
            LLVM_FOUND=true
            return 0
        fi
    elif [[ "$OS" == "windows" ]]; then
        # Check for LLVM in Program Files - I actually have no idea where LLVM is installed on Windows. This was what Claude suggested.
        # You can probably change this if needed.
        if [[ -d "/c/Program Files/LLVM" ]]; then
            LLVM_PATH="/c/Program Files/LLVM"
            LLVM_FOUND=true
            return 0
        fi
    fi

    return 1
}

install_llvm() {
    echo -e "${YELLOW}LLVM $LLVM_VERSION not found. Installing...${NC}"

    if [[ "$OS" == "macos" ]]; then
        if ! command -v brew &>/dev/null; then
            echo -e "${RED}Homebrew not found. Please install Homebrew first.${NC}"
            echo "Visit https://brew.sh"
            exit 1
        fi

        echo "Installing LLVM $LLVM_VERSION with Homebrew..."
        brew install "llvm@$LLVM_VERSION"
        LLVM_PATH=$(brew --prefix "llvm@$LLVM_VERSION")
        LLVM_FOUND=true

    elif [[ "$OS" == "linux" ]]; then
        if command -v apt-get &>/dev/null; then
            echo "Installing LLVM $LLVM_VERSION with apt..."
            sudo apt-get update
            sudo apt-get install -y llvm-$LLVM_VERSION llvm-$LLVM_VERSION-dev
            LLVM_PATH="/usr/lib/llvm-$LLVM_VERSION"
            LLVM_FOUND=true
        else
            echo -e "${RED}Could not find apt package manager. Please install LLVM $LLVM_VERSION manually.${NC}"
            exit 1
        fi

    elif [[ "$OS" == "windows" ]]; then
        echo -e "${RED}Automatic LLVM installation on Windows is not supported.${NC}"
        echo "Please download and install LLVM from https://releases.llvm.org/download.html"
        exit 1
    else
        echo -e "${RED}Unsupported operating system: $OS${NC}"
        exit 1
    fi
}

update_shell_profile() {
    PROFILE_FILE=""

    if [[ "$OS" == "macos" || "$OS" == "linux" ]]; then
        # Determine which shell is being used - Only tested on zsh and bash
        SHELL_NAME=$(basename "$SHELL")

        if [[ "$SHELL_NAME" == "zsh" ]]; then
            PROFILE_FILE="$HOME/.zshrc"
        elif [[ "$SHELL_NAME" == "bash" ]]; then
            if [[ "$OS" == "macos" ]]; then
                PROFILE_FILE="$HOME/.bash_profile"
            else
                PROFILE_FILE="$HOME/.bashrc"
            fi
        else
            echo -e "${YELLOW}Unsupported shell: $SHELL_NAME. You'll need to set the LLVM_SYS_${LLVM_VERSION}0_PREFIX variable manually.${NC}"
            return 1
        fi

        # Check if the environment variable is already set
        ENV_VAR_NAME="LLVM_SYS_${LLVM_VERSION}0_PREFIX"
        if grep -q "$ENV_VAR_NAME" "$PROFILE_FILE" 2>/dev/null; then
            echo "LLVM environment variable already set in $PROFILE_FILE"
        else
            # Add the environment variable to the profile
            echo "" >> "$PROFILE_FILE"
            echo "# Added by Lut Language build script" >> "$PROFILE_FILE"
            echo "export $ENV_VAR_NAME=\"$LLVM_PATH\"" >> "$PROFILE_FILE"
            echo "Added LLVM environment variable to $PROFILE_FILE"
            echo -e "${YELLOW}NOTE: You'll need to restart your terminal or run 'source $PROFILE_FILE' for the changes to take effect.${NC}"
        fi

        return 0
    else
        echo -e "${YELLOW}Shell profile update not supported on this OS. You'll need to set the LLVM_SYS_${LLVM_VERSION}0_PREFIX variable manually.${NC}"
        return 1
    fi
}

# Main build process
check_llvm || install_llvm

if [[ "$LLVM_FOUND" == "true" ]]; then
    echo -e "${GREEN}Using LLVM at: $LLVM_PATH${NC}"

    # Export for the current session
    export LLVM_SYS_${LLVM_VERSION}0_PREFIX="$LLVM_PATH"

    # Update their shell profile for future sessions
    update_shell_profile

    # Build the project
    echo -e "${GREEN}Building Lut language compiler...${NC}"
    cargo build --release

    # Finish HIM *hya*
    if [[ -f "./target/release/lut" ]]; then
        echo -e "${GREEN}Build successful!${NC}"
        echo ""
        echo "You can now use the Lut compiler with:"
        echo -e "${YELLOW}./target/release/lut build yourprogram.lut${NC}"
    else
        echo -e "${RED}Build failed. Check the output above for errors.${NC}"
        exit 1
    fi
else
    echo -e "${RED}Could not find or install LLVM $LLVM_VERSION.${NC}"
    exit 1
fi
