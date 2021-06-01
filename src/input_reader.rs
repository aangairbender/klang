use std::io;

pub struct InputReader {
    line: String,
    cur_index: usize
}

impl InputReader {
    pub fn new() -> Self {
        Self {
            line: String::new(),
            cur_index: 0
        }
    }

    pub fn peek(&self) -> Option<char> {
        panic!("not implemented")
    }
    
    pub fn consume(&mut self) -> Option<char> {
        if cur_index + 1 >= self.line.len() {
            io::stdin().read_line(&mut self.line);
            self.cur_index = 0;
        }

        cur_index += 1;
        self.line.get(cur_index - 1)
    }
}