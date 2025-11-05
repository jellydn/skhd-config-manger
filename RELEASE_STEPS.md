# Steps to Merge PR and Create Beta Release

## Current Status

✅ Branch pushed: `claude/setup-ci-auto-approve-011CUpwznde3Bdz1iEx19ACn`
✅ Changes committed and ready
📋 Current version: 0.5.0
🎯 Target beta version: 0.6.0-beta.1

## Step 1: Create and Merge PR

### Option A: Via GitHub CLI
```bash
gh pr create \
  --title "feat: Add auto-merge CI workflow for PRs" \
  --body-file PR_DETAILS.md \
  --head claude/setup-ci-auto-approve-011CUpwznde3Bdz1iEx19ACn
```

### Option B: Via GitHub Web Interface
Visit: https://github.com/jellydn/skhd-config-manger/pull/new/claude/setup-ci-auto-approve-011CUpwznde3Bdz1iEx19ACn

Copy PR details from `PR_DETAILS.md`

### Review and Merge
1. Review the changes (workflow files in docs/workflows-to-add/)
2. Approve the PR
3. Merge using "Squash and merge"

## Step 2: Install Workflow Files (Post-Merge)

After merging, install the workflow files:

```bash
# Switch to main/master branch
git checkout main  # or master
git pull

# Copy workflow files to correct location
cp docs/workflows-to-add/auto-merge.yml .github/workflows/
cp docs/workflows-to-add/pr-checks.yml .github/workflows/

# Commit and push
git add .github/workflows/
git commit -m "feat: activate auto-merge workflow and update PR checks"
git push origin main
```

## Step 3: Create Beta Release (0.6.0-beta.1)

### Update Version Numbers

```bash
# Update package.json
sed -i 's/"version": "0.5.0"/"version": "0.6.0-beta.1"/' package.json

# Update Cargo.toml
sed -i 's/version = "0.5.0"/version = "0.6.0-beta.1"/' src-tauri/Cargo.toml

# Update Cargo.lock
cd src-tauri && cargo update -p keybinder && cd ..
```

### Commit Version Bump

```bash
git add package.json src-tauri/Cargo.toml src-tauri/Cargo.lock
git commit -m "chore: bump version to 0.6.0-beta.1"
git push origin main
```

### Create Git Tag and Release

```bash
# Create and push tag
git tag v0.6.0-beta.1
git push origin v0.6.0-beta.1
```

### Create GitHub Release

```bash
gh release create v0.6.0-beta.1 \
  --title "v0.6.0-beta.1" \
  --notes "## 🚀 Beta Release: Auto-Merge CI Workflow

This beta release includes the auto-merge CI workflow setup.

### 🆕 New Features
- Auto-merge workflow files for CI/CD automation
- Complete documentation for workflow installation
- Enhanced PR checks with comprehensive testing

### 📦 Installation
See \`AUTO_MERGE_SETUP.md\` for workflow installation instructions.

### ⚠️ Beta Notice
This is a beta release. The auto-merge workflows need to be manually installed from \`docs/workflows-to-add/\`.

### 📝 Full Changelog
See commit history for detailed changes." \
  --prerelease
```

## Alternative: Using Make Commands

If the Makefile supports versioning:

```bash
# Check if make bump command exists
make help | grep bump

# If available, use:
make bump VERSION=0.6.0-beta.1
```

## Verification

After release, verify:

```bash
# Check version
make version

# Should show: 0.6.0-beta.1

# Check tag exists
git tag -l | grep 0.6.0-beta.1

# Check GitHub release
gh release view v0.6.0-beta.1
```

## Summary

1. ✅ PR created for auto-merge workflow
2. ⏳ Merge PR
3. ⏳ Install workflow files
4. ⏳ Bump version to 0.6.0-beta.1
5. ⏳ Create beta release

---

All files and documentation are ready. Execute the steps above when ready to proceed!
