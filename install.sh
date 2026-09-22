#!/usr/bin/env bash
set -euo pipefail

# Fill missing functions

Color_Off=''
Red=''
Green=''
Dim=''
Bold_White=''

if [[ -t 1 ]]; then
    # Reset
    Color_Off='\033[0m' # Text Reset

    # Regular Colors
    Red='\033[0;31m'   # Red
    Green='\033[0;32m' # Green
    Dim='\033[0;2m'    # White

    # Bold
    Bold_White='\033[1m'    # Bold White
fi

error() {
    echo -e "${Red}error${Color_Off}:" "$@" >&2
    exit 1
}

info() {
    echo -e "${Dim}$* ${Color_Off}"
}

info_bold() {
    echo -e "${Bold_White}$* ${Color_Off}"
}

success() {
    echo -e "${Green}$* ${Color_Off}"
}

# The tilde is literal here: this output is only ever shown to the user.
# shellcheck disable=SC2088
tildify() {
    if [[ $1 == "$HOME/"* ]]; then
        echo "~/${1#"$HOME"/}"
    else
        echo "$1"
    fi
}

add_to_config() {
    local config_file="$1"
    local marker="# bum"
    shift
    local commands=("$@")

    if grep -q "^$marker" "$config_file" 2>/dev/null; then
        info "Configuration block already present in $config_file, skipping addition."
        return
    fi

    echo -e "\n$marker" >> "$config_file"
    for command in "${commands[@]}"; do
        echo "$command" >> "$config_file"
    done

    info "Added configuration block to $config_file."
}

# Everything below relies on these being present. Checked up front so the
# script fails with a clear message instead of midway through an install.
# bun is deliberately not in this list: it is installed further down.
for cmd in curl tar uname mktemp mkdir mv rm chmod grep basename; do
	command -v "$cmd" >/dev/null 2>&1 ||
		error "'$cmd' is required to install bum but was not found in PATH"
done

VERSION="v0.7.22"

if ! command -v bun >/dev/null 2>&1; then
	info "bun was not found, installing it first..."
	curl -fsSL https://bun.sh/install | bash ||
		error "Failed to install bun, see https://bun.sh/docs/installation"
fi

arch=$(uname -ms)

case $arch in
'Darwin x86_64')
	target=x86_64-apple-darwin
	;;
'Darwin arm64')
	target=aarch64-apple-darwin
	;;
'Linux aarch64' | 'Linux arm64')
	target=aarch64-unknown-linux-gnu
	;;
'Linux x86_64' | *)
	target=x86_64-unknown-linux-gnu
	;;
esac

GITHUB=${GITHUB:-"https://github.com"}

github_repo="$GITHUB/owenizedd/bum"

bum_folder_name="bum-$VERSION-$target"

bum_uri=$github_repo/releases/download/$VERSION/bum-$VERSION-"$target".tar.gz

install_env=BUM_INSTALL
bin_env=\$$install_env/bin

install_dir=${!install_env:-$HOME/.bum}
bin_dir=$install_dir/bin
exe=$bin_dir/bum
exe_compressed=$bin_dir/bum.tar.gz

if [[ ! -d $bin_dir ]]; then
	mkdir -p "$bin_dir" ||
		error "Failed to create install directory \"$bin_dir\""
fi

# Retry function with exponential backoff
download_with_retry() {
	local url=$1
	local output=$2
	local max_attempts=3
	local timeout=30
	local attempt=1
	local wait_time=2

	while [ "$attempt" -le "$max_attempts" ]; do
		info "Downloading bum (attempt $attempt/$max_attempts)..."

		if curl --fail --location --progress-bar --max-time "$timeout" --output "$output" "$url"; then
			success "Download successful!"
			return 0
		fi

		if [ "$attempt" -lt "$max_attempts" ]; then
			info "Download failed. Retrying in ${wait_time}s..."
			sleep "$wait_time"
			wait_time=$((wait_time * 2))  # Exponential backoff
			timeout=$((timeout + 30))      # Increase timeout for next attempt
		fi

		attempt=$((attempt + 1))
	done

	return 1
}

# The tarball is unpacked in a scratch directory instead of the caller's
# working directory, so a failed install never leaves files behind and
# never overwrites something that happens to share the archive's name.
temp_dir=$(mktemp -d) ||
	error "Failed to create a temporary directory"
trap 'rm -rf "${temp_dir:?}"; rm -f "${exe_compressed:?}"' EXIT

download_with_retry "$bum_uri" "$exe_compressed" ||
	error "Failed to download bum from \"$bum_uri\" after 3 attempts"

tar -xf "$exe_compressed" -C "$temp_dir" ||
	error "Failed on decompress the executable"

[[ -f "$temp_dir/$bum_folder_name/bum" ]] ||
	error "Expected \"$bum_folder_name/bum\" inside the archive but it was not there"

mv "$temp_dir/$bum_folder_name/bum" "$exe" ||
	error "Failed to move the bum executable to \"$exe\""

mkdir -p "$install_dir/bun-versions"

chmod +x "$exe" ||
	error 'Failed to set permissions on bum executable'

success "bum was installed successfully to \"$exe\""

refresh_command=''

quoted_install_dir=\"${install_dir//\"/\\\"}\"
bun_bin_dir="$HOME/.bun/bin"

if [[ $quoted_install_dir == "\"$HOME/"* ]]; then
	quoted_install_dir=${quoted_install_dir/$HOME\//\$HOME/}
fi

echo

case $(basename "$SHELL") in
fish)
	commands=(
		"set --export $install_env $quoted_install_dir"
		"set --export PATH $bin_env \$PATH"
		"set --export PATH $bun_bin_dir \$PATH"
	)

	fish_config=$HOME/.config/fish/config.fish
	tilde_fish_config=$(tildify "$fish_config")

	if [[ -w $fish_config ]]; then
		add_to_config "$fish_config" "${commands[@]}"

		refresh_command="source $tilde_fish_config"
	else
		echo "Manually add the directory to $tilde_fish_config (or similar):"

		for command in "${commands[@]}"; do
			info_bold "  $command"
		done
	fi
	;;
zsh)

	commands=(
		"export $install_env=$quoted_install_dir"
		"export PATH=\"$bin_env:\$PATH\""
		"export PATH=\"$bun_bin_dir:\$PATH\""
	)

	zsh_config=$HOME/.zshrc
	tilde_zsh_config=$(tildify "$zsh_config")

	if [[ -w $zsh_config ]]; then
		add_to_config "$zsh_config" "${commands[@]}"

		refresh_command="exec $SHELL"
	else
		echo "Manually add the directory to $tilde_zsh_config (or similar):"

		for command in "${commands[@]}"; do
			info_bold "  $command"
		done
	fi
	;;
bash)

	commands=(
		"export $install_env=$quoted_install_dir"
		"export PATH=$bin_env:\$PATH"
		"export PATH=$bun_bin_dir:\$PATH"
	)

	bash_configs=(
		"$HOME/.bashrc"
		"$HOME/.bash_profile"
	)

	if [[ ${XDG_CONFIG_HOME:-} ]]; then
		bash_configs+=(
			"$XDG_CONFIG_HOME/.bash_profile"
			"$XDG_CONFIG_HOME/.bashrc"
			"$XDG_CONFIG_HOME/bash_profile"
			"$XDG_CONFIG_HOME/bashrc"
		)
	fi

	set_manually=true
	for bash_config in "${bash_configs[@]}"; do
		if [[ -w $bash_config ]]; then
			add_to_config "$bash_config" "${commands[@]}"

			refresh_command="source $(tildify "$bash_config")"
			set_manually=false
			break
		fi
	done

	if [[ $set_manually == true ]]; then
		echo "Manually add the directory to $(tildify "${bash_configs[0]}") (or similar):"

		for command in "${commands[@]}"; do
			info_bold "  $command"
		done
	fi
	;;
*)
	echo 'Manually add the directory to ~/.bashrc (or similar):'
	info_bold "  export $install_env=$quoted_install_dir"
	info_bold "  export PATH=\"$bin_env:\$PATH\""
	info_bold "  export PATH=\"$bun_bin_dir:\$PATH\""
	;;
esac

if [[ -n $refresh_command ]]; then
	echo
	info "To get started, run:"
	info_bold "  $refresh_command"
fi
