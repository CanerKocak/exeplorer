#!/bin/bash

cargo build --release --target wasm32-unknown-unknown --package exeplorer_backend

# then generate the candid file from the wasm file
candid-extractor target/wasm32-unknown-unknown/release/exeplorer_backend.wasm >src/exeplorer_backend/exeplorer_backend.did

# then copy the candid file to the frontend declarations by dfx generate
dfx generate exeplorer_backend
