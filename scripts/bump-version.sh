#!/bin/bash
set -e

# Version bump script for protoc-gen-rust-aip
# Usage: ./scripts/bump-version.sh [major|minor|patch] [version]
#   - bump-version.sh patch         # Bumps patch version (0.1.0 -> 0.1.1)
#   - bump-version.sh minor         # Bumps minor version (0.1.0 -> 0.2.0)
#   - bump-version.sh major         # Bumps major version (0.1.0 -> 1.0.0)
#   - bump-version.sh set 0.2.5     # Sets specific version

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

usage() {
    echo "Usage: $0 [major|minor|patch|set] [version]"
    echo ""
    echo "Examples:"
    echo "  $0 patch         # 0.1.0 -> 0.1.1"
    echo "  $0 minor         # 0.1.0 -> 0.2.0"
    echo "  $0 major         # 0.1.0 -> 1.0.0"
    echo "  $0 set 0.2.5     # Set to specific version"
    echo ""
    exit 1
}

log() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Get current version from main binary crate
get_current_version() {
    grep '^version = ' "$REPO_ROOT/crates/protoc-gen-rust-aip/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/'
}

# Bump version based on type
bump_version() {
    local current="$1"
    local bump_type="$2"
    
    # Parse semantic version
    if [[ ! "$current" =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
        error "Invalid version format: $current"
    fi
    
    local major="${BASH_REMATCH[1]}"
    local minor="${BASH_REMATCH[2]}"
    local patch="${BASH_REMATCH[3]}"
    
    case "$bump_type" in
        "major")
            echo "$((major + 1)).0.0"
            ;;
        "minor")
            echo "${major}.$((minor + 1)).0"
            ;;
        "patch")
            echo "${major}.${minor}.$((patch + 1))"
            ;;
        *)
            error "Invalid bump type: $bump_type"
            ;;
    esac
}

# Update version in a Cargo.toml file
update_cargo_toml() {
    local file="$1"
    local old_version="$2"
    local new_version="$3"
    
    if [[ ! -f "$file" ]]; then
        error "File not found: $file"
    fi
    
    # Update the package version
    sed -i.bak "s/^version = \"$old_version\"/version = \"$new_version\"/" "$file"
    
    # Update any local dependency versions
    sed -i.bak "s/version = \"$old_version\", path = \"/version = \"$new_version\", path = \"/" "$file"
    
    # Remove backup file
    rm -f "$file.bak"
    
    log "Updated $file: $old_version -> $new_version"
}

# Check if working directory is clean
check_git_status() {
    if [ -n "$(git status --porcelain)" ]; then
        error "Working directory is not clean. Please commit or stash changes first."
    fi
}

# Validate version format
validate_version() {
    local version="$1"
    if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        error "Invalid version format: $version (expected: x.y.z)"
    fi
}

# Main execution
main() {
    cd "$REPO_ROOT"
    
    # Parse arguments
    if [ $# -eq 0 ]; then
        usage
    fi
    
    local bump_type="$1"
    local new_version=""
    
    if [ "$bump_type" = "set" ]; then
        if [ $# -ne 2 ]; then
            error "Set command requires a version argument"
        fi
        new_version="$2"
        validate_version "$new_version"
    elif [[ "$bump_type" =~ ^(major|minor|patch)$ ]]; then
        local current_version
        current_version=$(get_current_version)
        new_version=$(bump_version "$current_version" "$bump_type")
        log "Current version: $current_version"
    else
        usage
    fi
    
    log "New version: $new_version"
    
    # Check git status
    check_git_status
    
    # Get current version for replacement
    local current_version
    current_version=$(get_current_version)
    
    # Confirm with user
    echo -e "${BLUE}About to update version from ${current_version} to ${new_version}${NC}"
    echo "This will update the following files:"
    echo "  - crates/protoc-gen-rust-aip/Cargo.toml"
    echo "  - crates/resource-codegen/Cargo.toml"
    echo "  - crates/resource-types/Cargo.toml"
    echo ""
    read -p "Continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log "Aborted"
        exit 0
    fi
    
    # Update all Cargo.toml files
    log "Updating Cargo.toml files..."
    
    update_cargo_toml "$REPO_ROOT/crates/protoc-gen-rust-aip/Cargo.toml" "$current_version" "$new_version"
    update_cargo_toml "$REPO_ROOT/crates/resource-codegen/Cargo.toml" "$current_version" "$new_version"
    update_cargo_toml "$REPO_ROOT/crates/resource-types/Cargo.toml" "$current_version" "$new_version"
    
    # Verify the changes work
    log "Verifying changes..."
    if ! cargo check --quiet; then
        error "Cargo check failed. Please fix the issues before proceeding."
    fi
    
    # Create git commit
    log "Creating git commit..."
    git add crates/*/Cargo.toml
    git commit -m "chore: bump version to v$new_version"
    
    # Create git tag
    log "Creating git tag..."
    git tag -a "v$new_version" -m "Release v$new_version"
    
    echo ""
    log "Version bumped successfully!"
    echo -e "${GREEN}✅ Version: $current_version -> $new_version${NC}"
    echo -e "${GREEN}✅ Git commit created${NC}"
    echo -e "${GREEN}✅ Git tag v$new_version created${NC}"
    echo ""
    echo -e "${BLUE}Next steps:${NC}"
    echo "  1. Review the changes: git show HEAD"
    echo "  2. Push the changes: git push origin main"
    echo "  3. Push the tag: git push origin v$new_version"
    echo "  4. Create a GitHub release (will trigger CI)"
    echo ""
    echo -e "${YELLOW}Note: The release workflow will automatically build and publish binaries.${NC}"
}

# Run main function
main "$@"
