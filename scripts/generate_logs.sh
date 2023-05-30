#!/bin/bash

filename="$PWD/logs/HubbleConstant.log"
outputfile="$PWD/logs/output.txt"

if [ ! -f "$filename" ]; then
    mkdir -p logs
    touch "$filename"
    echo "Created $filename"
else
    echo "$filename already exists"
fi

if [ ! -f "$outputfile" ]; then
    mkdir -p logs
    touch "$outputfile"
    echo "Created $outputfile"
else
    echo "$outputfile already exists"
fi