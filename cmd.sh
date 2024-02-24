#!/bin/bash

POSITIONAL_ARGS=()
LANGUAGES=("c" "c++/cpp" "python" "rust/rs" "go" "java" "javascript/js" "typescript/ts" "neovim")

DESTINATION="."


while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            HELP=true
            break
            ;;
        -g|--git)
            GIT=true
            shift
            ;;
        -l|--list)
            LIST=true
            shift
            ;;
        *)
            POSITIONAL_ARGS+=("$1")
            shift
            ;;
    esac
done

if [[ $HELP ]]; then
    echo "Usage: cmd.sh [options] [files]"
    echo "Options:"
    echo "  -h, --help: Show this help message"
    echo "  -g, --git: Initialize a git repository"
    echo "  -l, --list: List all available languages"
    exit 0
fi

if [[ $LIST ]]; then
    echo "Available languages:"
    echo "  ${LANGUAGES[@]}"
    exit 0
fi


