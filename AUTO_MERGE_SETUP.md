# Auto-Merge CI Setup Instructions

## Overview

I've created an automated CI workflow that will:
- ✅ Run tests (`make test`)
- ✅ Run linting (`make lint`)
- ✅ Run type checking (`make check`)
- ✅ Auto-approve PRs when all checks pass
- ✅ Auto-merge PRs after approval

## Files Created/Modified

### 1. `docs/workflows-to-add/auto-merge.yml` (NEW - Ready to Install)
This workflow automatically approves and merges PRs when the PR Checks workflow completes successfully.

**Features:**
- Triggers after "PR Checks" workflow completes
- Only runs if all checks passed
- Auto-approves with a descriptive comment
- Checks if PR is mergeable (no conflicts)
- Uses squash merge method
- Provides clear success/failure messages

**To Install:** Copy or move this file to `.github/workflows/auto-merge.yml`

### 2. `docs/workflows-to-add/pr-checks.yml` (UPDATED - Ready to Install)
Added `make test` step to ensure comprehensive validation before merge.

**Now runs:**
1. `make test` - Runs Rust and frontend tests
2. `make check` - Type checking
3. `make lint` - Linting

**To Install:** Copy or move this file to `.github/workflows/pr-checks.yml`

## Installation Instructions

Due to GitHub security restrictions, workflow files cannot be pushed automatically. The workflow files are ready in `docs/workflows-to-add/` - you just need to install them:

### Quick Install (Recommended)
```bash
# Copy the workflow files to the correct location
cp docs/workflows-to-add/auto-merge.yml .github/workflows/
cp docs/workflows-to-add/pr-checks.yml .github/workflows/

# Commit and push
git add .github/workflows/
git commit -m "feat: add auto-merge workflow and update PR checks"
git push origin main  # or your target branch
```

### Option 1: Via GitHub Web Interface
1. Navigate to `.github/workflows/` in your repository
2. Click "Add file" → "Upload files"
3. Upload both files from `docs/workflows-to-add/`
4. Commit directly to main branch

### Option 2: Manual File Creation
1. Copy contents from `docs/workflows-to-add/auto-merge.yml`
2. Create `.github/workflows/auto-merge.yml` in your repo
3. Copy contents from `docs/workflows-to-add/pr-checks.yml`
4. Update `.github/workflows/pr-checks.yml` in your repo

### Option 3: Merge This PR and Install
1. This branch contains the workflow files in `docs/workflows-to-add/`
2. Merge this PR to get the documentation and files
3. Then run the Quick Install commands above

## Testing the Workflow

After the workflow is added to the main branch:

1. Create a test PR
2. The "PR Checks" workflow will run automatically
3. If all checks pass, the "Auto Approve and Merge" workflow will trigger
4. The PR will be approved and merged automatically

## Workflow Permissions

The auto-merge workflow requires these permissions (already configured):
- `contents: write` - To merge PRs
- `pull-requests: write` - To approve PRs
- `checks: read` - To read check status

## Customization

You can customize the workflow by editing `.github/workflows/auto-merge.yml`:

- **Merge method**: Change `merge_method: 'squash'` to `'merge'` or `'rebase'`
- **Approval message**: Edit the `body` field in the approval step
- **Branch restrictions**: Modify the `branches` filter in the trigger

## Current Status

✅ Workflows created and committed locally
⚠️ Push blocked due to GitHub App workflow permissions
📋 Manual setup required to activate the workflow

## Next Steps

1. Review the workflow files in this branch
2. Manually add them to your repository
3. Test with a sample PR
4. Enjoy automated PR approvals and merges!
