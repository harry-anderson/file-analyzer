#!/bin/bash -xe
# END=1000
# for ((i=1;i<=END;i++)); do
#     cat files/raw/$i.txt >> files/joined/1000.txt
# done
#
cat files/raw/*.txt >> files/joined/all.txt
