#!/bin/zsh
export RUSTDOCFLAGS="--html-in-header $(pwd)/docs/rustdoc-html/custom-header.html --document-private-items"
export TOPO_LOG=all=trace
