use core::fmt::Error;

pub fn get_train(from: String, to: String) -> Result<String, Error> {
    println!("\t\tВызов get_train для {}-{}", from, to);
    Ok("Ok".to_string())
}
