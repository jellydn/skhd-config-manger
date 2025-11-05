# PR: Add Auto-Merge CI Workflow

## Title
feat: Add auto-merge CI workflow for PRs

## Branch
`claude/setup-ci-auto-approve-011CUpwznde3Bdz1iEx19ACn`

## Description

This PR sets up automated CI workflow for auto-approving and merging PRs when all checks pass.

### What's Included

- 📁 **Auto-merge workflow files** in `docs/workflows-to-add/`
  - `auto-merge.yml` - Automatically approves and merges PRs when checks pass
  - `pr-checks.yml` - Updated to include `make test` step

- 📖 **Complete documentation** (`AUTO_MERGE_SETUP.md`)
  - Installation instructions
  - Workflow features and customization
  - Testing guide

### Installation Required

Due to GitHub security restrictions, workflow files cannot be pushed directly. After merging this PR, install the workflows:

```bash
cp docs/workflows-to-add/auto-merge.yml .github/workflows/
cp docs/workflows-to-add/pr-checks.yml .github/workflows/
git add .github/workflows/
git commit -m "feat: add auto-merge workflow and update PR checks"
git push
```

### How It Works

1. PR created/updated
2. PR Checks workflow runs (`make test`, `make check`, `make lint`)
3. If all checks pass → Auto-approve workflow triggers
4. PR is automatically approved with comment
5. PR is squash merged automatically

### Benefits

✅ Automated PR approval when all tests pass
✅ Automated merging after approval
✅ Ensures comprehensive validation (test, check, lint)
✅ Keeps git history clean with squash merges
✅ Reduces manual review overhead for passing PRs

## Test Plan

- [x] Created workflow files in docs/workflows-to-add/
- [x] Added comprehensive documentation
- [x] Verified YAML syntax
- [ ] Install workflows after merge
- [ ] Test with a sample PR

## Create PR Command

Visit this URL or use the gh CLI:
https://github.com/jellydn/skhd-config-manger/pull/new/claude/setup-ci-auto-approve-011CUpwznde3Bdz1iEx19ACn
