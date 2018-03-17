use std::env;
use std::io;
use std::io::Write;

pub fn get_register_count_parmeter(default: usize) -> usize {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    default
  } else {
    args[1].parse().unwrap()
  }
}

pub fn read_instruction() -> String {
  print!("INS: ");
  io::stdout().flush().expect("failed to flush stdout");

  let mut buf = String::new();
  io::stdin()
    .read_line(&mut buf)
    .expect("failed to read stdin");

  buf.trim().to_string()
}

#[derive(Clone, PartialEq, Debug)]
pub enum ProcessorState {
  Ok,
  Err(String),
  Halt,
}

pub struct Processor {
  register_pointer: usize,
  flag_equal: bool,
  registers: Vec<u8>,
  state: ProcessorState,
}

impl Processor {
  pub fn new(register_count: usize) -> Processor {
    Processor {
      register_pointer: 0,
      flag_equal: true,
      registers: vec![0; register_count],
      state: ProcessorState::Ok,
    }
  }

  pub fn print_register_values(&self, tag: &str) {
    print!("{}: [", tag.to_uppercase());

    //There's probably a better way of accessing this, rather than cloning.
    for (i, register) in self.registers.clone().into_iter().enumerate() {
      print!(
        "{}{}",
        register,
        if i < self.registers.len() - 1 {
          ","
        } else {
          ""
        }
      )
    }

    println!("]");
  }

  pub fn register_values_to_string(&self) -> String {
    let mut buf = String::new();

    for (i, register) in self.registers.clone().into_iter().enumerate() {
      buf.push_str(
        format!(
          "{}{}",
          register,
          if i < self.registers.len() - 1 {
            ","
          } else {
            ""
          }
        ).as_str(),
      );
    }

    format!("[{}]", buf)
  }

  pub fn get_state(&self) -> ProcessorState {
    self.state.clone()
  }

  pub fn execute_symbol(&mut self, symbol: char) {
    match symbol {
      //No op
      '-' | ' ' => {}
      //Next set of instructions
      '>' | '[' | ']' => self.register_pointer = 0,
      //Next register
      ',' => self.register_pointer += 1,
      //Set current register to 0
      '0' => self.registers[self.register_pointer] = 0,
      //Set current register to 1
      '1' => self.registers[self.register_pointer] = 1,
      //Invert current register
      '+' => {
        self.registers[self.register_pointer] = if self.registers[self.register_pointer] == 0 {
          1
        } else {
          0
        };
      }
      //Set comparison flag to (current register == 1 && comparison flag == true)
      '&' => {
        self.flag_equal = self.flag_equal && self.registers[self.register_pointer] == 1;
      }
      //Push value of comparison flag into current register
      '=' => {
        self.registers[self.register_pointer] = if self.flag_equal { 1 } else { 0 };
        self.flag_equal = true;
      }
      //If current register == 1 then halt program
      '!' => if self.registers[self.register_pointer] == 1 {
        self.state = ProcessorState::Halt;
      } else {
        self.state = ProcessorState::Ok;
      },
      //Handle unknown symbols
      _ => {
        self.state = ProcessorState::Err(format!("invalid symbol \"{}\"", symbol));
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_processor_with_4_registers() {
    let p = Processor::new(4);
    assert_eq!(p.register_values_to_string(), "[0,0,0,0]");
  }

  #[test]
  fn create_processor_with_5_registers() {
    let p = Processor::new(5);
    assert_eq!(p.register_values_to_string(), "[0,0,0,0,0]");
  }

  #[test]
  fn create_processor_with_6_registers() {
    let p = Processor::new(6);
    assert_eq!(p.register_values_to_string(), "[0,0,0,0,0,0]");
  }

  #[test]
  fn invert_all() {
    let mut p = Processor::new(4);

    for symbol in "[+,+,+,+]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[1,1,1,1]");
  }

  #[test]
  fn invert_first() {
    let mut p = Processor::new(4);

    for symbol in "[+,-,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[1,0,0,0]");
  }

  #[test]
  fn invert_second() {
    let mut p = Processor::new(4);

    for symbol in "[-,+,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[0,1,0,0]");
  }

  #[test]
  fn invert_third() {
    let mut p = Processor::new(4);

    for symbol in "[-,-,+,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[0,0,1,0]");
  }

  #[test]
  fn invert_fourth() {
    let mut p = Processor::new(4);

    for symbol in "[-,-,-,+]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[0,0,0,1]");
  }

  #[test]
  fn invert_first_pair() {
    let mut p = Processor::new(4);

    for symbol in "[+,-,+,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[1,0,1,0]");
  }

  #[test]
  fn invert_second_pair() {
    let mut p = Processor::new(4);

    for symbol in "[-,+,-,+]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[0,1,0,1]");
  }

  #[test]
  fn invert_revert_first_pair() {
    let mut p = Processor::new(4);

    for symbol in "[1,1,1,1] > [+,-,+,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[0,1,0,1]");
  }

  #[test]
  fn invert_revert_second_pair() {
    let mut p = Processor::new(4);

    for symbol in "[1,1,1,1] > [-,+,-,+]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[1,0,1,0]");
  }

  #[test]
  fn comparison_first_set() {
    let mut p = Processor::new(4);

    for symbol in "[1,0,1,0] > [&,-,&,-] > [-,-,-,=]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[1,0,1,1]");
  }

  #[test]
  fn comparison_second_set() {
    let mut p = Processor::new(4);

    for symbol in "[0,1,0,1] > [-,&,-,&] > [=,-,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[1,1,0,1]");
  }

  #[test]
  fn set_first_literal() {
    let mut p = Processor::new(4);

    for symbol in "[1,-,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[1,0,0,0]");
  }

  #[test]
  fn set_second_literal() {
    let mut p = Processor::new(4);

    for symbol in "[-,1,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[0,1,0,0]");
  }

  #[test]
  fn set_third_literal() {
    let mut p = Processor::new(4);

    for symbol in "[-,-,1,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[0,0,1,0]");
  }

  #[test]
  fn set_fourth_literal() {
    let mut p = Processor::new(4);

    for symbol in "[-,-,-,1]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[0,0,0,1]");
  }

  #[test]
  fn set_first_literal_inverted() {
    let mut p = Processor::new(4);

    for symbol in "[1,1,1,1] > [0,-,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[0,1,1,1]");
  }

  #[test]
  fn set_second_literal_inverted() {
    let mut p = Processor::new(4);

    for symbol in "[1,1,1,1] > [-,0,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[1,0,1,1]");
  }

  #[test]
  fn set_third_literal_inverted() {
    let mut p = Processor::new(4);

    for symbol in "[1,1,1,1] > [-,-,0,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[1,1,0,1]");
  }

  #[test]
  fn set_fourth_literal_inverted() {
    let mut p = Processor::new(4);

    for symbol in "[1,1,1,1] > [-,-,-,0]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[1,1,1,0]");
  }

  #[test]
  fn set_literal_all() {
    let mut p = Processor::new(4);

    for symbol in "[1,1,1,1]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[1,1,1,1]");
  }

  #[test]
  fn set_literal_all_inverted() {
    let mut p = Processor::new(4);

    for symbol in "[1,1,1,1] > [0,0,0,0]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[0,0,0,0]");
  }

  #[test]
  fn no_op_all() {
    let mut p = Processor::new(4);

    for symbol in "[-,-,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[0,0,0,0]");
  }

  #[test]
  fn no_op_all_inverted() {
    let mut p = Processor::new(4);

    for symbol in "[1,1,1,1] > [-,-,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.register_values_to_string(), "[1,1,1,1]");
  }

  #[test]
  fn halt_on_first() {
    let mut p = Processor::new(4);

    for symbol in "[1,0,0,0] > [!,-,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.get_state(), ProcessorState::Halt);
  }

  #[test]
  fn halt_on_second() {
    let mut p = Processor::new(4);

    for symbol in "[0,1,0,0] > [-,!,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.get_state(), ProcessorState::Halt);
  }

  #[test]
  fn halt_on_third() {
    let mut p = Processor::new(4);

    for symbol in "[0,0,1,0] > [-,-,!,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.get_state(), ProcessorState::Halt);
  }

  #[test]
  fn halt_on_fourth() {
    let mut p = Processor::new(4);

    for symbol in "[0,0,0,1] > [-,-,-,!]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.get_state(), ProcessorState::Halt);
  }

  #[test]
  fn dont_halt_on_first() {
    let mut p = Processor::new(4);

    for symbol in "[0,0,0,0] > [!,-,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.get_state(), ProcessorState::Ok);
  }

  #[test]
  fn dont_halt_on_second() {
    let mut p = Processor::new(4);

    for symbol in "[0,0,0,0] > [-,!,-,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.get_state(), ProcessorState::Ok);
  }

  #[test]
  fn dont_halt_on_third() {
    let mut p = Processor::new(4);

    for symbol in "[0,0,0,0] > [-,-,!,-]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.get_state(), ProcessorState::Ok);
  }

  #[test]
  fn dont_halt_on_fourth() {
    let mut p = Processor::new(4);

    for symbol in "[0,0,0,0] > [-,-,-,!]".chars() {
      p.execute_symbol(symbol);
    }

    assert_eq!(p.get_state(), ProcessorState::Ok);
  }

}
