fn main() {
    println!("Hello, world!");

    let string1 = String::from("abcd");
    let string2 = "xys"; // greeting: &'static str

    let result = largest(string1.as_str(), string2);
    println!("The longest line is {:?}", result);
}

fn largest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
