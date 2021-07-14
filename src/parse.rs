use crate::types::*;

pub fn parse_line(line: &str) -> Result<Option<ParsedLine>, ParseError> {
    // ignore empty lines
    let line = line.split("//").take(1).collect::<Vec<_>>()[0].trim();
    if line.is_empty() {
        return Ok(None);
    }

    let sp: Vec<&str> = line.split(' ').collect();
    let parsed: ParsedLine = match sp.len() {
        1 => ParsedLine::Arithmetic(get_arithmatic(sp[0])?),
        2 => return Err(ParseError("NOT IMPLEMENTED!".to_string())),
        3 => ParsedLine::Command(get_command(sp[0], sp[1], sp[2])?),
        _ => return Err(ParseError("MORE THAN 3 COMMANds!".to_string())),
    };

    Ok(Some(parsed))
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
