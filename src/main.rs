use std::io::{Read, Write};
use std::{fs::File, num::ParseIntError};
const MAIN_PROMPT: &str =
    "1: To Text \n2: To Binary\n3: Encode binary by replacind 0 with a character. The last character will be what 0 is replaced with.\n4: Decode binary string. The last character is the decoding character.\n5: Swap 1s and 0s. \nE: exit.";
fn main() {
    run();
}

fn input(prompt: &str) -> Result<String, std::io::Error> {
    use std::io;
    println!("{}", prompt);
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s)
}
fn to_text(mut s: String) {
    s = s.trim().to_string();
    let mut h = Vec::new();
    for i in s.split_ascii_whitespace() {
        let _intval = u8::from_str_radix(i, 2);
        match _intval {
            Err(_) => {
                println!("That file is not formatted correctly!");
                run();
                break;
            }
            Ok(b) => {
                h.push(b);
            }
        }
    }
    let q = String::from_utf8(h).unwrap();
    println!("{}", q);
    write_to_file(&q);
}
fn to_text_in(s: &str) -> Result<String, ParseIntError> {
    let g = s.trim().to_string();
    let mut h = Vec::new();
    for i in g.split_ascii_whitespace() {
        let _intval = u8::from_str_radix(i, 2)?;
        h.push(_intval);
    }
    let q = String::from_utf8(h).unwrap();
    Ok(q)
}
fn to_binary_in(s: &str) -> String {
    let mut h = String::new();
    for i in s.as_bytes().iter().map(|x| format!("{:b}", x)) {
        h.push_str(&i);
        h.push(' ');
    }
    h
}
fn swap_binary_in(mut s: String) -> String {
    s = s.replace("1", "L");
    s = s.replace("0", "1");
    s = s.replace("L", "0");
    s
}

fn to_binary(s: String) {
    let mut h = String::new();
    for i in s.as_bytes().iter().map(|x| format!("{:b}", x)) {
        h.push_str(&i);
        h.push(' ');
    }
    println!("{}", h);
    write_to_file(&h);
}

fn replace_binary(s: String, c: char) {
    to_binary(s);
    let mut s = file_to_string(File::open("input.txt").unwrap());
    s = s.replace("0", &c.to_string());
    println!("{}", s);
    write_to_file(&s);
}

fn parse_insts(s: String, b: &str) {
    match b.trim().chars().next() {
        Some('1') => {
            to_text(s);
            run();
        }
        Some('2') => {
            to_binary(s);
            run();
        }
        Some('3') => {
            replace_binary(s, get_last_char(b).unwrap());
            run();
        }
        Some('4') => {
            decode_binary(s, get_last_char(b).unwrap());
            run();
        }
        Some('5') => {
            swap_binary(s);
            run();
        }
        Some('6') => {
            break_stuff(s);
            run();
        }
        Some('e') | Some('E') => {}
        _ => parse_insts(s, &input("That isn't a valid response!").unwrap()),
    };
}

fn decode_binary(s: String, c: char) {
    let b = s.replace(&c.to_string(), "0");
    to_text(b);
}
fn file_to_string(mut f: File) -> String {
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}
fn swap_binary(mut s: String) {
    s = s.replace("1", "L");
    s = s.replace("0", "1");
    s = s.replace("L", "0");
    write_to_file(&s);
}
fn write_to_file(s: &str) {
    let mut b = File::create("input.txt").unwrap();
    b.write_all(s.as_bytes()).unwrap();
}
fn get_last_char(b: &str) -> Option<char> {
    b.trim().chars().last()
}
fn run() {
    let b: String = match File::open("input.txt") {
        Ok(f) => file_to_string(f),
        Err(_) => {
            print!("Could not read input.txt! Please manually input the text:");
            input("").unwrap()
        }
    };
    parse_insts(b, &input(MAIN_PROMPT).unwrap());
}
fn break_stuff(h: String) {
    let mut a = Ok(h);
    while let Ok(s) = a {
        if s.is_empty() {
            break;
        }
        println!("{}", s);
        let mut b = s.clone();
        b = to_binary_in(&b);
        b = swap_binary_in(b);
        if to_text_in(&b).unwrap() == s.clone() {
            break;
        }
        a = to_text_in(&b);
        write_to_file(&s);
    }
}
