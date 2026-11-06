set -euxo pipefail
shopt -s dotglob
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "${SCRIPT_DIR}"
PROJECT_DIR=$( git rev-parse --show-toplevel )
cd "${PROJECT_DIR}/web"

# Build the app
BASE_PATH="/void-stranger-tools" npm run build

# Remove the old build
PAGE_PATH="${PROJECT_DIR}/web/gh-pages/"
rm -rf "$PAGE_PATH"
# Recreate the worktree
git worktree prune
git worktree add "$PAGE_PATH" gh-pages
# Copy over the new build
cp -r "build"/* "$PAGE_PATH"

# Commit, but don't push
pushd "${PAGE_PATH}"
git add .
date=`date '+%F %H:%M:%S'`
git commit -m "Publish $date" || $(exit 0)
popd

set +x
echo -e "\n\x1B[32mApp built! Don't forget to \`git push\` the /web/gh-pages directory once you're satisfied with your changes\x1B[0m"
