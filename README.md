# Workshop Repo: Intro to Rust with MQTT

> *NOTE*: This is Work-In-Progress! Please check for updates a day before the workshop.

This GitHub repo will contain all the examples and workshops files we create during our time together.

## Install Rust

[Rustup](https://rustup.rs) provides you with all the software to compile and run Rust applications, e.g.

1. Cargo - build tool and package manager
2. `rustfmt` - Auto-formatting tool for Rust code
3. `clippy` - Linting for common mistakes

[and many more](https://rust-lang.github.io/rustup-components-history/). *Rustup* also allows you to install different compile targets and multiple toolchains, as well as keeping your toolchains up to date.

After installing, you should have a set of new command line tools available.

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

## Tasks

We want to learn basic Rust concepts by subscribing and publishing to an MQTT broker. We use the `rumqttc` crate and `test.mosquitto.org` as our MQTT broker. Code along as we go through the following tasks:

1. [ ] Check out dependencies
2. [ ] Create an MQTT Client
3. [ ] Subscribe to a topic
4. [ ] Discuss:
    - [ ] Mutable vs. Immutable declaration
    - [ ] Tuple destructiring
    - [ ] Unwrap
5. [ ] Create a `SensorData` struct
6. [ ] Derive common traits
7. [ ] Use a tuple type to abstract temperature
8. [ ] Implement
    - [ ] `From<f64>` for `Celsius`
    - [ ] `Display` for `Celsius`
9. [ ] Mock `SensorData` generation using a  `read_sensor_data` function
10. [ ] `read_sensor_data` can error, model accordingly!
11. [ ] Read and publish sensor data in its own thread
12. [ ] Convert data to JSON befor publishing
13. [ ] Discuss
    - [ ] Move semantics
    - [ ] `match` expressions
    - [ ] `Result` type
    - [ ] Shadowing
14. [ ] Retrieve and print sensor data from MQTT broker
15. [ ] Discuss
    - [ ] Ownership rules
    - [ ] Iterators
    - [ ] `if let` expressions
    - [ ] Data conversions to `&[u8]` (slices)
16. [ ] Refactor the main thread
17. [ ] Happy path programming and proper error handling
18. [ ] Discuss
    - [ ] `Box<dyn Error>` vs conversions


## Snippets

The file `snippets.md` contains code snippets we will use during the workshop. Feel free to copy and paste them into your editor. *NOTE*: The final code might differ from what you see in the snippets!
