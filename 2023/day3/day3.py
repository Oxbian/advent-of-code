import math
import re
from collections import defaultdict

f = open("input.txt", "r")
data = f.readlines()

box = [(x,y) for x in [-1, 0, 1] for y in [-1, 0, 1]]

symbols = {(i, j): symbol 
            for i, line in enumerate(data) 
            for j, symbol in enumerate(line)
            if symbol != "." and symbol != '\n' and not symbol.isdigit()
           }


part_sum = 0
parts_by_symbol = defaultdict(list)

for i, line in enumerate(data):
    for match in re.finditer(r"\d+", line):
        n = int(match.group(0))
        boundary = {
            (i + di, j + dj)
            for di, dj in box
            for j in range(match.start(), match.end())
        }
        if symbols.keys() & boundary:
            part_sum += n
        for symbol in symbols.keys() & boundary:
            parts_by_symbol[symbol].append(n)

# Part 1
print(part_sum)

# Part 2
print(
    sum(
        math.prod(v)
        for k, v in parts_by_symbol.items()
        if len(v) == 2 and symbols[k] == "*"
    )
)
