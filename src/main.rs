use std::fs::create_dir_all;
use std::io::{self, stdin};
use std::path::{Path, PathBuf};
use std::process::exit;

//struct Data {}

fn main() {
    println!("\t\t~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\t\t| installing mods the easy way uwu |\n\t\t~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");

    println!("> Please press enter to continue when presented with");

    wait_enter();

    #[cfg(windows)]
    let app_data = find_appata();

    println!("> Determined your %appdata% folder to be {}\n  If this is incorrect please close the program now!\n", app_data);

    wait_enter();

    // Check if gwep_installer folder exists, if not run first time setup
    let gwep_installer = format!("{}\\gwep_installer", app_data);
    let gwep_installer_path = Path::new(&gwep_installer);
    if !gwep_installer_path.exists() {
        first_time_setup(gwep_installer_path);
    }

    let vintage_story_path = format!("{}\\VintagestoryData", app_data);

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
    println!("> Running first time setup for gwep_installer!");

    println!("> Are you okay to install data to {:?} ? (y/N)", path);

    if !get_y_n(false) {
        println!("> User did not consent to the program being installed! Terminating program!");
        wait_enter();
        exit(0);
    }

    create_dir_all(path.join("mod_packs"))
        .expect("Critical Error occured while attempting to install in appdata folder!");

    println!("> gwep_installer is now installed to {:?} ...", path);
}

fn get_y_n(default: bool) -> bool {
    loop {
        let mut response = String::new();

        let stdin = io::stdin();

        stdin
            .read_line(&mut response)
            .expect("Something went horribly wrong while getting user input!");

        response = response.trim().to_ascii_lowercase();

        if response == "" {
            return default;
        }
        if response == "y" {
            return true;
        }
        if response == "n" {
            return false;
        }

        println!("Please respond with y or n!");
    }
}

fn get_num(length: usize) -> usize {
    loop {
        let mut response = String::new();

        let stdin = io::stdin();

        stdin
            .read_line(&mut response)
            .expect("Something went horribly wrong while getting user input!");

        match response.trim().parse::<usize>() {
            Ok(v) => {
                if v <= length && v > 0 {
                    return v;
                } else {
                    println!("Input value is outside the correct bounds!")
                }
            }
            Err(e) => println!("Your input was not correct: {e}"),
        };
    }
}

fn wait_enter() {
    //Dirty way to wait for enter to be pressed...

    println!("> ...");

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
    println!("> Your Vintage Story data is in {}", path.to_str().unwrap());

    let config_path = path.join("ModConfig");

    if !config_path.exists() {
        println!("ModConfig folder being created for you since mods have not been used on this installation.");
        create_dir_all(config_path)
            .expect("Critical Error occured while attempting to create ModConfig directory!");
    }

    println!("> Check succeeded!");
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
        let mut count = 1;
        for pack in &modpacks {
            println!("{count}. {:?}", pack.file_name());
            count += 1;
        }
        modpack = modpacks
            .get(get_num(modpacks.len()) - 1)
            .unwrap()
            .to_path_buf();
        println!("> Selected modpack \"{}\" !", modpack.display());
    } else {
        modpack = modpacks.pop().unwrap();
        println!("> Modpack \"{}\" found!", modpack.display());
    }

    let mods = std::fs::read_dir(modpack.join("mods"))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    //mods.sort();

    println!(
        "\t~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\t{} required mods to install\n\t~~~~~~~~~~~~~~~~~~~~~~~~~~~",
        mods.len()
    );

    println!("> Do you want to display the contained mods? (y/N)");

    if get_y_n(false) {
        println!("\nRequired Mods \n-------------");
        for file in mods {
            println!("{:?}", file.file_name().unwrap())
        }
        println!("-------------");
    }
    Ok(())
}
