# 03-Program-Concepts

## 1、变量与可变性

### 1.1 变量

- 声明变量使用 `let` 关键字
- 默认情况下，变量是不可变的 `immutable`
- 声明变量时，变量前面增加 `mut`，可以使变量可变

```rust
// 如果希望 x 可变，则 let mut x = 5;
let x = 5;
println!("The value of x is {}", x);
// 报错 cannot assign twice to immutable variable
x = 6;
println!("The value of x is {}", x);
```

### 1.2 常量 `constant`

- 常量绑定值后也是不可变
- 不可以使用 mut ，常量永远不可变的
- 声明常量使用 const 关键字，它的类型必须要声明
- 常量可以在任意作用域内声明，包括全局作用域
- 常量只可以绑定到常量表达式
- 命名规范，常量全部大写，单词之间下划线分开

```rust
// 常量
const MY_NAME:&str = "Tacks";
const MY_AGE:u32 = 18;
const MY_MONEY:u32 = 100_000;
println!("My name is {} , age is {}, and has {} money!", MY_NAME, MY_AGE, MY_MONEY);
```


### 1.3 隐藏 `shadow`

- 可以使用相同的名字声明新的变量，新的变量就会 shadow 隐藏 之前声明的同名变量
- 后续的代码中变量名代表的就是新的变量
- shadow 和 mut 是不一样的
  - 使用 let 声明同名新变量，类型可以和之前的不同
  - 不使用 let , 重新给非 mut 变量赋值会导致编译错误

```rust
// shadow 新变量和之前的类型可以不一致，这就有点意思
let spaces = "shadow";
println!("The value of shadow is {}", spaces);
let spaces = spaces.len();
println!("The value of shadow is {}", spaces);
```

## 2、数据类型

> Rust 是静态编译语言，在编译的时候必须知道所有变量的类型。基于使用的值，编译器通常能推断出来它的具体类型，如果某个变量可能的类型比较多的时候，就需要明确变量的类型。

### 2.1 标量类型

- 整数类型
  - 有无符号
    - u 表示无符号类型，(u8/u16/u32/u64/u128/...)
    - i 表示有符号类型
  - 整数溢出
    - u8 范围 0～255，如果u8变量=256，那么dev下会panic/release下会环绕
- 浮点类型
  - f32 32位，单精度
  - f64 64位，双精度，默认是f64
- 布尔类型
  - 一个字节大小
  - true
  - false
- 字符类型 char
  - 单个字符，单引号比较
  - 占用 4 个字节 ，unicode 标量值

### 2.2 复合类型


- 元组 tuple
  - 可以存多个类型的多个值
  - 长度固定，一旦声明无法改变

```rust
// 创建一个元组
let tup:(i32,f64,u8) = (500,6.4,1);
// 点标记法，利用下标获取元组的某个元素
println!("{},{},{}", tup.0,tup.1,tup.2);
// 解构，获取 tuple 的元素值
let (a,b,c) = tup;
println!("{},{},{}", a, b, c);
```

- 数组
  - 可以将多个值放在一个类型中，每个值类型必须相同
  - 长度固定，一旦声明无法改变
  - 声明 [类型;长度] `let arr:[i32;5] = [1,2,3,4,5]`
  - 如果值都相等，可以简便声明 `let arr:[i32;5] = [1;5]` 相当于5个1
  - 可以采用索引访问元素的值
  - 如果访问超过数组的范围 `index of bound`
    - 编译通过
    - 运行时报错，panic ，不允许访问超出范围的内存；

```rust
// 数组
let arr:[i32;5] = [1,2,3,4,5];
println!("arr[0]={}", arr[0]);
// 声明重复值的数组
let arr:[i32;5] = [1;5];
println!("arr[4]={}", arr[4]);
```


## 3、函数

- 声明函数使用 fn 关键词
- 命名规范：函数和变量名，都是采用小写，单词之间下划线分开,否则报错 "convert the identifier to snake case"
- 函数的形参：必须声明每个参数的类型
- let 表达式没有返回值
- 函数的返回值
  - 在 -> 符号后面声明函数返回值类型，但是不可以以返回值命名
  - Rust 里面，返回值就是函数体里面最后一个表达式的值
  - 可以使用 return 返回一个值
```rust
fn main() {
    println!("main init");
    another_func();
}
fn another_func() {
    println!("func init");
    another_func1(1);
}
fn another_func1(x:i32) {
    println!("func1 init {}", x);
    println!("func2 return {}", another_func2());
}
fn another_func2() -> i32 {
    println!("func2 init");
    // 最后一个表达式的值 100，加分号就错误了
    100
}
```

## 4、控制流 if/else

- if/else

```rust
fn main() {
    let num = 3;
    if num < 5 {
        println!("num < 5 is bool");
    } else if num == 5 {
        println!("num == 5 is bool");
    } else {
        println!("num > 5 is bool");
    }
}
```

```rust
let condition = true;
let number = if condition {100} else {99};
println!("The value of number is: {}", number);
```

## 5、循环

- loop
  - 反复执行，直到你喊停，break 
  - break 后面可以跟表达式
- while
  - 条件循环
- for 
  - 遍历集合


```rust
fn main() {
    // loop
    let mut age = 18;
    loop {
        age = age + 1;
        if age == 20 {
            break
        }
    }
    println!("age:{}",  age);

    // while
    let mut number = 1;
    while number <= 5 {
        println!("number:{}",  number);
        number = number + 1;
    }

    let arr = [10,20,30,40,50];
    for ele in arr.iter() {
        println!("arr item:{}",  ele);
    }

    // [1,4) 3、2、1 倒着输出
    for ele in (1..4).rev() {
        println!("item:{}",  ele);
    }
}
```
