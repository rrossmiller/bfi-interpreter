use std::io;
use std::process::Command;

#[derive(Debug)]
pub struct Runner {
    i: usize,
    loop_i: usize,
    cursor: usize,
    state: [u8; 12],
    src: String,
    out: String,
    step: bool,
}

impl Runner {
    pub fn init(src: String) -> Runner {
        Runner {
            i: 0,
            loop_i: 0,
            cursor: 0,
            state: [0; 12],
            src,
            out: String::new(),
            step: false,
        }
    }
    pub fn run(self: &mut Runner) {
        if self.step {
            Command::new("clear").status().unwrap();
        }
        let chars = self.src.chars().collect::<Vec<_>>();
        while self.i < chars.len() {
            let c = chars[self.i];
            match c {
                '+' => self.state[self.cursor] += 1,
                '-' => self.state[self.cursor] -= 1,
                '>' => self.cursor += 1,
                '<' => self.cursor -= 1,
                '.' => {
                    if !self.step {
                        print!("{}", self.state[self.cursor] as char);
                    } else {
                        self.out.push(self.state[self.cursor] as char);
                    }
                }
                ',' => {
                    //user input
                }
                '[' => self.loop_i = self.i,
                ']' => {
                    if self.state[self.cursor] > 0 {
                        self.i = self.loop_i
                    }
                }
                _ => {}
            }
            self.i += 1;
            if self.step {
                println!("{}", c);
                println!("{:?}", self.state);
                println!("=> {}", self.out);
                let _ = io::stdin().read_line(&mut String::new());
                Command::new("clear").status().unwrap();
            }
        }
    }
}
