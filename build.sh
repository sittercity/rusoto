#!/bin/bash

for d in services/*/ ; do
    cd $d
    cargo build
    cd ../..
done
