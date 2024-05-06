# 04-Ownership

## 所有权

> 所有权是 Rust 最独特的特性，它让 Rust 无需 GC 就可以保证内存安全；

### 如何管理运行时的内存

基本上所有程序运行时都需要管理使用计算机内存的方式，大概有三种。

- 垃圾回收机制
- 显式分配和释放内存
- 所有权系统管理

Rust 采用所有权来管理系统，包含一组编译器在编译时检查的规则，所有权的特性不会减慢程序的运行速度。


### 栈内存 vs 堆内存

- Stack 
  - 存储在栈上的数据必须有已知固定的大小
  - 指针大小固定，可以把指针放在 stack 上
  - 存储数据：每次都是压入 stack 顶端，不需要寻找新的存储空间
  - 访问数据：局部性原理，会稍微快一些
- Heap
  - 编译时大小未知的数据、或者运行时可能发生变化的数据必须放在堆上
  - 内存分配，当把数据放入 Heap 中，会向操作系统找到一块足够大的空间，标记在用，返回地址，也就是指针
  - 存储数据：需要寻找足够大的空间去存放
  - 访问数据：相对 stack ，多一步指针寻找，会稍微慢一些

- 所有权存在的原因
  - 跟踪代码哪些部分正在使用 heap 的哪些数据
  - 最小化 heap 上重复数据量
  - 清理 heap 上未使用的数据以避免空间不足
  - 管理 heap 数据是所有权存在的原因

### 所有权规则

- 每个值都有一个变量，这个变量是该值的所有者；
- 每个值同时只能有一个所有者；
- 当所有者超出作用域 scope 时，该值将会被删除；

### 变量的作用域 

- scope 就是程序中变量的有效范围

#### String 类型


- 字符串字面值：字面值不可变
- 字符串String：在 heap 上分配，能够存储编译时未知数量的文本；
- 字符串String：比基础的标量数据类型更复杂；


- 创建 String 类型的值：使用 from 函数

```rust
fn main() {
    // 创建可变的 String 类型
    let mut s = String::from("hello");
    s.push_str(",world");
    println!("{}", s);
}
// s 离开作用域，rust 调用 drop 函数进行释放
```

- 为什么字符串字面值不可以修改，而 String 类型的值可以被修改？
  - 内存分配的方式不同
    - 字符串字面值，编译的时候就知道内容了，其文本内容是直接硬编码到最终可执行文件中；    
    - String 类型，为了支持可变性，需要在 heap 上分配内存来保存编译时候的未知文本内容；
  - String 操作系统需要在运行时请求内存
    - 主要是通过 String::from 来申请
  - String 内存的释放
    - GC类的语言，会跟踪并清理不再使用的内存；
    - 没有GC的语言，需要识别内存什么时候不再使用，并调用代码进行返回，不能提前释放，也不能多次释放，也不能忘记释放；
    - Rust 针对某个值来说，当拥有它的变量走出了作用范围时，自动调用 drop 函数，内存就会自动交还给操作系统，进行释放；

#### 变量和数据的交互方式，移动 move

- 多个变量可以与同一个数据使用一种独特的方式进行交互
    - 整数是固定大小的简单值，两个 5 都被压入 stack 中
```rust
let x = 5;
let y = x;
```

- String 类型
    - 组成部分，3块
      - 一个指向存放字符串内容的内存的指针
      - 一个长度 len，字节数
      - 一个容量 capacity，从操作系统中总共获取的内存数量
    - 这些东西放到 stack 中
    - 真正放字符串内容的部分在 heap 上

```rust
let s1 = String::from("hello");
// stack 复制了一份 指针、长度、容量
// heap 真正的字符串内容没有进行复制
let s2 = s1;

// 当变量离开作用域的时候，rust 调用 drop 函数，将变量使用的 heap 内存进行释放；
// 这样就导致 二次释放 double free bug


// 为此，！！！ rust 不一样的地方
// rust 让 s1 失效，当s1离开作用域的时候，不需要释放任何东西
// 此时你是没有办法使用 s1 的
// 报错 borrow of moved value: `s1`
// println!("{}", s1);
```

- 移动 Move
  - 浅拷贝 shallow copy
  - 深拷贝 deep copy
    -  Rust 不会自动创建数据的深拷贝

Rust 上面的操作，可能有点类似浅拷贝，但是又不完全一样，重点是你没办法使用 s1 ,这个操作可以理解为 移动 Move。


#### 变量和数据的交互方式，克隆 clone

- 类似，深拷贝
  
```rust
// 克隆
let s3 = String::from("world");
let s4 = s3.clone();
println!("{} {}", s3, s4);
```

#### Stack 上的数据，复制 Copy

```rust
let x = 5;
let y = x;
println!("{} {}", x, y);
```

- Copy trait 
  - 类似像整数这样完全存放在 stack 上的类型，如果一个类型实现了 Copy 这个 trait，那么旧的变量赋值后仍然可以用；
  - 如果一个类型或者该类型的一部分实现 Drop trait ，那么 Rust 就不会允许它再实现 Copy trait 了；
- 比如一些类型
  - 任何简单标量的组合类型都是可以 Copy
  - 任何需要分配内存资源都不是 Copy
- 拥有 Copy trait
  - 整数类型
  - 浮点类型
  - bool
  - char
  - tuple 元组，如果所有类型都是 copy

### 所有权与函数

```rust
fn main() {
    let strHello = String::from("Hello");
    take_ownership(strHello);
    let x = 10;
    make_copy(x);
    println!("x={}", x);
}

// 移动
fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

// 传递副本
fn make_copy(some_number: i32) {
    println!("{}", some_number);
}
```

- 返回值与作用域
  - 一个变量的所有权通常的变化
    - 把一个值赋给其他变量的时候发生移动
    - 当一个包含heap数据变量离开作用域的时候，就会被 drop 函数清理，除非数据的所有权移动到另一个变量上；

```rust
fn main() {


    let ss1 = gives_ownership();
    println!("ss1={}", ss1);

    let ss2 = String::from("hello");
    println!("ss2={}", ss2);
    
    let ss3 = takes_and_gives_back(ss2);
    println!("ss3={}", ss3);

}

fn gives_ownership() -> String {
    let st = String::from("world");
    st
}

fn takes_and_gives_back(a_str:String) -> String {
    a_str
}
```

- 如何让函数使用某个值，但是获得所有权？


```rust
fn main() {
    // 如何不获得的所有权，还能使用
    let str1 = String::from("hello");
    let (str2, len) = calculate_length(str1);
    println!("the length of '{}' is {}", str2, len);
}
// 所有权进去，又出来了
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```