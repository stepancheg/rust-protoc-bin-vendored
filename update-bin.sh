#!/usr/bin/env bash

# Update bundled `protoc` binary to the latest version.

set -ex

cd "$(dirname "$0")"

tag_name=$(curl -s https://api.github.com/repos/protocolbuffers/protobuf/releases/latest | grep tag_name | cut -d '"' -f 4)
echo "$tag_name" >protoc-bin-vendored/version.txt
echo "updating protoc binaries to version $tag_name" >&2

update_arch() {
    arch="$1"
    TMPFILE=$(mktemp)
    url="https://github.com/protocolbuffers/protobuf/releases/download/${tag_name}/protoc-${tag_name#v}-${arch}.zip"
    echo "downloading $url..." >&2
    curl -sL "$url" --output "${TMPFILE}.zip"
    if [[ $arch == "win32" ]]; then
        unzip -p "${TMPFILE}.zip" bin/protoc.exe >"protoc-bin-vendored/bin/protoc-${arch}.exe"
    else
        unzip -p "${TMPFILE}.zip" bin/protoc >"protoc-bin-vendored/bin/protoc-${arch}"
    fi
    if [[ $arch == "linux-x86_64" ]]; then
        # Check we are in correct directory
        test -e protoc-bin-vendored/README.md
        (
            cd protoc-bin-vendored
            rm -rf include
            unzip "${TMPFILE}.zip" "include/*" -d .
        )
    fi
    rm "${TMPFILE}.zip"
}

update_arch "linux-aarch_64"
update_arch "linux-ppcle_64"
update_arch "linux-x86_32"
update_arch "linux-x86_64"
update_arch "osx-x86_64"
update_arch "win32"
