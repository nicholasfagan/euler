#!/bin/bash
cat names.txt | tr -d '"' | tr ',' '\n' | sort | awk 'BEGIN {convert="ABCDEFGHIJKLMNOPQRSTUVWXYZ"} {split($0,chars,""); cur=0;for(i=1;i<=length($0);i++)cur+=index(convert, chars[i]); print cur}' | awk '{a++; sum += a * $1} END { print sum }'
