#!/bin/zsh
# Publishes the current HEAD to the app's public GitHub repo ($PUBLIC_REPO) as source, with no
# private history.
#   first run: creates the repo with one commit
#   later runs: opens a PR "Release vX" with one commit
# Merge release PRs with "Squash and merge" so main stays one commit per release.
set -e
source ./acorn-app.env
source ./acorn-release.env
TOOLS=${0:A:h}
VERSION=$(awk -F\" '/^version/ {print $2; exit}' Cargo.toml)
[[ -z $(git status --porcelain) ]] || { echo "commit or stash first: the snapshot is taken from HEAD" >&2; exit 1; }
SNAP=$($TOOLS/snapshot.sh dist/public)
WORK=dist/public-repo
rm -rf $WORK
if gh repo view $PUBLIC_REPO >/dev/null 2>&1; then
  # Each release replaces the public tree with the private snapshot, so anything merged
  # straight into public main since the last release would be lost: bring it in first.
  LAST=$(gh release view --repo $PUBLIC_REPO --json tagName -q .tagName 2>/dev/null || true)
  if [[ -n $LAST ]]; then
    AHEAD=$(gh api repos/$PUBLIC_REPO/compare/$LAST...main -q .ahead_by)
    if [[ $AHEAD != 0 ]]; then
      echo "public main has $AHEAD commit(s) since $LAST that aren't in a release:" >&2
      gh api repos/$PUBLIC_REPO/compare/$LAST...main -q '.commits[] | "  " + .sha[0:7] + " " + .commit.author.name + ": " + (.commit.message | split("\n")[0])' >&2
      echo "apply them to the private repo, then tag or reset public main, before publishing" >&2
      exit 1
    fi
  fi
  gh repo clone $PUBLIC_REPO $WORK -- --quiet
  cd $WORK
  BRANCH=release-v$VERSION
  git checkout -q -b $BRANCH
  git rm -rq . >/dev/null
  cp -R ../public/. .
  git add -A
  git commit -qm "Release v$VERSION"
  git push -q -u origin $BRANCH
  gh pr create --repo $PUBLIC_REPO --title "Release v$VERSION" --body "$APP_NAME v$VERSION source snapshot.

🤖 Generated with [Claude Code](https://claude.com/claude-code)" --head $BRANCH
else
  mkdir -p $WORK && cp -R $SNAP/. $WORK/ && cd $WORK
  git init -q -b main
  git add -A
  git commit -qm "$APP_NAME v$VERSION"
  gh repo create $PUBLIC_REPO --public --description "$PUBLIC_DESCRIPTION" --source . --push
fi
