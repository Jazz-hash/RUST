use std::mem;
fn main() {
    // ---------- Data Types ----------------------
    // i8, i16, i32, i64, isize -- Signed Integers
    // u8, u16, u32, u64, usize -- Unsigned Integer
    let x: i32 = 5;
    // f32, f64 -- Float
    let f: f32 = 5.0;
    // bool: true/false -- Boolean
    let b: bool = true;
    // char - Character
    let c: char = 'c';
    // String
    let s = "String".to_string(); // normal string before conversion
    let ss = String::from("String"); // slices of strings
    let slice = &ss[0..4];
    println!("{}", slice);

    // concatenation:
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let string = s1 + &s2;
    println!("{}", string);

    // operators
    let a = 1 + 20;
    let s = 30 - 20;
    let m = 5 * 2;
    let d = 4 / 6;
    let r = 49 % 2;

    // tuples
    let tuple = (); // unit tuple - functions that returns nothing, returns empty tuple
    let t: (i32, f64, char) = (5, 6.0, 'j');
    let (x, y, z) = t;
    let (_, _, single) = t;
    // t.0, t.1, t.2 - to get elements
    let t = (1, 'a', false);
    let f = (2, (1, 'a', false));
    println!("{} {} {}", t.0, t.1, t.2);
    // :? for debug flah
    println!("{:#?}", f); // # for extra styling

    // arrays
    let a = [1, 2, 3, 4, 5, 6, 7, 8];
    let a1 = a[0];
    let xs: [i32; 5] = [4, 5, 6, 7, 8];
    println!("{} {} {}", xs[0], xs.len(), mem::size_of_val(&xs));
    let ys = &xs[2..4];
    println!("{:?}", ys);
}
