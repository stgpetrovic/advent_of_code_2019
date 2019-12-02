pub struct IntMachine {
  pub state: Vec<usize>,
}

impl IntMachine {
  pub fn init(&mut self, a: usize, b: usize) {
      self.state[1]  = a;
      self.state[2]  = b;
  }

  pub fn run(&mut self) -> usize {
    let mut i = 0;
    while i < self.state.len() {
        match self.state[i] {
            1 => {let idx = self.state[i+3]; self.state[idx] = self.state[self.state[i+2]] + self.state[self.state[i+1]]; i+=4; },
            2 => {let idx = self.state[i+3]; self.state[idx] = self.state[self.state[i+2]] * self.state[self.state[i+1]]; i+=4; },
            99 => {break;},
            _ => println!("error opcode {}", i),
        }
    }
    return self.state[0]
  }
}
