# 02-Guessing-game

## 1、快速初始化项目

- Cargo New 快速初始化空项目 `cargo new guess-game`
  - 快速在当前目录新建一个 `guess-game` 项目
- VsCode 配置 `Install 'code' command in PATH` 模式
  - `cd guess-game`
  - 这样可以在命令行 `code .` 触发快速打开新窗口
- 快速运行这个空项目
  - `cargo run`

## 2、猜数

### 2.1 获取用户输入

```rust
// 标准库 std prelude 预导入
use std::io; 

fn main() {
    // 宏，输出字符到屏幕上
    println!("===Guessing-Number-Game===");

    println!(">猜测一个数字");

    // rust 默认所有变量都不可变，如果希望可变，需要加上 mut

    // 定义一个可变的变量 guess
    // String 是标准库 内部使用 utf-8字节，new()是一个关联函数，返回新的空白字符串实例；
    let mut guess = String::new();

    // io 库，stdin()返回一个 Stdin 实例，类似终端中的句柄，用来处理标准输入
    // 使用 std::io::stdin() 或者采用 use 引入
    // read_line() 获得用户的输入，需要一个可变的字符串参数 guess
    // read_line() 需要引用类型，方法参数按照引用传递 &guess，默认引用也是不可变的，所以需要加上 mut
    // read_line() 返回值 io::Result，是一个枚举类型，一个是 Ok 一个是 Err
    // io::Result 一定要被处理使用，可以采用 expect() 方法，假如返回 Err，便会中断这个程序
    io::stdin().read_line(&mut guess).expect("Error 读取失败");

    // {} 占位符，后面的变量
    println!(">你猜测的数字是：{}",guess)
}
```

### 2.2 生成随机数字

- [`crate` - `rand`](https://crates.io/crates/rand)


- cargo 下载 rand

```shell
# 追加 rand 库
$ vim Cargo.toml
[dependencies]
rand="0.8.5"

# 尝试编译，从 crates.io 拉取 rand ,rand 依赖 libc，所以需要把依赖项进行下载
$ cargo build     
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling libc v0.2.154
   Compiling getrandom v0.2.14
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guess-game v0.1.0 (/Users/stellonde/Code/rust/guess-game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.89s
```

- Cargo.lock 

在你第一次 `cargo build` 的时候，会找到符合要求的版本，并写入到 lock 文件中，这样就保证你使用的组件版本和第一次的build一样。如果你希望手动升级某个组件的版本，你可以手动执行 `cargo update` ，先更新 crates.io 的index 注册表，然后会暂时忽略 lock 文件，而重新去寻找符合 Cargo.toml 要求的最新的版本。

```toml
# lock 内大致的内容
[[package]]
name = "rand"
version = "0.8.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "34af8d1a0e25924bc5b7c43c079c942339d8f0a8b57c39049bef581b46327404"
dependencies = [
 "libc",
 "rand_chacha",
 "rand_core",
]
```

- Code

```rust
use rand::Rng; // trait 类似接口
fn main() {
    println!("===Guessing-Number-Game===");
    // 调用 rand 模块中的 thread_rng() 函数，它返回一个随机数生成器（RNG），该 RNG 是特定线程的本地实例
    // 调用 gen_range 方法，该方法用于生成一个指定范围内的随机数
    // 1..101 表示一个范围，从1到100（不包括101）。所以，这行代码生成的随机数将在1到100之间 [1,101)
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!(">随机数字是：{}" ,secret_number)
}
```


### 2.3 两数字比大小

- `std::cmp::Ordering` 大小比较
- `shadow` 变量名复用，类型变化
- `match` 语句，相同类型比较后根据不同的arm 执行不同的逻辑
- Rust 的类型推断猜测

```rust
use std::io;
// 排序是两个值之间的比较结果
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("===Guessing-Number-Game===");

    println!(">[1,100] 请您随机猜测一个数字");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Error Read number");
    println!(">你猜的数字：{}" , guess);

    // string 字符串 trim 去除前后空格换行等
    // parse 解析字符串为某个类型，比如 guess:u32，显示声明类型，无符号 int32类型
    // rust 特点 shadow 类型转换，复用这个 guess 变量名，但是类型从 string  转变成 number u32
    let guess:u32 = guess.trim().parse().expect("Please Type a number !");

    // rust 强化编译器
    // secret_number 类型是编译器推断出来 u32 类型，因为后面 cmp 比较，如果没有比较的话类型推断为 i32
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!(">随机数字是：{}" ,secret_number);

    // 返回 core::cmp::Ordering
    match guess.cmp(&secret_number) {
        // arm 每个模式匹配
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You Win!"),
    }

}
```

### 2.4 允许多次猜测

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("===Guessing-Number-Game===");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut count:u32=0;
    loop {
        println!(">[1,100] Please Guess a number !");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error Read number");
        let guess = guess.trim();
        if guess == "quit" {
            println!("GoodBye!");
            break;
        }
        let guess:u32 = match guess.parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!(">? The number must be between 1 and 100.");
                    continue;
                } else {
                    num
                }
            },
            Err(_) => {
                println!(">? Invalid input. Please enter a number between 1 and 100, or 'quit' to exit.");
                continue;
            },
        };
        println!(">You Type a Number: {}" , guess);
        count = count + 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!(">You Win! You guessed a total of {} times!", count);
                break;
            },
        }
    }
}
```