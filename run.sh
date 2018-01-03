#!/bin/bash

while :
do
    if [ x"$PID" != "x" ]
    then
        kill $PID
        sleep 3
    fi
    if `cargo build`; then
        cargo run &
        PID=$!
        trap "{ kill $PID; exit 0; }" EXIT
        sleep 2
        echo "Wait ..."
    else
        sleep 2
        echo "Wait(err) ..."
    fi
    inotifywait -r --format '%:e %f' ./src
    echo "Reload ..."
done
