# Project Navigator 🌟  

Add, remove, and navigate easily between different projects or frequently used folders.

## Features 🚀  
- Define aliases for your projects.  
- Quickly navigate to predefined project paths.  
- Flexible configuration via CLI.  

## Setup 🛠️  

1. Clone this repository:  
   ```bash
   git clone https://github.com/JeanVittory/project-navigator.git
   cd project-navigator
   
## Running and Compiling the Project with Cargo 🚀  

## Install Rust

To download Rustup and install Rust, run the following in your terminal

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Run the Project  
To execute the project locally, use the following command:  
```bash
cargo run
```

To compile the project and use it, copy the code from nav.sh and paste it into your ```.bashrc``` or ```.zshrc``` file. To open one of the files, run:

```
code ~/.bashrc
```
or
```
code ~/.zshrc
```

Then please define your ```PROJECT_NAVIGATOR_PATH``` into the same file like this:

```bash 
export PROJECT_NAVIGATOR_PATH="/home/your-user/the-route/to/your/rust-project/previously/cloned-it"
```

If you don't know which route is run in the console of vscode:

```bash 
pwd 
```

Your file should look something like this:

```bash 
export PROJECT_NAVIGATOR_PATH="/home/your-user/the-route/to/your/rust-project/previously/cloned-it"
export PATH="$HOME/.cargo/bin:$PATH"

nav() {
    if [ -z "$PROJECT_NAVIGATOR_PATH" ]; then
        echo "The variable PROJECT_NAVIGATOR_PATH is not configured."
        return 1
    fi

    ENV_FILE="$PROJECT_NAVIGATOR_PATH/.env"
    if [ ! -f "$ENV_FILE" ]; then
        echo "There is not a .env file into $ENV_FILE"
        return 1
    fi

    export $(grep -v '^#' "$ENV_FILE" | xargs)

    case "$1" in
        -a|--add)
            if [ $# -lt 4 ]; then
                echo "Usage: nav -a <name> <path> <alias1> [alias2...]"
                return 1
            fi
            
            project_navigator add "$2" "$3" "${@:4}"
        ;;
        -r|--remove)
            if [ $# -lt 2 ]; then
                echo "Uso: nav -r <name>"
                return 1
            fi
            project_navigator remove "$2"
        ;;
        -l|--list)
            project_navigator list
        ;;
       -g|--go)
            if [ $# -lt 2 ]; then
                echo "Usage: nav -g <project_name>"
                return 1
            fi
            target_dir=$(project_navigator go "$2" 2>/dev/null)
            if [ -n "$target_dir" ] && [ -d "$target_dir" ]; then
                cd "$target_dir" || {
                    echo "Can't move to $target_dir"
                    return 1
                }
            else
                echo "Error: Project '$2' not found or path invalid"
                return 1    
            fi
        ;;
        "")
            echo "Options:"
            echo "  -a, --add <name> <path> <alias...>  Add Project"
            echo "  -r, --remove <alias>                Delete Project"
            echo "  -l, --list                          Show Projects"
            echo "  -g, --go <alias>                    Move into project"

        ;;
    esac
}
```

after that, run:
```bash
source ~/.bashrc
```

Then, go to your Rust project directory and run the following command:

```bash
cargo build --release
```

⚠️ Remember to find the command to compile the binary for your preferred OS.


Next, take the file from ```target/release/project_navigator``` and move it to ```/usr/local/bin``` like this:

```bash
sudo cp target/release/project_navigator /usr/local/bin/
```

Enjoy!

⚠️ Please, remember to create your .env file with your personal paths following the structure of .env.example



## Usage

The use is very simple. You have some available commands:


## nav

It will show you the available commands:


```bash
nav
  -a, --add <name> <path> <alias...>  Add Project
  -r, --remove <alias>                Delete Project
  -l, --list                          Show Projects
  -g, --go                            Move into project
```

## nav --add

With this command, you can add your projects, which will be available in the `.env` file of the project you cloned at the beginning.
You must pass as parameters:

### name
The project name

### path
The path where the project is located.

### alias 
You must add an alias to refer to the project when you want to move to it.

## nav --remove
To delete a project, you simply need to add the project's alias to the command.

## nav --list
If you want to see the projects you have available, you can use this command.

## nav go
To quickly navigate to a project, you just need to pass the command the alias of the project you want to go to.