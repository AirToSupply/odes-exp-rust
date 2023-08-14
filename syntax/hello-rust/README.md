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



# How to speed up loading source?

​		You can add a file named `config` in the path of `~/.cargo/`.  Configure the following content：

```toml
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

# 指定镜像 如：tuna、sjtu、ustc，或者 rustcc
replace-with = 'tuna'

# 注：以下源配置一个即可，无需全部
# 中国科学技术大学
[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index/"
# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
# rustcc社区
[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
# 字节跳动
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```

