#!/bin/bash -xe
END=14999
for ((i=9001;i<=END;i++)); do
    echo "deleting $1.txt"
    rm files/raw/$i.txt
done
