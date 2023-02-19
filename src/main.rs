use std::io::{self, stdin};
use std::path::Path;

fn main() {
    println!("\t\t~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\t\t| installing mods the easy way uwu |\n\t\t~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");

    #[cfg(windows)]
    let paths = find_appata();
    let app_data = paths.0;
    let vintage_story_path = paths.1;

    println!("> Determined your %appdata% folder to be \"{}\"\n  If this is correct press enter... else close the program now!", app_data);

    //Dirty way to wait for enter to be pressed...
    stdin()
        .read_line(&mut String::new())
        .expect("Something went horribly wrong!");

    println!("> Your Vintage Story data is in {vintage_story_path}\n");

    install_required_mods().expect("Something went wrong when installing required mods!");
}

fn find_appata() -> (String, String) {
    let app_data = std::env::var("APPDATA").expect("No APP_DATA directory");
    let mut vintage_story_path = app_data.clone();
    vintage_story_path.push_str("\\VintagestoryData");

    if !Path::new(&vintage_story_path).exists() {
        panic!("Please run Vintage Story at least once before using this installer!")
    }
    (app_data, vintage_story_path)
}

fn install_required_mods() -> io::Result<()> {
    let mods = std::fs::read_dir("mods")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    //mods.sort();

    println!(
        "{} required mods to install\n~~~~~~~~~~~~~~~~~~~~~~~~~~~",
        mods.len()
    );

    for file in mods {
        println!("{:?}", file)
    }
    Ok(())
}
