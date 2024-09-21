#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
    let data_str = "Hello";         // `&str`, a string slice
    let data_string = data_str.to_string();  // `String`, an owned string

    println!("{}", data_str);       // Can still use `data_str`
    println!("{}", data_string);    // Can use the owned `String` as well
}
