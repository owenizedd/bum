#!/usr/bin/env bash
set -euo pipefail

VERSION="v0.5.0"

if ! bun &>/dev/null; then
	curl -fsSL https://bun.sh/install | bash
fi

arch=$(uname -ms)

case arch in
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

if [[$arch == *"windows"*]]; then
	target="TODO"
fi

GITHUB=${GITHUB-"https://github.com"}

github_repo="$GITHUB/JulesGuesnon/bum"

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

curl --fail --location --progress-bar --output "$exe_compressed" "$bum_uri" ||
	error "Failed to download bum from \"$bum_uri\""

tar -xvf "$exe_compressed" || error "Failed on decompress the executable"

rm "$exe_compressed"

mv "$bum_folder_name/bum" $exe

rm -r $bum_folder_name

chmod +x "$exe" ||
	error 'Failed to set permissions on bum executable'

tildify() {
	if [[ $1 = $HOME/* ]]; then
		local replacement=\~/

		echo "${1/$HOME\//$replacement}"
	else
		echo "$1"
	fi
}

echo "bum was installed successfully to  "$exe""

refresh_command=''

tilde_bin_dir="$bin_dir"
quoted_install_dir=\"${install_dir//\"/\\\"}\"

if [[ $quoted_install_dir = \"$HOME/* ]]; then
	quoted_install_dir=${quoted_install_dir/$HOME\//\$HOME/}
fi

echo

case $(basename "$SHELL") in
fish)
	commands=(
		"set --export $install_env $quoted_install_dir"
		"set --export PATH $bin_env \$PATH"
	)

	fish_config=$HOME/.config/fish/config.fish
	tilde_fish_config=$(tildify "$fish_config")

	if [[ -w $fish_config ]]; then
		{
			echo -e '\n# bum'

			for command in "${commands[@]}"; do
				echo "$command"
			done
		} >>"$fish_config"

		info "Added \"$tilde_bin_dir\" to \$PATH in \"$tilde_fish_config\""

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
	)

	zsh_config=$HOME/.zshrc
	tilde_zsh_config=$(tildify "$zsh_config")

	if [[ -w $zsh_config ]]; then
		{
			echo -e '\n# bum'

			for command in "${commands[@]}"; do
				echo "$command"
			done
		} >>"$zsh_config"

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
		tilde_bash_config=$(tildify "$bash_config")

		if [[ -w $bash_config ]]; then
			{
				echo -e '\n# bum'

				for command in "${commands[@]}"; do
					echo "$command"
				done
			} >>"$bash_config"

			info "Added \"$tilde_bin_dir\" to \$PATH in \"$tilde_bash_config\""

			refresh_command="source $bash_config"
			set_manually=false
			break
		fi
	done

	if [[ $set_manually = true ]]; then
		echo "Manually add the directory to $tilde_bash_config (or similar):"

		for command in "${commands[@]}"; do
			info_bold "  $command"
		done
	fi
	;;
*)
	echo 'Manually add the directory to ~/.bashrc (or similar):'
	info_bold "  export $install_env=$quoted_install_dir"
	info_bold "  export PATH=\"$bin_env:\$PATH\""
	;;
esac
