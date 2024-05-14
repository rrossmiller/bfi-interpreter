use std::process::Command;
use std::{io, usize};

#[derive(Debug)]
pub struct Runner {
    i: usize,
    loop_i: Vec<usize>,
    cursor: usize,
    state: [u8; 256],
    src: String,
    out: String,
    step: bool,
}

const LANG: &str = "+-<>.,[]";
impl Runner {
    pub fn init(src: String) -> Runner {
        Runner {
            i: 0,
            loop_i: Vec::new(),
            cursor: 0,
            state: [0; 256],
            src,
            out: String::new(),
            step: false,
        }
    }
    pub fn run(self: &mut Runner) {
        if self.step {
            Command::new("clear").status().unwrap();
        }
        let chars = self
            .src
            .chars()
            .filter(|e| LANG.contains(*e))
            .collect::<Vec<_>>();
        // chars.iter().enumerate().for_each(|(i, e)| {
        //     if i < 50 {
        //         println!("{i}: {e}");
        //     }
        // });
        // println!(".............\n{:?}", chars);
        while self.i < chars.len() {
            let c = chars[self.i];
            if self.step {
                println!("{} | {}: {}", self.cursor, c, self.i);
                println!("{:?}", self.state);
                // println!("=> {}", self.out);
                // let _ = io::stdin().read_line(&mut String::new());
                // Command::new("clear").status().unwrap();
            }
            match c {
                '+' => {
                    if self.state[self.cursor] == 255 {
                        self.state[self.cursor] = 0;
                    } else {
                        self.state[self.cursor] += 1
                    };
                }
                '-' => {
                    if self.state[self.cursor] == 0 {
                        self.state[self.cursor] = 255;
                    } else {
                        self.state[self.cursor] -= 1
                    };
                }
                '>' => self.cursor += 1,
                '<' => self.cursor -= 1,
                '.' => {
                    self.out.push(self.state[self.cursor] as char);
                    if !self.step {
                        print!("{}", self.state[self.cursor] as char);
                    }
                }
                ',' => {
                    //user input
                }
                '[' => self.loop_i.push(self.i),
                ']' => {
                    if self.state[self.cursor] > 0 {
                        self.i = *self.loop_i.last().unwrap();
                    } else {
                        self.loop_i.pop();
                    }
                }
                _ => {}
            }
            self.i += 1;
        }
    }
}
