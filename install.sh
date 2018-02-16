#!/bin/bash

cargo build --release
mv target/release/mockapi /usr/local/bin/mockapi

