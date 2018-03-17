extern crate lc4;

fn main() {
    let register_count = lc4::get_register_count_parmeter(4);

    let mut processor = lc4::Processor::new(register_count);

    processor.print_register_values("INT");

    while processor.get_state() != lc4::ProcessorState::Halt {
        let instruction = lc4::read_instruction();

        for symbol in instruction.chars() {
            processor.execute_symbol(symbol);
        }

        processor.print_register_values("REG")
    }
}
