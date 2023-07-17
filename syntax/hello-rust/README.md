# Install rust-lang enviroment

```shell
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

Install path:
 
  Linux/MacOS: $HOME/.cargo/bin

  Windows: $USERPROFILE\.cargo\bin

# How to rust commane

##  Get rust version

```shell
rustc --version
```

## Get cargo version

```shell
cargo --version
```

##  Upgrade rust-lang to latest

```shell
rustup update
```
##  Uninstall rust-lang

```shell
rustup self uninstall
```

# IDE 

We need to IDE to do programming by rust-lang, etc. VS code.

##  Add plugins

1.rust-analyzer

2.Even Better TOML

# How to Run

## Complie and generate an executable file

```shell
rustc main.rs
```

## Generate main file

command to `./main` to run

# How to build

## Create project

```shell
cargo new hello
```

It generates folders automatically

## Run project

```shell
cargo run
```

## Clean project

```shell
cargo clean
```

clean temp file (target/)

## Verify whether the code can compile properly

```shell
cargo check
```

Do not generate executable file 

## Build release version

```shell
cargo build --release
```

generate folder (target/release/)