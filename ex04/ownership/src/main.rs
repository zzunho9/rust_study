fn main() {
    // let s = String::from("Hello");
    // takes_ownership(s);
    // // println!("{}", s);
    //
    // let x: u32 = 20;
    // make_copy(x);
    // println!("x: {}", x);
    //
    // let s1 = String::from("hello");
    // let (s2, length) = get_size(s1);
    //
    // println!("{}'s length: {}", s2, length)

    let mut str = String::from("Hello World!");
    let word = first_world(& str);
    str.clear();

    println!("{}", word);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(x: u32) {
    println!("{}", x);
}

fn get_size(s1: String) -> (String, usize) {
    let length = s1.len();
    (s1, length)
}

fn first_world(str: &str) -> &str {
    let byte = str.as_bytes();

    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &str[.. i];
        }
    }

    &str[..]
}