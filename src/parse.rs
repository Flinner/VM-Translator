use crate::types::*;

pub fn parse_line(line: &str) -> Result<Option<ParsedLine>, ParseError> {
    // ignore empty lines
    let line = line.split("//").take(1).collect::<Vec<_>>()[0].trim();
    if line.is_empty() {
        return Ok(None);
    }

    let sp: Vec<&str> = line.split(' ').collect();
    let parsed: ParsedLine = match sp.len() {
        1 => {
            if sp[0] == "return" {
                //could be a `return`
                ParsedLine::Return
            } else {
                //or `add`, `sub`....
                ParsedLine::Arithmetic(get_arithmatic(sp[0])?)
            }
        }
        2 => ParsedLine::FlowControl(get_flow_control(sp[0], sp[1])?),
        3 => {
            // could be  function declartion (`function f n`)
            if sp[0] == "function" {
                ParsedLine::Function(get_fn(sp[1], sp[2])?)
            // or a function call, (`call f n`)
            } else if sp[0] == "return" {
                ParsedLine::FunctionCall(get_fn_call(sp[1], sp[2])?)
                // or a cmd (`push local 4`)
            } else {
                ParsedLine::Command(get_command(sp[0], sp[1], sp[2])?)
            }
        }
        _ => return Err(ParseError("MORE THAN 3 COMMANds!".to_string())),
    };

    Ok(Some(parsed))
}

/// ```
/// function func n // function named func. and n local vars
/// ```
fn get_fn<'a>(name: &'a str, local_vars: &'a str) -> Result<Function<'a>, ParseError> {
    let local_vars: u16 = local_vars.parse()?;
    Ok(Function { name, local_vars })
}

/// ```
/// call func n // call function named func. and n args pushed to stack
/// ```
fn get_fn_call<'a>(name: &'a str, args: &'a str) -> Result<FunctionCall<'a>, ParseError> {
    let args: u16 = args.parse()?;
    Ok(FunctionCall { name, args })
}

fn get_arithmatic(s: &str) -> Result<Arithmetic, ParseError> {
    use Arithmetic::*;
    let arithmetic = match s {
        "add" => Add,
        "sub" => Sub,
        "neg" => Neg,
        "eq" => Eq,
        "gt" => Gt,
        "lt" => Lt,
        "and" => And,
        "or" => Or,
        "not" => Not,
        a => return Err(ParseError(format!("illegal arithmetic operation: {}", a))),
    };
    Ok(arithmetic)
}

fn get_command<'a>(
    action: &'a str,
    segment: &'a str,
    address: &'a str,
) -> Result<Command, ParseError> {
    use Action::*;
    use Segment::*;
    let action = match action {
        "pop" => Pop,
        "push" => Push,
        a => return Err(ParseError(format!("illegal action: {}", a))),
    };
    let segment = match segment {
        "argument" => Argument,
        "local" => Local,
        "static" => Static,
        "constant" => Constant,
        "this" => This,
        "that" => That,
        "pointer" => Pointer,
        "temp" => Temp,
        m => return Err(ParseError(format!("illegal memory segment : {}", m))),
    };
    let address: u16 = address.parse()?;
    Ok(Command {
        action,
        address,
        segment,
    })
}

fn get_flow_control<'a>(flow_type: &str, label: &'a str) -> Result<FlowControl<'a>, ParseError> {
    use FlowType::*;
    let flow_type = match flow_type {
        "label" => Label,
        "if-goto" => IfGoto,
        "goto" => Goto,
        f => return Err(ParseError(format!("unknown command: {}", f))),
    };
    Ok(FlowControl { flow_type, label })
}
