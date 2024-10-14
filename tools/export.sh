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
            echo "To build client cd into ./client and run 'docker compose build' and 'docker compose up' to run"
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