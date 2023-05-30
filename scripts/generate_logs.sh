#!/bin/bash

filename="$PWD/logs/HubbleConstant.log"
outputfile="$PWD/logs/output.txt"

if [ ! -f "$filename" ]; then
    mkdir -p logs
    touch "$filename"
    echo "Created $filename"
fi

if [ ! -f "$outputfile" ]; then
    mkdir -p logs
    touch "$outputfile"
    echo "Created $outputfile"
fi