use std::fs::create_dir_all;
use std::io::{self, stdin};
use std::path::{Path, PathBuf};
use std::process::exit;

//struct Data {}

fn main() {
    println!("\t\t~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\t\t| installing mods the easy way uwu |\n\t\t~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");

    #[cfg(windows)]
    let app_data = find_appata();

    println!("> Determined your %appdata% folder to be {}\n  If this is correct press enter... else close the program now!\n", app_data);

    // Check if gwep_installer folder exists, if not run first time setup
    let gwep_installer = format!("{}\\gwep_installer", app_data);
    let gwep_installer_path = Path::new(&gwep_installer);
    if !gwep_installer_path.exists() {
        first_time_setup(gwep_installer_path);
    }

    let vintage_story_path = format!("{}\\VintagestoryData", app_data);

    wait_enter();

    // Check %appdata% for VintagestoryData and make ModConfig folder if it doesn't already exist
    vintage_story_check(vintage_story_path);

    wait_enter();

    // Backup mod data if it already exists in program folder

    // !TODO Query for local mode or server mode

    // Check for a local modpack
    detect_modpack().expect("Local modpack not installed correctly!");

    wait_enter();
}

fn first_time_setup(path: &Path) {
    println!("Running first time setup for gwep_installer!");

    println!("Are you okay to install data to {:?} ? (y/N)", path);

    let allow = get_y_n();

    if !allow {
        println!("User did not consent to the program being installed!");
        exit(0);
    }

    create_dir_all(path.join("mod_packs"))
        .expect("Critical Error occured while attempting to install in appdata folder!");
}

fn get_y_n() -> bool {
    loop {
        let mut response = String::new();

        let stdin = io::stdin();

        stdin
            .read_line(&mut response)
            .expect("Something went horribly wrong while getting user input!");

        response = response.trim().to_ascii_lowercase();

        if response == "y" {
            return true;
        }
        if response == "n" || response == "" {
            return false;
        }

        println!("Please respond with y or n!");
    }
}

fn wait_enter() {
    //Dirty way to wait for enter to be pressed...
    stdin()
        .read_line(&mut String::new())
        .expect("Something went horribly wrong!");
}

fn find_appata() -> String {
    let app_data = std::env::var("APPDATA").expect("No APP_DATA directory");
    let mut vintage_story_path = app_data.clone();
    vintage_story_path.push_str("\\VintagestoryData");

    app_data
}

fn vintage_story_check(input_path: String) {
    println!("> Checking your Vintage Story installation...");
    let path = Path::new(&input_path);
    if !path.exists() {
        panic!("Please run Vintage Story at least once before using this installer!")
    }
    println!(
        "> Your Vintage Story data is in {}\n",
        path.to_str().unwrap()
    );

    let config_path = path.join("ModConfig");

    if !config_path.exists() {
        println!("ModConfig folder being created for you since mods have not been used on this installation.");
        create_dir_all(config_path)
            .expect("Critical Error occured while attempting to create ModConfig directory!");
    }

    println!("> Check succeeded! Press Enter to continue...");
}

fn detect_modpack() -> io::Result<()> {
    let files = std::fs::read_dir("")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut modpack_found = false;
    let mut modpacks: Vec<PathBuf> = Vec::new();
    for file in files {
        if file.is_dir() {
            if file.join("mods").exists() {
                modpacks.push(file);
                modpack_found = true;
            }
        }
    }

    if !modpack_found {
        println!("No modpack in same folder as program .exe");
        wait_enter();
        panic!("No modpack in same folder as program .exe");
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

    let mods = std::fs::read_dir(modpack.join("mods"))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    //mods.sort();

    println!(
        "{} required mods to install\n~~~~~~~~~~~~~~~~~~~~~~~~~~~",
        mods.len()
    );

    for file in mods {
        println!("{:?}", file.file_name().unwrap())
    }
    Ok(())
}
