#!/bin/bash

riscv64-unknown-elf-gdb \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'