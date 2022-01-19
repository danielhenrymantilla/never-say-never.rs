#!/bin/sh

set -euxo pipefail

(cd src/proc_macros
    cargo +stable publish
)

for i in $(seq 10)
do
    cargo +stable publish && break
    sleep 5
done
