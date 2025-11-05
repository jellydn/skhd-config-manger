# Final Steps to Complete Beta Release

## ✅ Completed

- [x] Created auto-merge workflow files (in `docs/workflows-to-add/`)
- [x] Updated PR checks workflow to include tests
- [x] Created comprehensive documentation
- [x] Bumped version to 0.6.0-beta.1
- [x] Committed all changes to branch `claude/setup-ci-auto-approve-011CUpwznde3Bdz1iEx19ACn`
- [x] Pushed all commits to remote
- [x] Created release notes (`RELEASE_NOTES_v0.6.0-beta.1.md`)

## 📋 Manual Steps Required

### Step 1: Create Pull Request

Since `gh pr create` requires manual approval, create the PR manually:

**Option A: Via GitHub Web Interface**
Visit: https://github.com/jellydn/skhd-config-manger/pull/new/claude/setup-ci-auto-approve-011CUpwznde3Bdz1iEx19ACn

Use the content from `PR_DETAILS.md` for the PR description.

**Option B: Via CLI (from your terminal)**
```bash
gh pr create \
  --title "feat: Add auto-merge CI workflow for PRs (v0.6.0-beta.1)" \
  --body-file PR_DETAILS.md \
  --head claude/setup-ci-auto-approve-011CUpwznde3Bdz1iEx19ACn
```

### Step 2: Merge the Pull Request

1. Review the PR changes
2. Approve the PR
3. Merge using "Squash and merge"

### Step 3: Create GitHub Release

After merging, create the beta release with the tag:

```bash
# Ensure you're on main/master branch
git checkout main  # or master
git pull

# Create and push the tag
git tag v0.6.0-beta.1 -m "Beta Release v0.6.0-beta.1: Auto-Merge CI Workflow"
git push origin v0.6.0-beta.1

# Create GitHub release
gh release create v0.6.0-beta.1 \
  --title "v0.6.0-beta.1 (Beta)" \
  --notes-file RELEASE_NOTES_v0.6.0-beta.1.md \
  --prerelease
```

### Step 4: Install Auto-Merge Workflows (Optional)

After the release, if you want to activate the auto-merge workflows:

```bash
# Copy workflow files to active location
cp docs/workflows-to-add/auto-merge.yml .github/workflows/
cp docs/workflows-to-add/pr-checks.yml .github/workflows/

# Commit and push
git add .github/workflows/
git commit -m "feat: activate auto-merge workflow and update PR checks"
git push origin main
```

## 📊 Current Status

- **Branch**: `claude/setup-ci-auto-approve-011CUpwznde3Bdz1iEx19ACn`
- **Version**: `0.6.0-beta.1`
- **Commits pushed**: 5 commits
- **Tag created**: `v0.6.0-beta.1` (local only, needs to be pushed after merge)

## 📝 Summary of Changes

```
74a6a89 docs: add release notes for v0.6.0-beta.1
cf0baf2 chore: bump version to 0.6.0-beta.1
8c9eef6 docs: update installation instructions with clearer steps
e4068d9 docs: add auto-merge workflow setup guide and files
```

## 📚 Documentation Files

All documentation is ready:
- `AUTO_MERGE_SETUP.md` - Workflow setup guide
- `PR_DETAILS.md` - PR description content
- `RELEASE_STEPS.md` - Detailed release process
- `RELEASE_NOTES_v0.6.0-beta.1.md` - Release notes for GitHub
- `FINAL_STEPS.md` - This file

## 🚀 Quick Commands Summary

```bash
# 1. Create PR (visit URL or use gh CLI)
open https://github.com/jellydn/skhd-config-manger/pull/new/claude/setup-ci-auto-approve-011CUpwznde3Bdz1iEx19ACn

# 2. After merging, create release tag
git checkout main && git pull
git tag v0.6.0-beta.1 -m "Beta Release v0.6.0-beta.1"
git push origin v0.6.0-beta.1

# 3. Create GitHub release
gh release create v0.6.0-beta.1 \
  --title "v0.6.0-beta.1 (Beta)" \
  --notes-file RELEASE_NOTES_v0.6.0-beta.1.md \
  --prerelease

# 4. (Optional) Install workflows
cp docs/workflows-to-add/*.yml .github/workflows/
git add .github/workflows/
git commit -m "feat: activate auto-merge workflow"
git push
```

---

**Next Action**: Create the PR using the link or command above, then proceed with merge and release!
