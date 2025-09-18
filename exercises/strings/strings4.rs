// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    //"blue" 是字符串字面量，类型为 &'static str（静态字符串切片）。
    string_slice("blue");
    //to_string() 将 &str 转换为 String（拥有所有权）。
    string("red".to_string());
    //String::from() 直接创建 String 类型。
    string(String::from("hi"));
    //to_owned() 为 &str 创建所有权，返回 String。
    string("rust is fun!".to_owned());
    //into和from互为反义
    string_slice("nice weather".into());
    //format返回的宏是Sting
    string(format!("Interpolation {}", "Station"));
    //String::from("abc")[0..1] 是 String 的切片（类型 &str），前面的 & 不改变类型（仍为 &str）。
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
