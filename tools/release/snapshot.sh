#!/bin/zsh
# Writes the public tree for a release to $1 (default dist/public): the committed HEAD minus
# $SNAPSHOT_DROP and acorn-release.env, with the workspace pointed at crates/* only. Refuses
# if anything matches $SNAPSHOT_DENY other than the strings in $SNAPSHOT_ALLOW. Used by
# release/publish.sh.
set -e
source ./acorn-app.env
source ./acorn-release.env
OUT=${1:-dist/public}
rm -rf $OUT && mkdir -p $OUT
git archive HEAD | tar -x -C $OUT
for d in ${=SNAPSHOT_DROP} acorn-release.env; do rm -rf $OUT/$d; done
# Workspace members: the crates only.
sed -i '' -E 's|^members = \[.*\]|members = ["crates/*"]|' $OUT/Cargo.toml
grep -q '^members = \["crates/\*"\]' $OUT/Cargo.toml || { echo "snapshot: couldn't rewrite workspace members" >&2; exit 1; }
# Nothing personal or secret may slip through.
ALLOW_SED=""
for a in ${=SNAPSHOT_ALLOW}; do ALLOW_SED="$ALLOW_SED; s#${a//./\\.}##g"; done
if grep -rIniE "$SNAPSHOT_DENY" $OUT --exclude=Cargo.lock | sed -E "${ALLOW_SED#; }" | grep -iE ":[0-9]+:.*($SNAPSHOT_DENY)"; then
  echo "snapshot: personal or secret strings found (above); not publishing" >&2; exit 1
fi
echo $OUT
