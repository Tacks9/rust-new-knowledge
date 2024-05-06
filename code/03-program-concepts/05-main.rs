fn main() {
    let mut age = 18;
    loop {
        age = age + 1;
        if age == 20 {
            break
        }
    }
    println!("age:{}",  age);

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