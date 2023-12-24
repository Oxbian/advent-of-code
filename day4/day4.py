import re

f = open("input.txt", "r").read().strip().split("\n")

def scratchcards(data):
    number = 0
    scratchcards = [1] * len(data)
    for i,line in enumerate(data):
        lnum, rnum = line.split(':')[1].split("|")
        win = re.findall("\d+", lnum)
        num = re.findall("\d+", rnum)
        occurency = len([ s for s in num if s in win ])

        if occurency:
            number += 2 ** (occurency - 1)

        for j in range(occurency):
            scratchcards[i + j + 1] += scratchcards[i]

    print(f"Part 1: {number}")
    print(f"Part 2: {sum(scratchcards)}")

scratchcards(f)

