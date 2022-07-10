#!/bin/bash

near delete test.chrislm.testnet chrislm.testnet

near create-account test.chrislm.testnet --masterAccount chrislm.testnet

#near deploy test.chrislm.testnet --wasmFile=./res/vertabrate_animals.wasm
near deploy test.chrislm.testnet --wasmFile target/wasm32-unknown-unknown/release/vertabrate_animals.wasm

#near deploy --wasmFile target/wasm32-unknown-unknown/release/rust_counter_tutorial.wasm --accountId YOUR_ACCOUNT_HERE