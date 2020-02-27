#!/bin/sh
seq 1 1000 | xargs -n1 num2words | tr -d ' ,\n-' |wc -c
