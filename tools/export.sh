#!/bin/bash

# Function to display the options
display_options() {
    echo "Please select one of the following options:"
    echo "1. API"
    echo "2. Client"
    echo "3. WASM Client"
    echo "4. Server"
}

# Function to execute commands based on user selection
execute_command() {
    mkdir export
    case $1 in
        1)
            echo "You selected API. Building..."
            rm -Rf ./export/api
            mkdir export/api
            cd ./api/
            cargo build --release
            cd ..
            cp ./api/target/release/api ./export/api/
            echo "Exported api binary to ./export/api/api"
            ;;
        2)
            echo "You selected Client. Building..."
            # Replace the below command with the actual command for Client
            # Example: ./client_executable
            ;;
        3)
            echo "You selected WASM Client. Building..."

            if ! command -v wasm-bindgen &> /dev/null
            then
                echo "wasm-bindgen could not be found. Install using cargo install -f wasm-bindgen-cli"
                exit 1
            fi

            rm -Rf ./export/wasm_client
            mkdir -p ./export/wasm_client/wasm
            export $(grep -v '^#' .env | xargs)

            cp -R ./site/* ./export/wasm_client
            rm -Rf ./export/wasm_client/wasm/*
            cd client
            wasm-pack build --out-dir ../export/wasm_client/wasm/ --target bundler --release --bin rc_client
            cd ..

            cp -R ./assets ./export/wasm_client/public/

            echo "Exported site to ./export/wasm_client"
            ;;
        4)
            echo "You selected Server. Building..."
            rm -Rf ./export/server
            mkdir export/server
            cargo build --release --bin rc_server
            cp ./target/release/rc_server ./export/server/rc_server
            ;;
        *)
            echo "Invalid selection. Please try again."
            ;;
    esac
}

# Main loop
while true; do
    display_options
    read -p "Enter your choice (1-4): " choice
    if [[ "$choice" =~ ^[1-4]$ ]]; then
        execute_command "$choice"
        break
    else
        echo "Invalid option. Please enter a number between 1 and 4."
    fi
done