# Project Navigator 🌟  

A custom tool to navigate through your personal projects effortlessly.  

## Features 🚀  
- Define aliases for your projects.  
- Quickly navigate to predefined project paths.  
- Flexible configuration via environment variables.  

## Setup 🛠️  

1. Clone this repository:  
   ```bash
   git clone https://github.com/JeanVittory/project-navigator.git
   cd project-navigator
   
# Running and Compiling the Project with Cargo 🚀  

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

Your file should look something like this:

```bash 
export PROJECT_NAVIGATOR_PATH="/home/your-user/the-route/to/your/rust-project/previously/cloned-it"

nav() {
    if [ -z "$PROJECT_NAVIGATOR_PATH" ]; then
        echo "La variable PROJECT_NAVIGATOR_PATH no está configurada."
        return 1
    fi

    ENV_FILE="$PROJECT_NAVIGATOR_PATH/.env"
    if [ ! -f "$ENV_FILE" ]; then
        echo "No se encontró el archivo .env en $ENV_FILE"
        return 1
    fi

    echo "Cargando variables desde $ENV_FILE..."
    while IFS= read -r line; do
        if [[ $line =~ ^[^#].+=.+ ]]; then
            export "$line"
        fi
    done < "$ENV_FILE"

    echo "PROJECT_ALIASES: $PROJECT_ALIASES"
    echo "PROJECT_PATHS: $PROJECT_PATHS"

    target_dir=$(project_navigator "$1")
    if [ $? -eq 0 ]; then
        cd "$target_dir" || echo "No se pudo cambiar al directorio $target_dir"
    else
        echo "Error al ejecutar project_navigator."
    fi
}
```

after that, run:
```
source ~/.bashrc
```

Then, go to your Rust project directory and run the following command:

```
cargo build --release
```

⚠️ Remember to find the command to compile the binary for your preferred OS.


Next, take the file from ```target/release/project_navigator``` and move it to ```/usr/local/bin``` like this:

```
sudo cp target/release/project_navigator /usr/local/bin/nav
```

Enjoy!

⚠️ Please, remember to create your .env file with your personal paths following the structure of .env.example

