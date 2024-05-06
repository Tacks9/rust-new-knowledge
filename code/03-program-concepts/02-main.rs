fn main() {
    let x = 3.2;
    let y:f32 = 3.2;
    println!("x={},y={}", x, y);

    // 创建一个元组
    let tup:(i32,f64,u8) = (500,6.4,1);
    // 点标记法，利用下标获取元组的某个元素
    println!("{},{},{}", tup.0,tup.1,tup.2);
    // 解构，获取 tuple 的元素值
    let (a,b,c) = tup;
    println!("{},{},{}", a, b, c);


    // 数组
    let arr:[i32;5] = [1,2,3,4,5];
    println!("arr[0]={}", arr[0]);
    // 声明重复值的数组
    let arr:[i32;5] = [1;5];
    println!("arr[4]={}", arr[4]);


}