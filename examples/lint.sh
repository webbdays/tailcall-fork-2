#!/bin/bash

# Performs a basic check on the files in the examples directory

# Function to print an error message and exit
error_exit() {
    echo "Error: $1" >&2
    exit 1
}

# Function to check files with the specified extensions using tailcall
check_files() {
    local path="./examples"
    local depth=1
    local -a extensions=("-name" "*.json" -o "-name" "*.yml" -o "-name" "*.yaml" -o "-name" "*.graphql" -o "-name" "*.gql")
    local command="./target/debug/tailcall check"
    local -a ignore=("!" "-name" "grpc-reflection.graphql")

    # Execute find command with constructed options and extensions
    find "$path" -maxdepth "$depth" \( "${extensions[@]}" \) "${ignore[@]}" -exec sh -c '
        for file; do
            echo "Checking file: $file"
            '"$command"' "$file" || exit 255
        done
    ' sh {} + || error_exit "tailcall check failed for one or more files."
}

# Main script execution
main() {
    check_files
}

# Start the script
main
