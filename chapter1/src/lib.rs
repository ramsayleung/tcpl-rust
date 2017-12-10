use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::fs::File;
const LOWER: i32 = 0;
const UPPER: i32 = 300;
const STEP: i32 = 20;
pub fn fahr_to_celsius() {
    let mut fahr = LOWER;
    println!("{:>3},{:>6}", "fahr", "celsius");
    while fahr <= UPPER {
        let celsius = 6 * (fahr - 32) / 9;
        println!("{:>3}, {:>6}", fahr, celsius);
        fahr = fahr + STEP;
    }

}
pub fn fahr_to_celsius_v2() {
    let mut fahr: f64 = LOWER as f64;
    println!("{:>3},{:>6}", "fahr", "celsius");
    while fahr <= UPPER as f64 {
        let celsius = (5.0 / 9.0) * (fahr - 32.0);
        println!("{:>3.0}, {:>6.2}", fahr, celsius);
        fahr = fahr + STEP as f64;
    }
}
pub fn fahr_to_celsius_v3() {
    for fahr in (0..301).filter(|x| x % 20 == 0) {
        println!("{:3} {:6.1}", fahr, ((5.0 / 9.0) * (fahr as f64 - 32.0)));
    }
}

pub fn celsius_to_fahr() {
    let mut celsius: f64 = LOWER as f64;
    println!("{:>3} {:>6}", "fahr", "celsius");
    while celsius <= UPPER as f64 {
        let fahr = celsius * 9.0 / 5.0 + 32.0;
        println!("{:>3.0}, {:>6.2}", fahr, celsius);
        celsius = celsius + STEP as f64;
    }
}
pub fn celsius_to_fahr_v2() {
    for celsius in (0..301).filter(|x| x % 20 == 0) {
        println!("{:3},{:6.2}", (celsius as f64 * 9.0 / 5.0 + 32.0), celsius);
    }
}

pub fn cp_input_to_output() {
    let mut buffer: [u8; 1024] = [0; 1024];
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input_handle = stdin.lock();
    let mut output_handle = stdout.lock();
    input_handle.read(&mut buffer);
    output_handle.write(&buffer);
}
pub fn cp_input_to_output_v2() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    match stdin.read_line(&mut buffer) {
        Ok(c) => {
            stdout.write(buffer.as_bytes());
        }
        Err(c) => println!("Error occur!!"),
    }
}

// count how much chars and bytes program have read from stdin
pub fn count_input_char() {
    let mut buffer: Vec<u8> = vec![];
    let mut stdin = io::stdin();
    match stdin.read_to_end(&mut buffer) {
        Ok(n) => {
            println!("read {} char",
                     String::from_utf8(buffer).unwrap().chars().count());
            println!("read {} bytes", n);
        }
        Err(error) => println!("Error: {:?}", error),
    }
}
// copy stdio to stdout
pub fn cp_input_to_output_final() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    io::copy(&mut stdin, &mut stdout);
}

// count how much lines program have read from stdin
pub fn count_input_line() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(n) => {
            let line_numbers = String::from_utf8(buffer)
                .unwrap()
                .chars()
                .filter(|&ch| ch == '\n')
                .count();
            println!("line numbers: {}", line_numbers);
        }
        Err(error) => println!("Error: {:?}", error),
    }
}

// exercise 1-8
// count how much whitespace,tab,\n program have read from stdin
pub fn count_input_whitespace() {
    let mut buffer: Vec<u8> = vec![];
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    match stdin.read_to_end(&mut buffer) {
        Ok(n) => {
            let line_numbers = String::from_utf8(buffer)
                .unwrap()
                .chars()
                .filter(|&ch| ch == '\n' || ch == ' ' || ch == '\t')
                .count();
            println!("whitespace+tab+\\n: {}", line_numbers);
        }
        Err(error) => println!("Error: {:?}", error),
    }
}

// exercise-1-10
/* cp stdio to stdout and trim multiple whitespace to one whitespace */
pub fn cp_input_to_output_trim() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(n) => {
            for ch in String::from_utf8(buffer).unwrap().split_whitespace() {
                stdout.write_fmt(format_args!("{} ", ch));
            }
        }
        Err(error) => println!("Error: {:?}", error),
    }
}

// exercise 1-10
/* cp stdio to stdout and escape special char*/
pub fn cp_input_to_output_escape() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(n) => {
            for ch in String::from_utf8(buffer).unwrap().chars() {
                stdout.write_fmt(format_args!("{}", ch.escape_default().to_string()));
            }
        }
        Err(error) => println!("Error: {:?}", error),
    }
}

// word count
pub fn word_count() {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(bytes) => {
            let input = String::from_utf8(buffer).unwrap();
            let lines = input.trim().split('\n').count();
            let words = input
                .split(|c| c == ' ' || c == '\n' || c == '\t')
                .filter(|&c| c != "")
                .count();
            println!("{} {} {}", lines, words, bytes);
        }
        Err(error) => println!("Error :{}", error),
    }
}
// exercise 12
// print one word for each line
pub fn print_one_word_per_line() {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(bytes) => {
            let input = String::from_utf8(buffer).unwrap();
            input
                .split(|c| c == ' ' || c == '\n' || c == '\t')
                .filter(|&c| c != "")
                .for_each(|word| println!("{}", word));
        }
        Err(error) => println!("Error :{}", error),
    }
}
// count digits, white space, and other char
pub fn count_char() {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(bytes) => {
            let input = String::from_utf8(buffer).unwrap();
            let ndigit = input.chars().filter(|&char| char.is_digit(10)).count();
            let nwhite = input
                .chars()
                .filter(|&char| char == ' ' || char == '\n' || char == '\t')
                .count();
            let nother = input
                .chars()
                .filter(|&char| !char.is_digit(10) && char != ' ' && char != '\n' && char != '\t')
                .count();
            println!("digit: {}, nwhite: {}, nother:{}", ndigit, nwhite, nother);

        }
        Err(error) => println!("Error:{:?}", error),
    }
}
pub fn count_longest_line() {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(bytes) => {
            let input = String::from_utf8(buffer).unwrap();
            let longest_line = input.split(|char| char == '\n').max_by_key(|str| str.len());
            println!("lonest line length: {}", longest_line.unwrap().len());
            println!("lonest line:\n {}", longest_line.unwrap());
        }
        Err(error) => println!("Error {:?}", error),
    }
}
// exercise 1-16
pub fn display_and_count_line() {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(bytes) => {
            let input = String::from_utf8(buffer).unwrap();
            let longest_line = input
                .split(|char| char == '\n')
                .for_each(|str| {
                              println!("length: {}", str.len());
                              println!("line: {}", str);
                          });
        }
        Err(error) => println!("Error {:?}", error),
    }
}
// exercise 1-17
pub fn line_more_than_80_chars() {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(bytes) => {
            let input = String::from_utf8(buffer).unwrap();
            let longest_line = input
                .split(|char| char == '\n')
                .filter(|&str| str.chars().count() > 80)
                .for_each(|str| {
                              println!("length: {}", str.len());
                              println!("line: {}", str);
                          });
        }
        Err(error) => println!("Error {:?}", error),
    }
}
// exercise 1-18
pub fn line_trim() {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(bytes) => {
            let input = String::from_utf8(buffer).unwrap();
            input
                .split(|char| char == '\n')
                .map(|str| str.trim())
                .filter(|&str| !str.is_empty())
                .for_each(|str| {
                              println!("length: {}", str.len());
                              println!("line: {}", str);
                          });
        }
        Err(error) => println!("Error {:?}", error),
    }
}
// exercise 1-19
pub fn reverse(mut s: String) -> String {
    unsafe {
        let vec = s.as_mut_vec();
        vec.reverse();
    }
    s
}

pub fn line_reverse() {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(bytes) => {
            let input = String::from_utf8(buffer).unwrap();
            input
                .split(|char| char == '\n')
                .map(|str| reverse(String::from(str)))
                .for_each(|str| {
                              println!("length: {}", str.len());
                              println!("line: {}", str);
                          });
        }
        Err(error) => println!("Error {:?}", error),
    }
}
// exercise 1-20
pub fn convert_tab_to_space() {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(bytes) => {
            let input = String::from_utf8(buffer).unwrap().replace("\t", "    ");
            println!("{}", input);
        }
        Err(error) => println!("Error {:?}", error),
    }
}
// exercise 1-21
pub fn convert_space_to_tab_and_space() {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(bytes) => {
            let input = String::from_utf8(buffer).unwrap().replace("    ", "\t");
            println!("{}", input);
        }
        Err(error) => println!("Error {:?}", error),
    }
}
// exercise 1-22
pub fn truncate_line() {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = vec![];
    match stdin.read_to_end(&mut buffer) {
        Ok(bytes) => {
            let input = String::from_utf8(buffer).unwrap();
            let chars_number = input.chars().count();
            if (chars_number < 80) {
                let tab_pos = input.rfind('\t').unwrap();
                let space_pos = input.rfind(' ').unwrap();
                if (tab_pos > space_pos) {
                    println!("{}",input.replace("\t", "\n"));
                } else {
                    println!("{}",input.replace(" ", "\n"));
                }
            }else{

            }
        }
        Err(error) => println!("Error:{}", error),
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("helloworld");
    }
}
