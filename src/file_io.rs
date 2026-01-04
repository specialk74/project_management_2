use std::fs::File;
use std::io::Write;

use crate::models::EffortsDto;

pub fn save_efforts_to_file(efforts: &EffortsDto, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(efforts).unwrap();
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_efforts_from_file(path: &str) -> EffortsDto {
    let json_str = std::fs::read_to_string(path);
    if let Ok(json_str) = json_str {
        let efforts_dto: Result<EffortsDto, serde_json::Error> = serde_json::from_str(&json_str);
        if let Ok(efforts_dto) = efforts_dto {
            return efforts_dto;
        } else {
            println!(
                "{}",
                efforts_dto
                    .expect_err(format!("Error during parse the file \"{}\"", path).as_str())
            );
        }
    } else {
        println!(
            "{}",
            json_str.expect_err(format!("Error during load the file \"{}\"", path).as_str())
        );
    }
    println!("Create a default EffortsDto");
    EffortsDto::default()
}
