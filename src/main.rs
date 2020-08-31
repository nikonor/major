use std::env;
use major::*;

fn main() {
    println!("Старт");
    call(parse_cmd(env::args().collect()));
    println!("Финиш");
}

fn call(c: DoWhat){
    let cmd = c.cmd.unwrap();
    if  cmd =="TRAIN" {
        println!("\tИщем электрички из {} в {}", c.params[0].as_str(), c.params[1].as_str());
        let r = match get_train(c.params[0].to_string(), c.params[1].to_string()) {
            Ok(s) => s,
            Err(e) => format!("Ошибка:{}", e.to_string())
        };
        println!("\t{}",r);
    } else if cmd =="ALARM" {
        println!("\tСтавим будильник на {}",c.params[0].as_str());
        let r = match set_alarm(c.params[0].to_string()) {
            Ok(s) => s,
            Err(e) => format!("Ошибка:{}", e.to_string())
        };
        println!("\t{}",r);
    } else {
        println!("\n\nНе знаю что делать\n\n")
    }

}