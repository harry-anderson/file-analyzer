#!/bin/bash
START=$1
END=$2
NAME=$3

for ((i=$START;i<=END;i++)); do
    cat files/raw/$i.txt >> files/joined/$NAME.txt
done

