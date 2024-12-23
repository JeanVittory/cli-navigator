use std::{collections::HashMap, env};
use clap::Parser;
use dotenv::dotenv;


#[derive(Parser)]
struct Args {
    project_name: String
}

fn load_project_paths() -> HashMap<String, String> {
    let mut final_paths = HashMap::new();

    let aliases_str = match env::var("PROJECT_ALIASES") {
        Ok(val) => val,
        Err(err) => {
            println!("Error leyendo PROJECT_ALIASES: {:?}", err);
            return final_paths;
        },
    };

    let aliases: HashMap<String, Vec<String>> = match serde_json::from_str(&aliases_str) {
        Ok(val) => val,
        Err(err) => {
            println!("Error deserializando aliases: {:?}", err);
            return final_paths;
        },
    };

    let paths_str = match env::var("PROJECT_PATHS") {
        Ok(val) => val,
        Err(err) => {
            println!("Error leyendo PROJECT_PATHS: {:?}", err);
            return final_paths;
        },
    };

    let project_paths: HashMap<String, String> = match serde_json::from_str(&paths_str) {
        Ok(val) => val,
        Err(err) => {
            println!("Error deserializando project_paths: {:?}", err);
            return final_paths;
        },
    };

    for (project_key, alias_list) in aliases {
        if let Some(path) = project_paths.get(&project_key) {
            for alias in alias_list {
                final_paths.insert(alias, path.clone());
            }
        }
    }

    final_paths
}


fn main() {
    dotenv().ok();
    let args = Args::parse();
    let project_name: String = args.project_name;
    let project_paths = load_project_paths();

    println!("Project name: {}", project_name);
    println!("Available paths: {:?}", project_paths);

    match project_paths.get(&project_name) {
        Some(path) => print!("{}", path),
        None => {
            eprintln!("Error: Unknown project '{}'", project_name);
            std::process::exit(1);
        },
    }
}

