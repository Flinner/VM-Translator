/**
Performs a pop operation on `Argument`, `Local`, `This` and `That`
*/
pub fn pop(seg: &str, address: u16) -> String {
    format!(
        "\
@{address}
D=A
@{seg}
D=D+M
{}",
        POP_TO_D,
        address = address,
        seg = seg,
    )
}

/**
Performs a pop operation on tmp and pointer
with an offset: pointer=3, temp=5
*/
pub fn pop_offset(offset: u16, address: u16) -> String {
    format!(
        "\
@{address}
D=A
@{offset}
D=D+A
{}",
        POP_TO_D,
        address = address,
        offset = offset,
    )
}

/// Pops to address in D
/// D is an address
pub const POP_TO_D: &str = "\
@R13
M=D

@SP
M=M-1
A=M
D=M

@R13
A=M
M=D";

pub fn push(seg: &str, address: u16) -> String {
    format!(
        "\
@{address}
D=A
@{seg}
D=D+M
A=D
D=M
{}",
        PUSH_FROM_D,
        seg = seg,
        address = address
    )
}

pub fn push_offset(offset: u16, address: u16) -> String {
    format!(
        "\
@{address}
D=A
@{offset}
D=D+A
A=D
D=M
{}",
        PUSH_FROM_D,
        offset = offset,
        address = address
    )
}

/// pushs from value in D
/// D is a value
pub const PUSH_FROM_D: &str = "\
@SP
A=M
M=D
@SP
M=M+1";

//====================================================================================//
//                            ARITHMETIC                                              //
//====================================================================================//
pub const ADD: &str = "\
@SP
M=M-1
A=M
D=M

@SP
M=M-1
A=M
D=D+M";

pub const SUB: &str = "\
@SP
M=M-1
A=M
D=M

@SP
M=M-1
A=M
D=M-D";

pub const NEG: &str = "\
@SP
M=M-1
A=M
D=-M";

pub fn compare(i: usize, jump: &str) -> String {
    format!(
        "\
@SP
M=M-1
A=M
D=M//a

@SP 
M=M-1
A=M //b
D=M-D // b - a

@{jump}.{i}
D;{jump}
D=0 //false

@END_{jump}.{i}
0;JMP

({jump}.{i})
D=-1  //true
    
(END_{jump}.{i})
{}",
        PUSH_FROM_D,
        i = i,
        jump = jump
    )
}

pub const AND: &str = "\
@SP
M=M-1
A=M
D=M

@SP
M=M-1
A=M
D=D&M";

pub const OR: &str = "\
@SP
M=M-1
A=M
D=M

@SP
M=M-1
A=M
D=D|M";

pub const NOT: &str = "\
@SP
M=M-1
A=M
D=!M";

/// *SP-- should be set to -1 to be true,
/// also `D` is used for the jump
/// Do after this!
///```
/// @{label}
/// D;JEQ";
///```
pub const IF_GOTO: &str = "\
@SP
M=M-1 // SP--
A=M
D=M   // D = *SP
@1
D=D+A // D = D + 1";
// with format:
