#!/bin/bash
set -e

# Complete release workflow for protoc-gen-rust-aip
# Usage: ./scripts/make-release.sh [major|minor|patch] [--dry-run]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

usage() {
    echo "Usage: $0 [major|minor|patch] [--dry-run]"
    echo ""
    echo "Examples:"
    echo "  $0 patch         # Bump patch version and release"
    echo "  $0 minor         # Bump minor version and release"
    echo "  $0 major         # Bump major version and release"
    echo "  $0 patch --dry-run  # Show what would happen"
    echo ""
    echo "This script will:"
    echo "  1. Bump version in all Cargo.toml files"
    echo "  2. Create a git commit and tag"
    echo "  3. Push to GitHub and trigger release"
    exit 1
}

log() {
    echo -e "${GREEN}[RELEASE]${NC} $1"
}

info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

main() {
    # Parse arguments
    local bump_type=""
    local dry_run=""
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            major|minor|patch)
                if [ -n "$bump_type" ]; then
                    echo "Error: Multiple bump types specified"
                    usage
                fi
                bump_type="$1"
                shift
                ;;
            --dry-run)
                dry_run="--dry-run"
                shift
                ;;
            -h|--help)
                usage
                ;;
            *)
                echo "Error: Unknown argument $1"
                usage
                ;;
        esac
    done
    
    if [ -z "$bump_type" ]; then
        usage
    fi
    
    log "Starting release workflow: $bump_type version bump"
    
    if [ -n "$dry_run" ]; then
        info "DRY RUN MODE - No changes will be made"
        echo ""
    fi
    
    # Step 1: Bump version
    log "Step 1: Bumping version..."
    if [ -n "$dry_run" ]; then
        info "Would run: $SCRIPT_DIR/bump-version.sh $bump_type"
    else
        "$SCRIPT_DIR/bump-version.sh" "$bump_type"
    fi
    
    echo ""
    
    # Step 2: Release
    log "Step 2: Creating release..."
    if [ -n "$dry_run" ]; then
        # In dry run mode, we need to get the version that would be created
        current_version=$(grep '^version = ' "$SCRIPT_DIR/../crates/protoc-gen-rust-aip/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/')
        case "$bump_type" in
            "major")
                new_version="$((${current_version%%.*} + 1)).0.0"
                ;;
            "minor")
                major="${current_version%%.*}"
                minor="${current_version#*.}"
                minor="${minor%%.*}"
                new_version="${major}.$((minor + 1)).0"
                ;;
            "patch")
                major="${current_version%%.*}"
                rest="${current_version#*.}"
                minor="${rest%%.*}"
                patch="${rest#*.}"
                new_version="${major}.${minor}.$((patch + 1))"
                ;;
        esac
        "$SCRIPT_DIR/release.sh" --dry-run "v$new_version"
    else
        "$SCRIPT_DIR/release.sh"
    fi
    
    echo ""
    log "Release workflow complete! 🎉"
}

main "$@"
