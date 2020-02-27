#!/bin/python

def digits_of(n):
    for c in str(n):
        yield int(c)

def digits():
    n = 1
    i = 1
    while True:
        for digit in digits_of(n):
            yield (digit, i)
            i += 1
        n += 1

prod = 1
for d,i in digits():
    if i == 1 or i == 10 or i == 100 or i == 1000 or i == 10000 or i == 100000 or i == 1000000:
        prod *= d
    if i == 1000000:
        break
print(prod)

