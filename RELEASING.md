# Release Process

This document describes how to create releases for `protoc-gen-rust-aip`.

## Quick Release

For a simple release, use the all-in-one script:

```bash
# Patch release (0.1.0 -> 0.1.1)
./scripts/make-release.sh patch

# Minor release (0.1.0 -> 0.2.0)
./scripts/make-release.sh minor

# Major release (0.1.0 -> 1.0.0)
./scripts/make-release.sh major

# Dry run to see what would happen
./scripts/make-release.sh patch --dry-run
```

## Step-by-Step Release

For more control, use the individual scripts:

### 1. Bump Version

```bash
# Increment patch version (0.1.0 -> 0.1.1)
./scripts/bump-version.sh patch

# Increment minor version (0.1.0 -> 0.2.0)
./scripts/bump-version.sh minor

# Increment major version (0.1.0 -> 1.0.0)
./scripts/bump-version.sh major

# Set specific version
./scripts/bump-version.sh set 0.2.5
```

### 2. Create Release

```bash
# Release the latest tag
./scripts/release.sh

# Release a specific tag
./scripts/release.sh v0.1.1

# Dry run
./scripts/release.sh --dry-run
```

## What the Scripts Do

### `bump-version.sh`

1. **Updates all Cargo.toml files** with the new version:
   - `crates/protoc-gen-rust-aip/Cargo.toml`
   - `crates/resource-codegen/Cargo.toml`
   - `crates/resource-types/Cargo.toml`
2. **Updates internal dependencies** to use the new version
3. **Runs `cargo check`** to verify everything still compiles
4. **Creates a git commit** with the version changes
5. **Creates a git tag** (e.g., `v0.1.1`)

### `release.sh`

1. **Runs pre-release checks**:
   - Verifies working directory is clean
   - Runs tests (`cargo test`)
   - Checks code formatting (`cargo fmt --check`)
   - Runs clippy (`cargo clippy`)
2. **Shows release summary** with changes since last release
3. **Pushes to GitHub**:
   - Pushes the main branch
   - Pushes the git tag
4. **Triggers GitHub Actions** release workflow

### `make-release.sh`

Combines both scripts for a complete release workflow.

## GitHub Actions

When a tag is pushed, GitHub Actions automatically:

1. **Builds binaries** for multiple platforms:
   - Linux x86_64
   - macOS x86_64 and aarch64
   - Windows x86_64
2. **Creates a GitHub release** with:
   - Release notes
   - Binary downloads
   - Installation instructions
3. **Updates installation script** references

## Manual Steps After Release

1. **Monitor the GitHub Actions**: Check that all builds succeed
2. **Edit release notes**: Add changelog and notable features
3. **Test installation**: Verify the install script works
4. **Update documentation**: If needed for new features

## Version Strategy

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** version: Incompatible API changes
- **MINOR** version: New functionality (backward compatible)
- **PATCH** version: Bug fixes (backward compatible)

## Examples

### Patch Release (Bug Fix)

```bash
# Fix a bug, commit the fix
git add -A
git commit -m "fix: handle empty resource patterns correctly"

# Release patch version
./scripts/make-release.sh patch
```

### Minor Release (New Feature)

```bash
# Add a new feature, commit it
git add -A
git commit -m "feat: add support for collection IDs"

# Release minor version
./scripts/make-release.sh minor
```

### Major Release (Breaking Change)

```bash
# Make breaking changes, commit them
git add -A
git commit -m "feat!: change generated API structure"

# Release major version
./scripts/make-release.sh major
```

## Troubleshooting

### "Working directory is not clean"

Commit or stash your changes before releasing:

```bash
git add -A
git commit -m "your changes"
# or
git stash
```

### "Tests failed"

Fix the failing tests before releasing:

```bash
cargo test
# Fix issues, then:
./scripts/make-release.sh patch
```

### "Code is not formatted"

Format your code:

```bash
cargo fmt
git add -A
git commit -m "style: format code"
./scripts/make-release.sh patch
```

### "Clippy found issues"

Fix clippy warnings:

```bash
cargo clippy --all-targets --all-features -- -D warnings
# Fix issues, then continue with release
```

## Recovery

If something goes wrong during release:

### Delete a tag

```bash
# Delete local tag
git tag -d v0.1.1

# Delete remote tag (if already pushed)
git push origin --delete v0.1.1
```

### Revert version changes

```bash
# Reset to previous commit
git reset --hard HEAD~1

# If already pushed, you'll need to force push (be careful!)
git push --force-with-lease origin main
```
