# Workshop Repo: Intro to Rust with MQTT

This GitHub repo will contain all the examples and workshops files we create during our time together.

## Install Rust

[Rustup](https://rustup.rs) provides you with all the software to compile and run Rust applications, e.g.

1. Cargo - build tool and package manager
2. `rustfmt` - Auto-formatting tool for Rust code
3. `clippy` - Linting for common mistakes

[and many more](https://rust-lang.github.io/rustup-components-history/). *Rustup* also allows you to install different compile targets and multiple toolchains, as well as keeping your toolchains up to date.

### Install Rust on Linux

![Rustup.rs website](https://i.imgur.com/3LI5HsW.png)

#### Prerequisites

You need `curl` and `cmake` installed on your system. Install both with the package manager of your choice/distribution. Alternatively, install the basic development tools for your distribution, e.g.

- for Arch Linux: `sudo pacman -S base-devel`
- for Ubuntu: `sudo apt install build-essential`
- for Centos: `sudo yum install gcc`
- for Solus: `sudo eopkg it -c system.devel`

#### Rustup

1. Go to [rustup.rs](https://rustup.rs)
2. Copy the command for Linux and run it in your terminal / command line of choice. The line should look like this:

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

*Troubleshooting*: Some Linux distributions don't allow to connect via TLS v1.2. Change the `--tlsv1.2` to `--tlsv1.3` if you encounter any issues.

You will be prompted with a text that ends with...

```
  default host triple: x86_64-unknown-linux-gnu
    default toolchain: stable (default)
              profile: default
 modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
```

Rust will be installed in your home directory. Usually, the default parameters are fine, unless you want to work exclusively and specifically with a different toolchain. You find a list of toolchains [here](https://rust-lang.github.io/rustup-components-history/). For the contents of this workshop, continue with the default and press `Enter`.

After installing, you will be prompted with the following message:

```
To configure your current shell, you need to source
the corresponding env file under $HOME/.cargo.

This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
```

3. If it didn't happen automatically, add the corresponding line to your shell configuration file. After installing, you should have a set of new command line tools available.

### Install Rust on MacOS

Installing Rust on MacOS works very similar to Linux. Go to [rustup.rs](https://rustup.rs), copy the command and execute them in your command line of choice. You might be prompted to install Apple's XCode command line tools:

```bash
$ xcode-select --install
````

### Install Rust on Windows

You can either install Rust on Windows via the [official installer `.exe` found on Rustup.rs](https://rustup.rs) or use the _Windows Subsystem for Linux_ (see above). For the `.exe` version, you will be prompted to install a linker via the Visual Studio Build Tools.

### Verify your Rust installation:

1. Open a Terminal/Shell/Command Line of your choice
2. Check out this repo
3. Navigate to this repository
4. Enter

```bash
$ cargo build
```

This will install all dependencies and create a binary of your application. If you see no errors, you're good to go.

> *NOTE*: It is highly recommended to do this _before_ we start with the workshop, not save internet bandwith and spot installation errors upfront. This will save you time and you can focus on the workshop content.

Run

```bash
$ cargo run
```

And check if the output is "Hello, MQTT".

### Updating Rust

If you've already Rust installed on your system, please update to version 1.81 before the workshop.

```bash
$ rustup update
```

## Recommended Editor

During the workshop, we will use [Visual Studio Code](https://code.visualstudio.com/) as editor. It's free, fast and very extensible. Making yourself familiar with VS Code is highly recommended.

However, working with VS Code is not required. If you have a preferred editor with Rust support you're more productive with, please feel free to use whatever you like. What we highyly recommend though, is checking if your editor has support for [Rust analyzer](https://rust-analyzer.github.io/).

## Recommended VS Code Extensions

To work effeciently, please install a couple of extensions that help you developing Rust. *Note*: Please don't install the recommendend Rust extension. It's outdated and the community decided to move to other tools. You can search and install VS Code extensions through the menu on the side

We recommend the following extensions:

### Essential

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer). This is the main extension for Rust development, with the best language support available. *Note*: This extension is also available for other IDEs and editors, check out [their website](https://rust-analyzer.github.io/)

- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb). All Rust code is compiled against LLVM. This extension helps debugging LLVM code inside VS Code

### Nice to have

- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates). This extension helps installing dependencies from crates.io

- [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml). TOML is the format that the dependency manager Cargo uses to manage dependencies. This extension helps formatting and editing TOML files

- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens). Inline errors

## Snippets

The file `snippets.md` contains code snippets we will use during the workshop. Feel free to copy and paste them into your editor. *NOTE*: The final code might differ from what you see in the snippets!
