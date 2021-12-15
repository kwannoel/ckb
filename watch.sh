#!/usr/bin/env bash
inotifywait ../ckb-avoum-auction avoum store chain tx-pool --monitor --event modify --recursive --include .rs |
while read -r _folder event _file; do
    echo $_folder
    if [ $event = "MODIFY" ]
    then
        cargo check
    fi
done
