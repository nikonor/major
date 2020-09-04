use core::fmt::Error;

pub fn set_alarm(time: String) -> Result<String, Error> {
    println!("\t\tВызов set_alarm для {}", time);
    Ok("Ok".to_string())
}
