use std::fs;
use std::io::{self, stdin};
use std::path::{Path, PathBuf};
struct Data {}

fn main() {
    println!("\t\t~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\t\t| installing mods the easy way uwu |\n\t\t~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");

    // Check %appdata% for VintagestoryData and make ModConfig folder if it doesn't already exist
    #[cfg(windows)]
    let paths = find_appata();
    let app_data = paths.0;
    let vintage_story_path = paths.1;

    println!("> Determined your %appdata% folder to be {}\n  If this is correct press enter... else close the program now!", app_data);

    wait_enter();

    println!("> Your Vintage Story data is in {vintage_story_path}\n");

    // Backup mod data if it already exists in program folder

    // !TODO Query for local mode or server mode

    // Check for a local modpack
    check_mods_available().expect("Local modpack not installed correctly!");

    wait_enter();
}

fn wait_enter() {
    //Dirty way to wait for enter to be pressed...
    stdin()
        .read_line(&mut String::new())
        .expect("Something went horribly wrong!");
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

fn check_mods_available() -> io::Result<()> {
    let files = std::fs::read_dir("./")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut modpack_found = false;
    let mut modpacks: Vec<PathBuf> = Vec::new();
    for file in files {
        let path = file.join("mods");
        if path.is_dir() {
            modpacks.push(file);
            modpack_found = true;
        }
    }

    let modpack: PathBuf;
    // !TODO NEED TO SET WAY TO SELECT BETWEEN MULTIPLE MODPACKS
    if modpacks.len() > 1 {
        println!("Multiple modpacks found please select one.");
        modpack = modpacks.pop().unwrap();
    } else {
        modpack = modpacks.pop().unwrap();
        println!("Modpack {} found!", modpack.display());
    }

    if !modpack_found {
        println!("No modpack in same folder as program .exe");
        wait_enter();
        panic!("No modpack in same folder as program .exe");
    }

    let mods = std::fs::read_dir(modpack.join("mods"))?
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
