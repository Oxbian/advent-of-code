import re

seeds, *data = open("input-test.txt","r").read().strip().split('\n')

def almanac(seedsl, data):
    seeds = list(map(int, re.findall("(\d+)", seedsl)))
    for i, line in enumerate(data):
        if len(line) != 0:
            print(f"Line {line}")
            seed_map = list(map(int, re.findall("(\d+)", line)))
            if len(seed_map) != 0:
                for j,seed in enumerate(seeds):
                    if seed_map[1] <= seed < seed_map[1] + seed_map[2]:
                        print(f"Val - src: {seed-seed_map[1]}, val - src + dst: {seed-seed_map[1]+seed_map[0]}")
                        seeds[j] = seed_map[0] + (seed - seed_map[1])
                print(f"Seeds: {seeds}\n")
    print(seeds)
    print(min(seeds))

almanac(seeds, data)
