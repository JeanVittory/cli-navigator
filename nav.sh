#!/bin/bash

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