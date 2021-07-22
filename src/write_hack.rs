use crate::hack_binary::*;
use crate::types::{Action, Arithmetic, Command, FlowControl, FlowType, ParsedLine, Segment};

pub fn convert(parsed: ParsedLine, i: usize) -> String {
    match parsed {
        ParsedLine::Arithmetic(arithmetic) => convert_arithmetic(arithmetic, i),
        ParsedLine::Command(command) => convert_command(command),
        ParsedLine::FlowControl(flow) => convert_flow_control(flow),
        ParsedLine::Return => todo!(),
        ParsedLine::Function(func) => todo!(),
        ParsedLine::FunctionCall(call) => todo!(),
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
            Argument => push("ARG", address),
            Local => push("LCL", address),
            Static => format!("@file.{}\nD=M\n{}", address, PUSH_FROM_D),
            Constant => format!("@{}\nD=A\n{}", address, PUSH_FROM_D),
            This => push("THIS", address),
            That => push("THAT", address),
            Pointer => push_offset(3, address),
            Temp => push_offset(5, address),
        },
        Action::Pop => match segment {
            Argument => pop("ARG", address),
            Local => pop("LCL", address),
            Static => format!("@file.{}\nD=A\n{}", address, POP_TO_D),
            Constant => panic!("Cannot Pop constant!"),
            This => pop("THIS", address),
            That => pop("THAT", address),
            Pointer => pop_offset(3, address),
            Temp => pop_offset(5, address),
        },
    }
}

fn convert_arithmetic(arithmetic: Arithmetic, i: usize) -> String {
    use Arithmetic::*;
    match arithmetic {
        Add => return format!("{}\n{}", ADD, PUSH_FROM_D),
        Sub => return format!("{}\n{}", SUB, PUSH_FROM_D),
        Neg => return format!("{}\n{}", NEG, PUSH_FROM_D),
        Eq => return compare(i, "JEQ"),
        Gt => return compare(i, "JGT"),
        Lt => return compare(i, "JLT"),
        And => return format!("{}\n{}", AND, PUSH_FROM_D),
        Or => return format!("{}\n{}", OR, PUSH_FROM_D),
        Not => return format!("{}\n{}", NOT, PUSH_FROM_D),
    };
}

fn convert_flow_control(FlowControl { flow_type, label }: FlowControl) -> String {
    use FlowType::*;
    match flow_type {
        Label => return format!("({})", label),
        Goto => return format!("@{}\n0;JMP", label),
        IfGoto => return format!("{}\n@{}\nD;JNE", IF_GOTO, label),
    }
}
