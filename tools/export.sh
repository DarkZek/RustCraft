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
            echo "To build api cd into ./api and run 'docker compose build' and 'docker compose up' to run"
            ;;
        2)
            echo "You selected Client. Building..."
            # Replace the below command with the actual command for Client
            # Example: ./client_executable
            ;;
        3)
            echo "You selected WASM Client. Building..."

            if ! command -v wasm-pack &> /dev/null
            then
                echo "wasm-pack could not be found. Install via https://rustwasm.github.io/wasm-pack/installer/"
                exit 1
            fi

            rm -Rf ./export/wasm_client
            mkdir -p ./export/wasm_client/site/wasm
            export $(grep -v '^#' build.env | xargs)

            cp -R ./site ./export/wasm_client
            rm -Rf ./export/wasm_client/site/wasm/*
            rm -Rf ./export/wasm_client/site/dist
            rm -Rf ./export/wasm_client/site/node_modules
            cd client
            wasm-pack build --out-dir ../export/wasm_client/site/wasm/ --target bundler --release --bin rc_client
            cd ..

            cp -R ./assets ./export/wasm_client/site/public/

            cd ./export/wasm_client/site/

            pwd

            npm i
            npm run build

            if [ ! -d ./dist ]; then
              echo "Build failed"
              exit
            fi

            cd ../../../

            cp -R ./export/wasm_client/site/dist ./export/wasm_client
            cp ./export/wasm_client/site/docker-compose.yml ./export/wasm_client/
            cp ./export/wasm_client/site/nginx.conf ./export/wasm_client/

            rm -Rf ./export/wasm_client/site

            echo "Exported site to ./export/wasm_client. Run 'docker compose up' in it to start wasm"
            ;;
        4)
            echo "You selected Server. Building..."
            rm -Rf ./export/server
            mkdir export/server
            cargo build --release --bin rc_server
            cp ./target/release/rc_server ./export/server/rc_server

            cp ./server/docker-compose.yml ./export/server/
            cp ./server/Dockerfile ./export/server/
            cp -R ./assets ./export/server/

            echo "Exported site to ./export/server"
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