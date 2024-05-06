# Hello World!

## ref

- [官网：www.rust-lang.org](https://www.rust-lang.org/)
- [文档：《The Rust Programming Language》](https://learnku.com/docs/rust-lang/2018)

## 1、安装

- Mac

```cmd
$ curl https://sh.rustup.rs -sSf | sh
```

- 安装过程

```cmd
// 下载安装器

info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /Users/tacks/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /Users/tacks/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /Users/tacks/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /Users/tacks/.profile
  /Users/tacks/.bash_profile
  /Users/tacks/.zshenv

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: aarch64-apple-darwin
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation

// 输入1，默认安装
>1

info: profile set to 'default'
info: default host triple is aarch64-apple-darwin
info: syncing channel updates for 'stable-aarch64-apple-darwin'
### TACKS
730.9 KiB / 730.9 KiB (100 %) 688.0 KiB/s in  1s ETA:  0s
info: latest update on 2024-05-02, rust version 1.78.0 (9b00956e5 2024-04-29)
info: downloading component 'cargo'
  6.3 MiB /   6.3 MiB (100 %)   1.2 MiB/s in  4s ETA:  0s
info: downloading component 'clippy'
  2.2 MiB /   2.2 MiB (100 %)   2.0 MiB/s in  1s ETA:  0s
info: downloading component 'rust-docs'
 15.1 MiB /  15.1 MiB (100 %)   2.9 MiB/s in  5s ETA:  0s
info: downloading component 'rust-std'
 22.6 MiB /  22.6 MiB (100 %)   3.4 MiB/s in  6s ETA:  0s
info: downloading component 'rustc'
 50.7 MiB /  50.7 MiB (100 %)   4.1 MiB/s in 12s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 15.1 MiB /  15.1 MiB (100 %)   7.2 MiB/s in  2s ETA:  0s
info: installing component 'rust-std'
 22.6 MiB /  22.6 MiB (100 %)  19.3 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 50.7 MiB /  50.7 MiB (100 %)  21.0 MiB/s in  2s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-aarch64-apple-darwin'

  stable-aarch64-apple-darwin installed - rustc 1.78.0 (9b00956e5 2024-04-29)

// 安装成功

Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, you need to source
the corresponding env file under $HOME/.cargo.

This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish

// 设置环境变量
☁  rust  vim ~/.bash_profile
// 最后增加
export PATH="$HOME/.cargo/bin:$PATH"
☁  rust  source ~/.bash_profile

// 查看版本号 rustc x.y.z (abcabcabc yyyy-mm-dd)
☁  rust  rustc --version
rustc 1.78.0 (9b00956e5 2024-04-29)
```

- 其他基础命令

```shell
# 查看版本号
$ rustc --version

# 更新rust
$ rustup update

# 卸载rust
$ rustup self uninstall
```


## 2、Hello World

- Code
  - 主函数 `main`
    - 声明 `fn` 
  - 语句结尾 `;`
  - 宏 `macro`
    - `println!` 打印文本

```rust
// main 没有参数、没有返回值、是每个rust可执行程序最新运行的代码
fn main() {
    // 打印文本
    // println! 属于 Rust macro 宏定义
    // 正常来说函数是没有 感叹号的
    println!("Hello World");
}
```

- Compile
  - `ahead-of-time` 先编译后执行
  - `rustc` 编译
    - `rustc 原文件名`
    - 尽适合简单几个文件的编译

```shell
$ rustc hello-world.rs
$ ls
hello-world    hello-world.rs
```

- Run

```shell
$ ./hello-world
Hello World
```

## 3、Hello Cargo

- Cargo 是 Rust 的构建系统和包管理工具
- Rust 安装时会默认安装 Cargo
  - `cargo --version`

```shell
$ cargo --version
cargo 1.78.0 (54d8815d0 2024-03-26)
```

### 使用 Cargo 创建项目


- `cargo new hello-cargo`
  - 上述命令会创建新的目录 `hello-cargo`
  - `Cargo.toml` 配置文件
  - `src` 目录
    - `main.rs`
  - `.gitignore` 默认初始化一个新的 git 仓库

- /Cargo.toml
  - [package] 区域标题，配置包
    - name 项目名称
    - version 项目版本
    - edition 使用Rust版本
  - [dependencies] 依赖项
  - rust 中，代码包称之为 crate

```toml
[package]
name = "hello-cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

- /src/main.rs
  - 源代码都应该放置到 src 下；
  - 顶层目录可以放 Readme 许可信息、配置信息等；


### 使用 Cargo Build 构建项目


```shell
$ cargo build
  Compiling hello-cargo v0.1.0 (/Users/stellonde/Code/rust/rust-new-knowledge/code/01-hello-world/hello-cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
```

会生成 `Cargo.lock` 文件

```
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "hello-cargo"
version = "0.1.0"
```

会生成 `target` 目录


### 使用 Cargo Run 构建并运行项目

```shell
$ cargo run
   Compiling hello-cargo v0.1.0 (/Users/stellonde/Code/rust/rust-new-knowledge/code/01-hello-world/hello-cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/hello-cargo`
Hello, world!

# 如果多次执行，源代码没有变化，则直接省去 compile 过程
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello-cargo`
Hello, world!
```

### 使用 Cargo Check 快速检查代码

- Cargo Check 检查代码，确保可以编译通过，但是不产生任何可执行文件
  - 用于编写代码的时候快速反复的检查代码，相比 cargo build ，效率高的多

```shell
$ cargo check
    Checking hello-cargo v0.1.0 (/Users/stellonde/Code/rust/rust-new-knowledge/code/01-hello-world/hello-cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
```

### 使用 Cargo Build 发布构建

- 编译的时候会进行优化
  - 代码运行更快，但是编译时间更长
- 编译后的可执行文件位置
  - target/release 下
  - target/debug 是开发过程中生成的位置

```shell
$ cargo build --release
  Compiling hello-cargo v0.1.0 (/Users/stellonde/Code/rust/rust-new-knowledge/code/01-hello-world/hello-cargo)
    Finished `release` profile [optimized] target(s) in 0.10s
```


## 4、总结


| 语句      | 作用 | 
| :---:       |    :----:   |    
| `rustc --version`   | 查看 rust 版本       | 
| `cargo --version`   | 查看 cargo 版本      | 
| `cargo build`   | 构建项目      | 
| `cargo run`     | 执行项目      | 
| `cargo check`   | 检查项目代码  ｜ 
| `cargo build --release`   | 发版  ｜ 