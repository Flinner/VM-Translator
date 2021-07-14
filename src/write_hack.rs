use crate::types::{Action, Arithmetic, Command, ParsedLine, Segment};

pub fn convert(parsed: ParsedLine) -> String {
    match parsed {
        ParsedLine::Arithmetic(arithmetic) => convert_arithmetic(arithmetic),
        ParsedLine::Command(command) => convert_command(command),
    }
}

fn convert_command(
    Command {
        action,
        segment,
        address,
    }: Command,
) -> String {
    use Segment::*;
    match action {
        Action::Push => match segment {
            Argument => 1,
            Local => 1,
            Static => 1,
            Constant => return format!("@{}\nD=A\n@SP\nA=D", address),
            This => 1,
            That => 1,
            Pointer => 1,
            Temp => 1,
        },
        Action::Pop => match segment {
            Argument => return pop("ARG", address),
            Local => return pop("LCL", address),
            Static => 1,
            Constant => return "Cannot Pop constant!".to_string(),
            This => return pop("THIS", address),
            That => return pop("THAT", address),
            Pointer => 1,
            Temp => 1,
        },
    };
    todo!()
}

fn convert_arithmetic(arithmetic: Arithmetic) -> String {
    println!("{:?}", arithmetic);
    todo!()
}

/**
Performs a pop operation
assembly code generated is:
```asm
@`seg`
D=M
@`address`
D=D+A
@SP
M=M-1
A=M
M=D
```
*/
fn pop(seg: &str, address: u16) -> String {
    format!("@{}\nD=M\n@{}\nD=D+A\n@SP\nM=M-1\nA=M\nM=D", seg, address)
}
