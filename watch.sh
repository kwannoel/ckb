#!/usr/bin/env bash
inotifywait avoum chain tx-pool --monitor --event modify --recursive --include .rs |
while read -r _folder event _file; do
    echo $_folder
    if [ $event = "MODIFY" ]
    then
        cargo check
    fi
done
