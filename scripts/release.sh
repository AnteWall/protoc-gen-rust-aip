#!/bin/bash
set -e

# Release script for protoc-gen-rust-aip
# This script helps create releases by running checks and pushing tags
# Usage: ./scripts/release.sh [--dry-run] [tag]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

DRY_RUN=false

usage() {
    echo "Usage: $0 [--dry-run] [tag]"
    echo ""
    echo "Examples:"
    echo "  $0                    # Release current HEAD tag"
    echo "  $0 v0.1.1             # Release specific tag"
    echo "  $0 --dry-run          # Show what would be done"
    echo ""
    echo "Note: Use bump-version.sh to create version and tag first"
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

dry_run() {
    echo -e "${BLUE}[DRY RUN]${NC} $1"
}

# Get the latest tag
get_latest_tag() {
    git describe --tags --abbrev=0 2>/dev/null || echo ""
}

# Check if tag exists
tag_exists() {
    git tag -l "$1" | grep -q "^$1$"
}

# Run pre-release checks
run_checks() {
    log "Running pre-release checks..."
    
    # Check if we're on main branch
    local current_branch
    current_branch=$(git branch --show-current)
    if [ "$current_branch" != "main" ] && [ "$current_branch" != "master" ]; then
        warn "You're not on main/master branch (current: $current_branch)"
        read -p "Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            error "Aborted"
        fi
    fi
    
    # Check if working directory is clean
    if [ -n "$(git status --porcelain)" ]; then
        error "Working directory is not clean. Please commit changes first."
    fi
    
    # Check if we have any tags
    if [ -z "$(get_latest_tag)" ]; then
        error "No tags found. Use bump-version.sh to create a version first."
    fi
    
    # Run tests
    log "Running tests..."
    if ! cargo test --quiet; then
        error "Tests failed. Please fix before releasing."
    fi
    
    # Check formatting
    log "Checking code formatting..."
    if ! cargo fmt --check; then
        error "Code is not formatted. Run 'cargo fmt' first."
    fi
    
    # Check for clippy warnings
    log "Running clippy..."
    if ! cargo clippy --all-targets --all-features -- -D warnings; then
        error "Clippy found issues. Please fix before releasing."
    fi
    
    log "All checks passed ✅"
}

# Show release information
show_release_info() {
    local tag="$1"
    local version="${tag#v}"
    
    echo ""
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}                       RELEASE SUMMARY                          ${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}Tag:${NC}           $tag"
    echo -e "${GREEN}Version:${NC}       $version"
    echo -e "${GREEN}Branch:${NC}        $(git branch --show-current)"
    echo -e "${GREEN}Commit:${NC}        $(git rev-parse --short HEAD)"
    echo -e "${GREEN}Packages:${NC}      protoc-gen-rust-aip, resource-codegen, resource-types"
    echo ""
    
    # Show recent commits since last tag
    local prev_tag
    prev_tag=$(git describe --tags --abbrev=0 HEAD~1 2>/dev/null || echo "")
    if [ -n "$prev_tag" ]; then
        echo -e "${GREEN}Changes since $prev_tag:${NC}"
        git log --oneline --no-merges "${prev_tag}..HEAD" | head -10
        echo ""
    fi
    
    echo -e "${GREEN}Release artifacts will include:${NC}"
    echo "  • Linux x86_64 binary"
    echo "  • macOS x86_64 binary" 
    echo "  • macOS aarch64 binary"
    echo "  • Windows x86_64 binary"
    echo "  • Source code (zip/tar.gz)"
    echo ""
    echo -e "${GREEN}Installation methods:${NC}"
    echo "  • cargo install protoc-gen-rust-aip"
    echo "  • curl install script"
    echo "  • Direct binary download"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
}

# Push release
push_release() {
    local tag="$1"
    
    if [ "$DRY_RUN" = true ]; then
        dry_run "Would push main branch"
        dry_run "Would push tag $tag"
        dry_run "Would trigger GitHub Actions release workflow"
        return
    fi
    
    log "Pushing main branch..."
    git push origin main
    
    log "Pushing tag $tag..."
    git push origin "$tag"
    
    log "Release pushed successfully!"
    echo ""
    echo -e "${GREEN}✅ Tag $tag pushed to GitHub${NC}"
    echo -e "${GREEN}✅ GitHub Actions will build release artifacts${NC}"
    echo ""
    echo -e "${BLUE}Next steps:${NC}"
    echo "  1. Monitor GitHub Actions: https://github.com/protoc-gen-rust-aip/protoc-gen-rust-aip/actions"
    echo "  2. Edit release notes: https://github.com/protoc-gen-rust-aip/protoc-gen-rust-aip/releases"
    echo "  3. Test installation methods once binaries are ready"
    echo ""
    echo -e "${YELLOW}The release will be available at:${NC}"
    echo "  https://github.com/protoc-gen-rust-aip/protoc-gen-rust-aip/releases/tag/$tag"
}

# Main execution
main() {
    cd "$REPO_ROOT"
    
    # Parse arguments
    local tag=""
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            -h|--help)
                usage
                ;;
            *)
                if [ -n "$tag" ]; then
                    error "Too many arguments"
                fi
                tag="$1"
                shift
                ;;
        esac
    done
    
    # If no tag specified, use the latest tag
    if [ -z "$tag" ]; then
        tag=$(get_latest_tag)
        if [ -z "$tag" ]; then
            error "No tags found and no tag specified"
        fi
        log "Using latest tag: $tag"
    fi
    
    # Validate tag format
    if [[ ! "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        error "Invalid tag format: $tag (expected: vX.Y.Z)"
    fi
    
    # Check if tag exists
    if ! tag_exists "$tag"; then
        if [ "$DRY_RUN" = true ]; then
            warn "Tag $tag does not exist yet (would be created by bump-version.sh)"
        else
            error "Tag $tag does not exist. Use bump-version.sh to create it first."
        fi
    fi
    
    # Show what we're about to do
    show_release_info "$tag"
    
    # Confirm with user (unless dry run)
    if [ "$DRY_RUN" = false ]; then
        read -p "Continue with release? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log "Aborted"
            exit 0
        fi
    fi
    
    # Run checks
    if [ "$DRY_RUN" = false ]; then
        run_checks
    else
        dry_run "Would run pre-release checks"
    fi
    
    # Push the release
    push_release "$tag"
}

# Run main function
main "$@"
