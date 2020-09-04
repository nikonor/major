pub mod funcs;

pub struct DoWhat {
    pub cmd: Option<String>,
    pub params: Vec<String>,
}

pub fn parse_cmd(args: Vec<String>) -> DoWhat {
    let cmd = args[0].to_string();

    let ret = DoWhat {
        cmd: Some(cmd.to_uppercase()),
        params: args[1..].to_vec(),
    };

    return ret;
}
