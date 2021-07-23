use crate::hack_binary::*;
use crate::types::{
    Action, Arithmetic, Command, FlowControl, FlowType, Function, FunctionCall, ParsedLine, Segment,
};

pub fn convert(parsed: ParsedLine, i: usize) -> String {
    match parsed {
        ParsedLine::Arithmetic(arithmetic) => convert_arithmetic(arithmetic, i),
        ParsedLine::Command(command) => convert_command(command),
        ParsedLine::FlowControl(flow) => convert_flow_control(flow),
        ParsedLine::Return => convert_return(),
        ParsedLine::Function(func) => convert_function(func),
        ParsedLine::FunctionCall(call) => convert_call(call),
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

fn convert_function(Function { name, local_vars }: Function) -> String {
    // init all local vars with 0
    format!(
        "
({})
@0
D=A
{}",
        name,
        PUSH_FROM_D.repeat(local_vars)
    )
}

fn convert_call(FunctionCall { name, args }: FunctionCall) -> String {
    format!(
        "
{save_ret_address}
{save_lcl}
{save_arg}
{save_this}
{save_that}

@SP // ARG=SP-args-5
D=M
@5
D=D-A
@{args}
D=D-A
@ARG
M=D

@SP  // LCL=SP
D=M
@LCL
M=D


@{name}
0;JMP
(return-{name})
",
        save_ret_address = save_seg(&("return-".to_string() + name)), // RET address
        save_lcl = save_seg("LCL"),
        save_arg = save_seg("ARG"),
        save_this = save_seg("THIS"),
        save_that = save_seg("THAT"),
        args = args,
        name = name,
    )
}

fn convert_return() -> String {
    format!(
        "
@LCL // TMP=LCL
D=M
@R15
M=D

{POP_TO_D}
@ARG
A=M
M=D // saving return value, *ARG=pop()

@ARG
D=M
@SP
M=D+1 // SP = ARG + 1

//restore THAT,THIS,ARG,LCL
{restore_that}
{restore_this}
{restore_arg}
{restore_lcl}
{restore_address}
//
@R14 // RET
A=M
0;JMP
",
        POP_TO_D = POP_TO_D,
        restore_this = restore_seg("THIS"),
        restore_that = restore_seg("THAT"),
        restore_arg = restore_seg("ARG"),
        restore_lcl = restore_seg("LCL"),
        restore_address = restore_seg("R14"), // RET address
    )
}
