#![allow(dead_code)]
use std::collections::HashMap;
use std::fmt;

mod tests;

const VALID_CHARS: [char; 8] = ['+', '-', '>', '<', '.', ',', '[', ']'];

struct Brainfuck {
    cells: [u8; 30000],
    cell_pointer: usize,
    code_pointer: usize,
}

impl fmt::Display for Brainfuck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let to_print: String = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                if i == self.cell_pointer {
                    String::from("hmm")
                } else {
                    format!("{}", cell)
                }
            })
            .collect();
        write!(f, "{}", to_print)
    }
}

impl Brainfuck {
    fn new() -> Brainfuck {
        Brainfuck {
            cells: [0; 30000],
            cell_pointer: 0,
            code_pointer: 0,
        }
    }

    fn run_stream(&mut self, code: String) {
        let code: String = code
            .chars()
            .filter(|token| VALID_CHARS.contains(&token))
            .collect();
        let mut bracemap = HashMap::new();
        {
            let mut bracestack = Vec::new();
            for (i, char) in code.chars().enumerate() {
                match char {
                    '[' => bracestack.push(i),
                    ']' => {
                        let start = bracestack
                            .pop()
                            .expect(format!("Mismatched bracket at {}", i).as_str());
                        bracemap.insert(start, i);
                        bracemap.insert(i, start);
                    }
                    _ => (),
                };
            }
            if bracestack.len() > 0 {
                panic!("Missing closing bracket for {}", bracestack[0]);
            }
        }
        while self.code_pointer < code.len() {
            let token = &code[self.code_pointer..self.code_pointer + 1];
            match token {
                "+" => {
                    let cell = &mut self.cells[self.cell_pointer];
                    *cell = cell.wrapping_add(1);
                }
                "-" => {
                    let cell = &mut self.cells[self.cell_pointer];
                    *cell = cell.wrapping_sub(1);
                }
                ">" => {
                    let pointer = self.cell_pointer + 1;
                    self.cell_pointer = match pointer {
                        30_000 => 0,
                        x => x,
                    };
                }
                "<" => {
                    self.cell_pointer = match self.cell_pointer {
                        0 => 30_000,
                        x => x,
                    } - 1;
                }
                "." => {
                    print!("{}", self.cells[self.cell_pointer] as char);
                }
                "[" if self.cells[self.cell_pointer] == 0 => {
                    self.code_pointer = bracemap[&self.code_pointer];
                }
                "]" if self.cells[self.cell_pointer] != 0 => {
                    self.code_pointer = bracemap[&self.code_pointer];
                }
                _ => (),
            }
            self.code_pointer += 1;
        }
    }
}

fn main() {
    let code = String::from("+.[+.]");
    let mut b = Brainfuck::new();
    // dbg!(b.cells);
    b.run_stream(code);
    dbg!(&b.cells[..5]);
}
