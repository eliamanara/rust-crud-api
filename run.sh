#!/usr/bin/env bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
cargo build --release && cargo run && exit