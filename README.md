## Bum - Bun Version Manager

<img src="https://github.com/owenizedd/bum/assets/26961166/b231b1ff-dcde-4cc1-a0de-fa0f4964e54e" height="auto" width="150" style="border-radius:50%">
<br/>

<a href="https://github.com/owenizedd/bum/actions/workflows/rust.yml"><img alt="GitHub CI Status" src="https://img.shields.io/github/actions/workflow/status/owenizedd/bum/rust.yml?label=CI&logo=GitHub"></a> <br/>

# ⚠️ This fork is only here to release ARM version of the binary

Introducing Bum, a fast Bun version manager written in Rust from scratch.

<img src=".docs/bum-quick-demo.gif" width="600">

## How to install:

1. Open your terminal and execute:

```
curl -fsSL https://github.com/JulesGuesnon/bum/raw/main/install.sh | bash
```

> This installation will install bun for you and also bum altogether.  
> Bum is supported on **Linux x86_64** and **Darwin x86_64** (Mac OS)  
> You can enter `uname -ms` command in your terminal to see yours 2. Restart terminal or `source ~/.zshrc` or `source ~/.bashrc` depending on your terminal.

## How to use:

- You can just run `bum` without any parameter to see default help menu.

### Commands

- `bum use <version>`
  - Change the current active bun version, e.g. `bum use 1.0.3`. This will automatically use v1.0.3.
  - If there's no target version is installed in the local, it will install that version then use it.
  - Else, it will just use that version directly as the active version.
- `bum remove <version>`
  - Remove the installed version locally.
    > This feature will only remove local copy, but if you're using the removed version, you will still be able to use that version, but once you change to other version, you will not be able to change to that version anymore.
  - In the future we will automatically switch to the latest version available upon removal of the version.
- `bum list`
  - Show all local installed versions of Bun.
- `bum list-remote`

  - Show all remote versions of Bun that could be installed.

- .bumrc file
  - When file exists, everytime you use `bum use` command without `<version>` argument, Bum will try to use the version from the .bumrc file.

### Future features (possibly)

- `bum default <version>`

> Any contribution is appreciated, have any doubts/questions/suggestions/ideas? Drop them in the Discussion page.
