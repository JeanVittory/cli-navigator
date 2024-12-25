#!/bin/bash

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

    echo "Loading variables from $ENV_FILE..."
    while IFS= read -r line; do
        if [[ $line =~ ^[^#].+=.+ ]]; then
            processed_line=$(echo "$line" | sed "s/'//g")
            export "$processed_line"
        fi
    done < "$ENV_FILE"

    echo "PROJECT_ALIASES: $PROJECT_ALIASES"
    echo "PROJECT_PATHS: $PROJECT_PATHS"

    target_dir=$(project_navigator "$1")
    if [ $? -eq 0 ]; then
        cd "$target_dir" || echo "Was not posible move into -> $target_dir"
    else
        echo "Error executing project_navigator file, please check the Rust project."
    fi
}