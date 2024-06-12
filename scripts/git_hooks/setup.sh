#!/usr/bin/env bash
set -e
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Using a symbolic link to allow the script to be seemlessly updated without
# manual intervention.
# Using a relative path to allow the user to copy the entire repo directory
# without affecting the git hooks execution.
rm -f "$(git rev-parse --show-toplevel)/.git/hooks/pre-commit"
ln -s "../../scripts/git_hooks/pre-commit" "$(git rev-parse --show-toplevel)/.git/hooks/pre-commit"

echo "Git hooks successfuly set up"
