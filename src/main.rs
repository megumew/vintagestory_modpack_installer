use std::{ops::Add, str::FromStr};

fn main() {
    println!("\t\t~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\t\t|installing mods the easy way uwu|\n\t\t~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");

    #[cfg(windows)]
    let app_data = std::env::var("APPDATA").expect("No APP_DATA directory");
    let mut vintage_story_data: String = String::from(app_data);
    vintage_story_data.push_str("\\VintagestoryData");

    println!("> Your Vintage Story data is in {vintage_story_data}");
}
