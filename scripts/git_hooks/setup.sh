#!/usr/bin/env bash
set -e
GIT_ROOT="$(git rev-parse --show-toplevel)"

# Using a symbolic link to allow the script to be seamlessly updated without
# manual intervention.
# Using a relative path to allow the user to copy the entire repo directory
# without affecting the git hooks execution.
rm -f "$GIT_ROOT/.git/hooks/pre-commit"
ln -s "../../scripts/git_hooks/pre-commit" "$GIT_ROOT/.git/hooks/pre-commit"

echo "Git hooks successfully set up"
