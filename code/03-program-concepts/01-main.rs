fn main() {
    // 没有声明 mut 可变变量
    let x = 5;
    println!("The value of x is {}", x);
    // x = 6;
    // println!("The value of x is {}", x);

    // 常量
    const MY_NAME:&str = "Tacks";
    const MY_AGE:u32 = 18;
    const MY_MONEY:u32 = 100_000;
    println!("My name is {} , age is {}, and has {} money!", MY_NAME, MY_AGE, MY_MONEY);

    // shadow
    let spaces = "shadow";
    println!("The value of shadow is {}", spaces);
    let spaces = spaces.len();
    println!("The value of shadow is {}", spaces);


}