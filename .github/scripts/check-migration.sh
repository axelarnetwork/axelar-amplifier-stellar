#!/bin/bash
set -euo pipefail

main() {
    # Get all changed files in the PR
    if ! CHANGED_FILES=$(git diff --name-only "$1" "$2" | grep "contracts/[^/]*/src/testdata/ensure_.*_storage_schema_is_unchanged.golden"); then
        echo "No storage schema changes detected"
        exit 0
    fi

    echo "Found changed storage schema files:"
    echo "$CHANGED_FILES"

    # Check each changed storage schema file
    while IFS= read -r file; do
        [ -z "$file" ] && continue

        # Check if the file still exists (not deleted)
        if [ ! -f "$file" ]; then
            echo "File $file has been deleted, skipping migration checks"
            continue
        fi

        check_migration_file "$file"
    done <<< "$CHANGED_FILES"
}

check_migration_file() {
    local file="$1"

    local contract_dir
    contract_dir=$(contract_dir "$file")

    if ! is_upgradable "$contract_dir"; then
        echo "Contract in $contract_dir is not upgradable, skipping migration checks"
        exit 0
    fi

    local migrate_file
    migrate_file=$(migrate_file "$contract_dir" "$file")

    local test_file
    test_file=$(test_file "$contract_dir" "$migrate_file")

    verify_legacy_storage_module "$migrate_file"

    echo "✓ Found valid migration file for $file"
}

contract_dir() {
    local file="$1"
    local contract_dir

    contract_dir=$(echo "$file" | grep -o 'contracts/stellar-[^/]*')
    [ -z "$contract_dir" ] && print_error "Could not determine contract directory for $file"

    echo "$contract_dir"
}

is_upgradable() {
    local contract_dir="$1"
    local contract_file
    contract_file=$(find "$contract_dir/src" -name "contract.rs" -not -path "*/test*" | head -n 1)
    [ -z "$contract_file" ] && print_error "Could not find contract.rs in $contract_dir/src/"

    if grep -q "#\[derive(.*Upgradable.*)\]" "$contract_file"; then
        return 0
    fi

    return 1
}

migrate_file() {
    local contract_dir="$1"
    local file="$2"
    local migrate_file

    migrate_file=$(find "$contract_dir/src" -name "migrate.rs" -not -path "*/test*" | head -n 1)
    [ -z "$migrate_file" ] && print_error "Storage schema change detected in $file but no migrate.rs found under $contract_dir/src/
Please create a migration file at $contract_dir/src/migrate.rs to handle the schema changes"

    echo "$migrate_file"
}

test_file() {
    local contract_dir="$1"
    local migrate_file="$2"
    local test_file

    test_file=$(find "$contract_dir" -path "*/test*/migrate.rs" -o -path "*/tests/migrate.rs" | head -n 1)
    [ -z "$test_file" ] && print_error "migrate.rs found at $migrate_file but no corresponding migrate.rs test file found
Please create a test file at $contract_dir/tests/migrate.rs to test the schema migration"

    echo "$test_file"
}

verify_legacy_storage_module() {
    local migrate_file="$1"

    grep -q "mod legacy_storage" "$migrate_file" || print_error "migrate.rs found at $migrate_file but missing required 'legacy_storage' module declaration
Please add a 'legacy_storage' module to handle the schema migration"
}

print_error() {
    echo "Error: $1"
    exit 1
}

main "$@";
