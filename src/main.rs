use chrono::prelude::*;
use int_enum::IntEnum;
use std::env;

#[repr(u8)]
#[derive(Clone, Copy, Debug, IntEnum)]
enum ModelType {
    GUILD = 0,
    USER = 1,
    CHANNEL = 2,
    MESSAGE = 3,
    ROLE = 4,
    InternalUse = 5,
    DmChannel = 6,
}

fn parse(snowflake: u128) {
    println!("Snowflake {}", snowflake);
    let fc_offset = 1_577_836_800_000;

    // Finds timestamp when snowflake was created and converts to datetime
    let ts_offset_ms = snowflake >> 64;
    let ts_ms = ts_offset_ms + fc_offset;
    let timestamp: i64 = (ts_ms / 1000) as i64;
    let nt = NaiveDateTime::from_timestamp(timestamp, 0);
    let dt: DateTime<Utc> = DateTime::from_utc(nt, Utc);
    let res = dt.format("%Y-%m-%d %H:%M:%S");
    println!("Created at {} ({})", res, timestamp);

    // Finds the model type
    let model_type = (snowflake >> 56 & 0b11111111) as u8;
    let mt_enum = ModelType::from_int(model_type);
    println!("Model Type - {:?}", mt_enum);

    // Finds internal counter number
    let internal_counter = snowflake >> 40 & 0b1111111111111111;
    println!("Internal Counter - {}", internal_counter);

    // Finds the API version
    let api_version = snowflake >> 32 & 0b11111111;
    println!("API Version - {}", api_version);

    // Finds unset bits
    let unset = snowflake & 0b1111111111111111;
    println!("Unset - {}", unset);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let id = args[1].clone();
    let snowflake = id.parse::<u128>().unwrap();
    parse(snowflake)
}
