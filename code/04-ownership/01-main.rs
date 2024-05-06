fn main() {
    // 创建可变的 String 类型
    let mut s = String::from("hello");
    s.push_str(",world");
    println!("{}", s);


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


    // 克隆
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("{} {}", s3, s4);


    let strHello = String::from("Hello");
    take_ownership(strHello);
    let x = 10;
    make_copy(x);
    println!("x={}", x);


    let ss1 = gives_ownership();
    println!("ss1={}", ss1);

    let ss2 = String::from("hello");
    println!("ss2={}", ss2);
    
    let ss3 = takes_and_gives_back(ss2);
    println!("ss3={}", ss3);

    // 如何不获得的所有权，还能使用
    let str1 = String::from("hello");
    let (str2, len) = calculate_length(str1);
    println!("the length of '{}' is {}", str2, len);


}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_number: i32) {
    println!("{}", some_number);
}

fn gives_ownership() -> String {
    let st = String::from("world");
    st
}

fn takes_and_gives_back(a_str:String) -> String {
    a_str
}

// 所有权进去，又出来了
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}