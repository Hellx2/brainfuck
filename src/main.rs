use std::io::{stdin, stdout, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut files: Vec<&str> = vec![];

    let mut temp = String::new();

    if args.len() < 2 {
        print!("Input the name of the file(s) to run: ");
        if stdout().flush().is_err() {
            println!();
        }

        if let Err(e) = stdin().read_line(&mut temp) {
            eprintln!("Failed to read line: {e}");
            std::process::exit(1);
        }

        files.append(&mut temp.split_whitespace().collect());
    } else {
        files.append(&mut args.iter().skip(1).map(|x| x.as_str()).collect());
    }

    println!("Brainfuck Interpreter (c) 2024 Hellx2\n");
    println!("Executing the following files: {:#?}", files);
    for file in files {
        println!("\nReading file {}", file);
        println!();

        let text = match std::fs::read_to_string(file) {
            Ok(txt) => txt,
            Err(err) => {
                eprintln!("Failed to read file '{file}': {err}");
                continue;
            }
        };

        let mut arr = [0; 30000];
        let mut ptr = 0;
        let mut inp = String::new();
        exec(text.as_str(), &mut arr, &mut ptr, &mut inp);
        println!();
    }
}

fn exec(text: &str, arr: &mut [i32; 30000], ptr: &mut usize, inp: &mut String) {
    let mut i = 0;
    // TODO: Rewrite using an iterator instead
    while i < text.len() {
        match text.chars().nth(i).unwrap() {
            '[' => {
                let mut j = 0;
                let mut loop_: &str = "";
                i += 1;
                for k in i..text.len() {
                    let t = text.chars().nth(k).unwrap();
                    if t == '[' {
                        j += 1
                    }
                    if t == ']' {
                        if j == 0 {
                            loop_ = &text[i..k];
                            i = k;
                            break;
                        } else {
                            j -= 1
                        }
                    }
                    if k == text.len() - 1 {
                        panic!("Unclosed loop starting at {}", i);
                    }
                }
                while arr[*ptr] != 0 {
                    exec(loop_, arr, ptr, inp);
                }
            }
            ']' => {
                panic!("End of loop without start at {}", i)
            }
            '>' => *ptr += 1,
            '<' => {
                if *ptr > 0 {
                    *ptr -= 1
                }
            }
            '+' => arr[*ptr] += 1,
            '-' => arr[*ptr] -= 1,
            '.' => {
                print!("{}", char::from_u32(arr[*ptr] as u32).unwrap());
                stdout().flush().unwrap();
            }
            // TODO: Implement the , operator
            ',' => {
                if !inp.is_empty() {
                    stdout().flush().unwrap();
                    if stdin().read_line(inp).unwrap_or(0) == 0 {
                        std::process::exit(0)
                    }
                }
                arr[*ptr] = inp.chars().next().unwrap() as i32;
                *inp = inp[1..].to_string();
            }
            _ => (),
        }
        i += 1;
    }
}
