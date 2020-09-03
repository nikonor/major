mod train;

use std::fmt::Error;

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

pub fn get_train(from: String, to: String) -> Result<String, Error> {
    println!("\t\tВызов get_train для {}-{}", from, to);
    self::train::qwe();
    Ok("Ok".to_string())
}

pub fn set_alarm(time: String) -> Result<String, Error> {
    println!("\t\tВызов set_alarm для {}", time);
    Ok("Ok".to_string())
}
