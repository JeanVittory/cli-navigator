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

    case "$1" in
        -a|--add)
            project_navigator add "$2" "$3" "${@:4}"
            ;;
        -r|--remove)
            project_navigator remove "$2"
            ;;
        -l|--list)
            project_navigator list
            ;;
        *)
            target_dir=$(project_navigator go "$1")
            if [ $? -eq 0 ]; then
                cd "$target_dir" || echo "No se pudo cambiar al directorio $target_dir"
            else
                echo "Error al ejecutar project_navigator."
            fi
            ;;
    esac
}