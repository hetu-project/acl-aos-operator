#!/bin/bash

while true
do
    clear
    curl http://127.0.0.1:9000/metrics
    echo "Request sent at $(date)"
    sleep 5
done
