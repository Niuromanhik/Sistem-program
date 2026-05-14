#![allow(dead_code, unused)]
// ========== РОЗДІЛ 3: ЗМІННІ ==========

fn variables1() {
    let x: i32 = 5;
    let _y: i32 = 0;
    assert_eq!(x, 5);
    println!("Success!");
}

fn variables2() {
    let mut x = 1;
    x += 2;
    assert_eq!(x, 3);
    println!("Success!");
}

fn variables3() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y);
}

fn variables4() {
    define_x();
}
fn define_x() {
    let x = "hello";
    println!("{}, world", x);
}

fn variables5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);
    let x = 42;
    println!("{}", x);
}

fn variables6() {
    let x: i32 = 0;
    let _x = x;
    let _y = 4;
    let _y = "I can also be bound to text!";
    println!("Success!");
}

// ========== РОЗДІЛ 4: ЧИСЛА ==========

// Цілі числа
fn numbers1() {
    let x: i32 = 5;
    let mut _y: i32 = 5;
    _y = x;
    let _z = 10;
    println!("Success!");
}

fn numbers2() {
    let _v: u16 = 38_u8 as u16;
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn numbers3() {
    let x = 5u32;
    assert_eq!("u32".to_string(), type_of(&x));
    println!("Success!");
}

fn numbers4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("Success!");
}

fn numbers5() {
    let v1 = 251_u8 as u16 + 8;
    let v2 = i8::checked_add(127, 0).unwrap();
    println!("{},{}", v1, v2);
}

fn numbers6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1579);
    println!("Success!");
}

// Числа з плаваючою комою
fn floats1() {
    let x = 1_000_000.1_f64;
    let _y: f32 = 0.12;
    let _z = 0.01_f64;
    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn floats2() {
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 1e-10);
    println!("Success!");
}

// Діапазон
fn ranges1() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }
    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}", c);
    }
}

fn ranges2() {
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    println!("Success!");
}

// Вирахування
fn computations() {
    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(1u8.wrapping_sub(2) == 255);
    assert!(3 * 50 == 150);
    assert!(9.6_f32 / 3.2 == 3.0);
    assert!(24 % 5 == 4);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
// ========== РОЗДІЛ 6: РЯДКИ ==========

fn str1() {
    let _s: &str = "hello, world";
    println!("Success!");
}

fn str2() {
    let s: Box<str> = "hello, world".into();
    greetings(&s);
}
fn greetings(s: &str) {
    println!("{}", s);
}

fn str3() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');
    assert_eq!(s, "hello, world!");
    println!("Success!");
}

fn str4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!".to_string().as_str();
    println!("{}", s);
}

fn str5() {
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");
    assert_eq!(s1, "I like cats");
    println!("Success!");
}

fn str6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s2);
}

fn str7() {
    let s = "hello, world";
    greetings(s.to_string().as_str());
}

fn str8() {
    let s = "hello, world".to_string();
    let _s1: &str = &s;
    println!("Success!");
}

fn str9() {
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                        can be escaped too!";
    println!("{}", long_string);
}

fn str10() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: \\x3F \\u{211D}");
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);
    let long_delimiter = "Hello, \"##\"";
    assert_eq!(long_delimiter, "Hello, \"##\"");
    println!("Success!");
}

fn str12() {
    for c in "你好，世界".chars() {
        println!("{}", c);
    }
}

// ========== РОЗДІЛ 6: МАСИВИ ==========

fn array1() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert!(arr.len() == 5);
    println!("Success!");
}

fn array2() {
    let _arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    assert!(std::mem::size_of_val(&arr) == 12);
    println!("Success!");
}

fn array3() {
    let list: [i32; 100] = [1; 100];
    assert!(list[0] == 1);
    assert!(list.len() == 100);
    println!("Success!");
}

fn array4() {
    let _arr = [1, 2, 3];
    println!("Success!");
}

fn array5() {
    let arr = ['a', 'b', 'c'];
    let ele = arr[0];
    assert!(ele == 'a');
    println!("Success!");
}

fn array6() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    let _name0 = names.get(0).unwrap();
    let _name1 = names.get(1).unwrap();
    println!("Success!");
}

// ========== РОЗДІЛ 6: ЗРІЗИ ==========

fn slice1() {
    let arr = [1, 2, 3];
    let _s1: &[i32] = &arr[0..2];
    let _s2: &str = "hello, world" as &str;
    println!("Success!");
}

fn slice2() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
    println!("Success!");
}

fn slice3() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
    println!("Success!");
}

fn slice4() {
    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    assert_eq!(slice1, slice2);
    println!("Success!");
}

fn slice5() {
    let s = "你好，世界";
    let slice = &s[0..3];
    assert!(slice == "你");
    println!("Success!");
}

// ========== РОЗДІЛ 6: КОРТЕЖІ ==========

fn tuple1() {
    let _t0: (u8, i16) = (0, -1);
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    println!("Success!");
}

fn tuple2() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");
    println!("Success!");
}

fn tuple3() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}

fn tuple4() {
    let tup = (1, 6.4, "hello");
    let (x, z, y) = tup;
    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
    println!("Success!");
}

fn tuple5() {
    let (x, y, z);
    (y, x, z) = (1, 2, 3);
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);
    println!("Success!");
}

fn tuple6() {
    let (x, y) = sum_multiply((2, 3));
    assert_eq!(x, 5);
    assert_eq!(y, 6);
    println!("Success!");
}
fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

// ========== РОЗДІЛ 6: СТРУКТУРИ ==========

struct Person {
    name: String,
    age: u8,
    hobby: String,
}

fn struct1() {
    let age = 30;
    let _p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };
    println!("Success!");
}

struct Unit;
trait SomeTrait {}
impl SomeTrait for Unit {}
fn do_something_with_unit(_u: Unit) {}

fn struct2() {
    let u = Unit;
    do_something_with_unit(u);
    println!("Success!");
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn struct3() {
    let v = Point(0, 127, 255);
    check_color(Color(v.0, v.1, v.2));
    println!("Success!");
}
fn check_color(p: Color) {
    assert_eq!(p.0, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}

struct Person2 {
    name: String,
    age: u8,
}

fn struct4() {
    let age = 18;
    let mut p = Person2 {
        name: String::from("sunface"),
        age,
    };
    p.age = 30;
    let _p_name = String::from("sunfei");
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person2 {
    Person2 {
        age,
        name,
    }
}

fn struct5() {
    println!("Success!");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

fn struct6() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };
    let _u2 = set_email(u1);
    println!("Success!");
}

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn struct8() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };
    let _name = f.name.clone();
    println!("{}, {}, {:?}", f.name, f.data, f);
}

// ========== РОЗДІЛ 6: ENUM ==========

enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}

fn enum1() {
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
    println!("Success!");
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum2() {
    let _msg1 = Message::Move { x: 1, y: 2 };
    let _msg2 = Message::Write(String::from("hello, world!"));
    println!("Success!");
}

fn enum3() {
    let msg = Message::Move { x: 1, y: 2 };
    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN!");
    }
    println!("Success!");
}

fn enum4() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];
    for msg in msgs {
        show_message(msg);
    }
}
fn show_message(msg: Message) {
    println!("{}", match msg {
        Message::Quit => String::from("Quit"),
        Message::Move { x, y } => format!("Move to ({}, {})", x, y),
        Message::Write(s) => s,
        Message::ChangeColor(r, g, b) => format!("Color: {}, {}, {}", r, g, b),
    });
}

fn enum5() {
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);
    if let Some(n) = six {
        println!("{}", n);
        println!("Success!");
    } else {
        panic!("NEVER LET THIS RUN!");
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    // розділ 3
    variables1();
    variables2();
    variables3();
    variables4();
    variables5();
    variables6();
    // розділ 4
    numbers1();
    numbers2();
    numbers3();
    numbers4();
    numbers5();
    numbers6();
    floats1();
    floats2();
    ranges1();
    ranges2();
    computations();
    // розділ 6
    str1();
    str2();
    str3();
    str4();
    str5();
    str6();
    str7();
    str8();
    str9();
    str10();
    str12();
    array1();
    array2();
    array3();
    array4();
    array5();
    array6();
    slice1();
    slice2();
    slice3();
    slice4();
    slice5();
    tuple1();
    tuple2();
    tuple3();
    tuple4();
    tuple5();
    tuple6();
    struct1();
    struct2();
    struct3();
    struct4();
    struct5();
    struct6();
    struct8();
    enum1();
    enum2();
    enum3();
    enum4();
    enum5();
}