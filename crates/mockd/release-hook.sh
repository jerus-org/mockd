#!/bin/bash
set -exo pipefail

# Release hook for mockd crate - generates CHANGELOG.md
# Called by cargo release via pre-release-hook

NAME="CHANGELOG.md"
PACKAGE="mockd"
REPO_DIR="../.."

# NEW_VERSION is set by cargo release
VERSION="${NEW_VERSION:-${1}}"

if [[ -z "${VERSION}" ]]; then
    echo "Error: No version specified (set NEW_VERSION or pass as argument)" >&2
    exit 1
fi

gen-changelog generate \
    --display-summaries \
    --name "${NAME}" \
    --package "${PACKAGE}" \
    --repository-dir "${REPO_DIR}" \
    --next-version "${VERSION}"

echo "Generated ${NAME} for ${PACKAGE}@${VERSION}"
