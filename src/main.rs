use std::{collections::HashMap, env, fs};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use serde_json::{self, json};


#[derive(Parser, Debug)]
#[command(name="nav", version, about="Move quickly between projects.")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Go {
        project_name: String
    },
    
    Add {
        project_key: String,
        path: String,
        aliases: Vec<String>
    },

    Remove {
        project_key: String
    },

    List,
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

fn list_projects() {
    let project_paths = load_env_var_as_json("PROJECT_PATHS");
    let project_aliases = load_env_var_as_json("PROJECT_ALIASES");
    
    println!("Available projects:");
    for (key, path) in project_paths {
        println!("- {}: {}", key, path);
        if let Some(aliases) = project_aliases.get(&key) {
            println!("  Aliases: {}", aliases);
        }
    }
}

fn add_project(project_key: &str, path: &str, aliases: &[String]) {
    let env_path = get_env_path();
    
    let mut project_paths = load_env_var_as_json("PROJECT_PATHS");
    let mut project_aliases = load_env_var_as_json("PROJECT_ALIASES");

    project_paths.insert(project_key.to_string(), json!(path));
    project_aliases.insert(project_key.to_string(), json!(aliases));
    
    update_env_file(&env_path, "PROJECT_PATHS", &project_paths);
    update_env_file(&env_path, "PROJECT_ALIASES", &project_aliases);
    
    println!("Project '{}' added successfully!", project_key);
}

fn remove_project(project_key: &str) {
    let env_path = get_env_path();
    
    let mut project_paths = load_env_var_as_json("PROJECT_PATHS");
    let mut project_aliases = load_env_var_as_json("PROJECT_ALIASES");
    
    if project_paths.remove(project_key).is_none() {
        eprintln!("Project '{}' not found", project_key);
        return;
    }
    project_aliases.remove(project_key);
    
    update_env_file(&env_path, "PROJECT_PATHS", &project_paths);
    update_env_file(&env_path, "PROJECT_ALIASES", &project_aliases);
    
    println!("Project '{}' removed successfully!", project_key);
}


fn load_env_var_as_json(var_name: &str) -> serde_json::Map<String, serde_json::Value> {
    env::var(var_name)
        .map(|val| serde_json::from_str(&val).unwrap_or_default())
        .unwrap_or_default()
}

fn get_env_path() -> String {
    env::var("PROJECT_NAVIGATOR_PATH")
        .expect("PROJECT_NAVIGATOR_PATH not set") + "/.env"
}

fn update_env_file(env_path: &str, var_name: &str, data: &serde_json::Map<String, serde_json::Value>) {
    let content = fs::read_to_string(env_path).unwrap_or_default();
    let new_line = format!("{}='{}'", var_name, serde_json::to_string(data).unwrap());
    
    let new_content = if content.contains(&format!("{}=", var_name)) {
        content.replace(
            &content.lines().find(|l| l.starts_with(&format!("{}=", var_name))).unwrap(),
            &new_line
        )
    } else {
        format!("{}\n{}", content, new_line)
    };
    
    fs::write(env_path, new_content).expect("Unable to write to .env file");
}


fn main() {    
    let args = Args::parse();
    dotenv().ok();

    match args.command {
        Commands::Go { project_name } => {
            let project_paths = load_project_paths();
            match project_paths.get(&project_name) {
                Some(path) => print!("{}", path),
                None => {
                    eprintln!("Error: Unknown project '{}'", project_name);
                    std::process::exit(1);
                },
            }
        }
        Commands::Add { project_key, path, aliases } => {
            add_project(&project_key, &path, &aliases)

        }

        Commands::Remove { project_key } => {
            remove_project(&project_key)
        }

        Commands::List => {
            list_projects();
        }
    }
}

