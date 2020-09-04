use funcs;
use major::*;
use std::io;

fn main() {
    println!("Старт");

    print!("Введите команду: ");
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        // println!(
        //     "command={}",
        //     command.split_ascii_whitespace().enumerate().collect()
        // );
        if call(parse_cmd(
            command
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect(),
        )) {
            break;
        }
        // call(parse_cmd(env::args().collect()));
    }
    println!("Финиш");
}

fn call(c: DoWhat) -> bool {
    let cmd = c.cmd.unwrap();
    let mut ret = false;

    match cmd.as_str() {
        "TRAIN" => {
            println!(
                "\tИщем электрички из {} в {}",
                c.params[0].as_str(),
                c.params[1].as_str()
            );
            let r = match funcs::train::get_train(c.params[0].to_string(), c.params[1].to_string())
            {
                Ok(s) => s,
                Err(e) => format!("Ошибка:{}", e.to_string()),
            };
            println!("\t{}", r);
        }
        "ALARM" => {
            println!("\tСтавим будильник на {}", c.params[0].as_str());
            let r = match funcs::alarm::set_alarm(c.params[0].to_string()) {
                Ok(s) => s,
                Err(e) => format!("Ошибка:{}", e.to_string()),
            };
            println!("\t{}", r);
        }
        "QUIT" => {
            println!("просят выйти");
            ret = true;
        }
        _ => println!("\n\nНе знаю что делать: {}\n\n", cmd.as_str()),
    };

    ret
}
