#!/bin/bash
# Convenience wrapper for xtask
# Usage: cargo xtask <command> [args]
# This script allows `cargo xtask` to work as if it were a cargo alias

exec cargo run --package xtask -- "$@"
