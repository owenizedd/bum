#!/usr/bin/env bash
set -euo pipefail

# --- Helper Functions ---
error() { echo -e "[ERROR] $*" >&2; exit 1; }
info() { echo -e "[INFO] $*"; }
info_bold() { echo -e "\e[1m$*\e[0m"; }

# --- Dependency Check ---
for cmd in bun curl tar uname mkdir mv rm; do
    command -v "$cmd" >/dev/null 2>&1 || error "The command '$cmd' is required but not installed."
done

# --- Initial Definitions ---
VERSION="v0.6.1"

# Install Bun if not available
if ! command -v bun &>/dev/null; then
    info "Bun not found. Installing Bun..."
    curl -fsSL https://bun.sh/install | bash || error "Failed to install Bun."
fi

# Detect system architecture
arch=$(uname -ms)
case "$arch" in
    "Darwin x86_64")
        target="x86_64-apple-darwin"
        ;;
    "Darwin arm64")
        target="aarch64-apple-darwin"
        ;;
    "Linux aarch64" | "Linux arm64")
        target="aarch64-unknown-linux-gnu"
        ;;
    "Linux x86_64" | *)
        target="x86_64-unknown-linux-gnu"
        ;;
esac

# Define GitHub repository variables
GITHUB="${GITHUB:-https://github.com}"
github_repo="$GITHUB/owenizedd/bum"
bum_folder_name="bum-$VERSION-$target"
bum_uri="$github_repo/releases/download/$VERSION/bum-$VERSION-$target.tar.gz"

# Installation variables
install_env="BUM_INSTALL"
bin_env="\$$install_env/bin"
install_dir="${!install_env:-$HOME/.bum}"
bin_dir="$install_dir/bin"
exe="$bin_dir/bum"
exe_compressed="$bin_dir/bum.tar.gz"

# Create installation directory if needed
if [[ ! -d "$bin_dir" ]]; then
    mkdir -p "$bin_dir" || error "Failed to create directory \"$bin_dir\""
fi

# --- Download and Installation ---
info "Downloading bum from: $bum_uri"
curl --fail --location --progress-bar --output "$exe_compressed" "$bum_uri" \
    || error "Failed to download bum from \"$bum_uri\""

# Extract the tarball to a temporary directory
temp_dir=$(mktemp -d)
trap 'rm -rf "${temp_dir:?}"' EXIT

tar -xvf "$exe_compressed" -C "$temp_dir" \
    || error "Failed to extract the archive."
rm "$exe_compressed"

# Move the executable to the destination directory
if [[ -f "$temp_dir/$bum_folder_name/bum" ]]; then
    mv "$temp_dir/$bum_folder_name/bum" "$exe" \
        || error "Failed to move the executable to \"$exe\""
else
    error "Executable not found in the temporary directory."
fi

# Remove the extracted folder; use parameter expansion guards to avoid dangerous expansion
rm -rf "${temp_dir:?}/${bum_folder_name:?}"
mkdir -p "$install_dir/bun-versions"

chmod +x "$exe" || error "Failed to set execute permission on bum"

# Function to convert paths to use tilde notation when applicable
tildify() {
    if [[ "$1" == "$HOME/"* ]]; then
        echo "~/${1#"$HOME"/}"
    else
        echo "$1"
    fi
}

info "bum was successfully installed to \"$exe\""

# --- Environment Configuration ---
refresh_command=''
tilde_bin_dir="$(tildify "$bin_dir")"
quoted_install_dir="\"${install_dir//\"/\\\"}\""
if [[ "$quoted_install_dir" == "\"$HOME/"* ]]; then
    quoted_install_dir=${quoted_install_dir/"$HOME"/"\$HOME"}
fi

echo ""

# Function to add commands to a shell configuration file
add_commands_to_file() {
    local file="$1"
    shift
    local commands=("$@")
    if [[ -w "$file" ]]; then
        {
            echo -e "\n# bum"
            for command in "${commands[@]}"; do
                echo "$command"
            done
        } >> "$file"
        info "Added \"$tilde_bin_dir\" to PATH in \"$(tildify "$file")\""
        refresh_command="source $(tildify "$file")"
        return 0
    else
        info "Please add the following lines manually to $(tildify "$file"):"
        for command in "${commands[@]}"; do
            info_bold "  $command"
        done
        return 1
    fi
}

case "$(basename "$SHELL")" in
    fish)
        commands=(
            "set --export $install_env $quoted_install_dir"
            "set --export PATH $bin_env \$PATH"
        )
        fish_config="$HOME/.config/fish/config.fish"
        add_commands_to_file "$fish_config" "${commands[@]}"
        ;;
    zsh)
        commands=(
            "export $install_env=$quoted_install_dir"
            "export PATH=\"$bin_env:\$PATH\""
        )
        zsh_config="$HOME/.zshrc"
        add_commands_to_file "$zsh_config" "${commands[@]}"
        ;;
    bash)
        commands=(
            "export $install_env=$quoted_install_dir"
            "export PATH=$bin_env:\$PATH"
        )
        bash_configs=(
            "$HOME/.bashrc"
            "$HOME/.bash_profile"
        )
        if [[ -n "${XDG_CONFIG_HOME:-}" ]]; then
            bash_configs+=(
                "$XDG_CONFIG_HOME/.bash_profile"
                "$XDG_CONFIG_HOME/.bashrc"
                "$XDG_CONFIG_HOME/bash_profile"
                "$XDG_CONFIG_HOME/bashrc"
            )
        fi
        set_manually=true
        for bash_config in "${bash_configs[@]}"; do
            if add_commands_to_file "$bash_config" "${commands[@]}"; then
                set_manually=false
                break
            fi
        done
        if $set_manually; then
            info "Please add the following lines manually to $(tildify "${bash_configs[0]}") (or similar):"
            for command in "${commands[@]}"; do
                info_bold "  $command"
            done
        fi
        ;;
    *)
        echo "Please add the following lines manually to your shell configuration file (e.g., ~/.bashrc):"
        info_bold "  export $install_env=$quoted_install_dir"
        info_bold "  export PATH=\"$bin_env:\$PATH\""
        ;;
esac

if [[ -n "$refresh_command" ]]; then
    info "To update your shell environment, run:"
    info_bold "  $refresh_command"
fi
