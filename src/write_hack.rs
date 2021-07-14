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
    match action {
        Action::Push => 1,
        Action::Pop => 1,
    };
    match segment {
        Segment::Argument => 1,
        Segment::Local => 1,
        Segment::Static => 1,
        Segment::Constant => 1,
        Segment::This => 1,
        Segment::That => 1,
        Segment::Pointer => 1,
        Segment::Temp => 1,
    };
    let _ = address;
    todo!()
}

fn convert_arithmetic(arithmetic: Arithmetic) -> String {
    println!("{:?}", arithmetic);
    todo!()
}
