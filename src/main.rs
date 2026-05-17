use std::io;
fn main() {
    // Just a test
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("无法读取输入");
    let a: i32 = match input.trim().parse() {
        Ok(a) => a,
        Err(_) => {
            println!("请输入一个整数");
            return;
        }
    };
    for i in 0..=a {
        print!("{} ", i);
    }
    println!();
}
