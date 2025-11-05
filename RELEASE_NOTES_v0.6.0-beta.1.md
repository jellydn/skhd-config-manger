# Release Notes: v0.6.0-beta.1 (Beta)

## 🚀 Beta Release: Auto-Merge CI Workflow

This is a **beta release** introducing automated CI workflow for auto-approving and merging pull requests when all checks pass.

### 🆕 New Features

#### Auto-Merge CI Workflow
- **Auto-approval workflow** - Automatically approves PRs when all CI checks pass
- **Auto-merge functionality** - Automatically merges approved PRs using squash merge
- **Enhanced PR checks** - Now includes comprehensive testing (`make test`, `make check`, `make lint`)
- **Complete documentation** - Installation guide and customization options in `AUTO_MERGE_SETUP.md`

### 📦 What's Included

- **Workflow files** ready for installation in `docs/workflows-to-add/`:
  - `auto-merge.yml` - Auto-approval and merge workflow
  - `pr-checks.yml` - Enhanced PR validation with tests

- **Documentation**:
  - `AUTO_MERGE_SETUP.md` - Comprehensive setup guide
  - `PR_DETAILS.md` - PR creation details
  - `RELEASE_STEPS.md` - Release process documentation

### ⚙️ Installation Instructions

The auto-merge workflows need to be manually installed due to GitHub security restrictions:

```bash
# Copy workflow files to the correct location
cp docs/workflows-to-add/auto-merge.yml .github/workflows/
cp docs/workflows-to-add/pr-checks.yml .github/workflows/

# Commit and push
git add .github/workflows/
git commit -m "feat: activate auto-merge workflow and update PR checks"
git push
```

See `AUTO_MERGE_SETUP.md` for detailed instructions and customization options.

### 🔄 How It Works

1. PR is created or updated
2. PR Checks workflow runs:
   - `make test` - Runs Rust and frontend tests
   - `make check` - Type checking
   - `make lint` - Linting
3. If all checks pass → Auto-merge workflow triggers
4. PR is automatically approved with a descriptive comment
5. PR is squash merged automatically (if no conflicts)

### ✨ Benefits

- ✅ **Automated approval** - No manual review needed for passing PRs
- ✅ **Comprehensive validation** - Tests, type checking, and linting must all pass
- ✅ **Clean history** - Squash merge keeps git history clean
- ✅ **Conflict detection** - Won't merge if conflicts exist
- ✅ **Reduced overhead** - Saves time on routine PR reviews

### ⚠️ Beta Notice

This is a **beta release** for testing purposes:

- The auto-merge workflows are ready but require manual installation
- Test the workflows thoroughly before relying on them for production
- Provide feedback on the workflow behavior and any issues encountered

### 📋 Changes Since v0.5.0

- Added auto-merge CI workflow files
- Updated PR checks to include comprehensive testing
- Added documentation for workflow setup and usage
- Version bumped to 0.6.0-beta.1

### 🔗 Pull Request

See the PR for this release: `claude/setup-ci-auto-approve-011CUpwznde3Bdz1iEx19ACn`

### 📝 Full Changelog

```
cf0baf2 chore: bump version to 0.6.0-beta.1
8c9eef6 docs: update installation instructions with clearer steps
e4068d9 docs: add auto-merge workflow setup guide and files
```

### 🐛 Known Issues

None reported for this beta release.

### 💬 Feedback

Please report any issues or provide feedback on:
- Workflow behavior
- Documentation clarity
- Installation process
- Any bugs or unexpected behavior

---

**Note**: This is a beta release. Test thoroughly before using in production environments.

For installation help, see `AUTO_MERGE_SETUP.md` or `RELEASE_STEPS.md`.
