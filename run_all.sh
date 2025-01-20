#!/bin/bash

cargo run --example drawing --features="image-loading" &&
cargo run --example fullscreen &&
cargo run --example input &&
cargo run --example integer_scaling &&
cargo run --example pivot &&
cargo run --example post_process &&
cargo run --example tileset --features="image-loading" &&
cargo run --example ui --features="image-loading" &&
cargo run --example window
