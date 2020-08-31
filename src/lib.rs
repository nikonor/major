use std::fmt::Error;

pub struct DoWhat{
    pub cmd: Option<String>,
    pub params: Vec<String>
}

pub fn parse_cmd(args: Vec<String>) -> DoWhat{
    let cmd = args[1].to_string();

    DoWhat{cmd: Some(cmd.to_uppercase()),  params: args[2..].to_vec() }
}

pub fn get_train(from:String, to: String) -> Result<String, Error> {
    println!("\t\tВызов get_train для {}-{}", from, to);
    Ok("Ok".to_string())
}

pub fn set_alarm(time:String) -> Result<String, Error> {
    println!("\t\tВызов set_alarm для {}", time);
    Ok("Ok".to_string())
}