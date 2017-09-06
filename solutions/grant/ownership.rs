fn main() {
    let value = "hello".to_string();
    //println!("Val: {}", value);
    foo(&value);
    foo(&value);
}

fn foo(val: &String) {
    println!("Val: {}", val);
}