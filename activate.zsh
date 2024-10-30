#!/bin/zsh
export RUSTDOCFLAGS="--html-in-header $(pwd)/docs/html/custom-header.html --document-private-items"
export TOPO_LOG=all=trace

export CARGO_REGISTRIES_CLOUDSMITH_TOKEN=$(op read "op://dev/cloudsmith-api-key1/credential")

token=$(op read "op://dev/cloudsmith-default-token/credential")
export CARGO_REGISTRIES_CLOUDSMITH_INDEX="https://dl.cloudsmith.io/$token/topohedrallabs/topohedral/cargo/index.git"