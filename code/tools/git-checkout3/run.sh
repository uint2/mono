#!/bin/sh

cargo build
PATH=./target/debug:$PATH git checkout3 hello
echo "returned code $?"
