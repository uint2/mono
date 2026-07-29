#!/bin/sh

cargo build
PATH=./target/debug:$PATH git checkout3
echo $?
