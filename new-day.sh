#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

usage() {
    echo "Usage: $(basename "$0") <day-number> [rust|ocaml|both]"
    echo ""
    echo "Arguments:"
    echo "  day-number    Day number (1-25), will be zero-padded"
    echo "  language      Language to set up (default: both)"
    echo ""
    echo "Examples:"
    echo "  $(basename "$0") 1 rust      # Set up day01 for Rust only"
    echo "  $(basename "$0") 5 ocaml     # Set up day05 for OCaml only"
    echo "  $(basename "$0") 12 both     # Set up day12 for both languages"
    echo "  $(basename "$0") 3           # Set up day03 for both languages"
    exit 1
}

if [[ $# -lt 1 || $# -gt 2 ]]; then
    usage
fi

# Parse day number and zero-pad it
day_num="$1"
if ! [[ "$day_num" =~ ^[0-9]+$ ]] || [[ "$day_num" -lt 1 ]] || [[ "$day_num" -gt 25 ]]; then
    echo "Error: Day number must be between 1 and 25" >&2
    exit 1
fi
day_padded=$(printf "%02d" "$day_num")
day_name="day${day_padded}"

# Parse language argument
lang="${2:-both}"
case "$lang" in
    rust|ocaml|both) ;;
    *)
        echo "Error: Language must be 'rust', 'ocaml', or 'both'" >&2
        exit 1
        ;;
esac

setup_rust() {
    local rust_dir="$SCRIPT_DIR/rust"
    local day_dir="$rust_dir/$day_name"

    if [[ -d "$day_dir" ]]; then
        echo "Rust: $day_name already exists, skipping"
        return
    fi

    echo "Setting up Rust $day_name..."

    # Create the crate directory structure
    mkdir -p "$day_dir/src"
    mkdir -p "$day_dir/input"

    # Copy template files
    cp "$rust_dir/template/main.rs" "$day_dir/src/main.rs"
    sed "s/DAY_NAME/$day_name/" "$rust_dir/template/Cargo.toml.tmpl" > "$day_dir/Cargo.toml"

    # Create empty input files
    touch "$day_dir/input/input.txt"
    touch "$day_dir/input/sample.txt"

    # Add to workspace members
    cd "$rust_dir"
    if grep -q 'members = \[\]' Cargo.toml; then
        # Empty members array, add first member
        sed -i "s/members = \[\]/members = [\"$day_name\"]/" Cargo.toml
    elif grep -q "\"$day_name\"" Cargo.toml; then
        # Already in members
        :
    else
        # Append to members array
        sed -i "s/members = \[/members = [\"$day_name\", /" Cargo.toml
    fi

    echo "Rust: Created $day_dir"
    echo "  Run with: cargo run --bin $day_name < $day_name/input/input.txt"
}

setup_ocaml() {
    local ocaml_dir="$SCRIPT_DIR/ocaml"
    local day_dir="$ocaml_dir/$day_name"

    if [[ -d "$day_dir" ]]; then
        echo "OCaml: $day_name already exists, skipping"
        return
    fi

    echo "Setting up OCaml $day_name..."

    # Create the day directory structure
    mkdir -p "$day_dir/input"

    # Copy template files
    cp "$ocaml_dir/template/main.ml" "$day_dir/main.ml"
    sed "s/DAY_NAME/$day_name/" "$ocaml_dir/template/dune.tmpl" > "$day_dir/dune"

    # Create empty input files
    touch "$day_dir/input/input.txt"
    touch "$day_dir/input/sample.txt"

    echo "OCaml: Created $day_dir"
    echo "  Build with: dune build"
    echo "  Run with: dune exec ./$day_name/main.exe < $day_name/input/input.txt"
}

# Execute based on language selection
case "$lang" in
    rust)
        setup_rust
        ;;
    ocaml)
        setup_ocaml
        ;;
    both)
        setup_rust
        setup_ocaml
        ;;
esac

echo ""
echo "Done! Happy coding on Day $day_num!"
