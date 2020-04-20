#! /usr/bin/env bash
cargo build --release
cp ./target/release/mytip ~/Library/Application\ Scripts/tanin.tip/provider.script
