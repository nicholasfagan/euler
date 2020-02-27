

from functools import reduce
from sys import argv

total = 0
for num in range(2,int(argv[1])):
    s = sum(map(lambda n: int(n)**5, str(num)))
    if s == num:
        total += num

print(total)
