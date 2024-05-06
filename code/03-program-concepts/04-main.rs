fn main() {
    let num = 3;
    if num < 5 {
        println!("num < 5 is bool");
    } else if num == 5 {
        println!("num == 5 is bool");
    } else {
        println!("num > 5 is bool");
    }

    let condition = true;
    let number = if condition {100} else {99};
    println!("The value of number is: {}", number);
}