import re

def read_file(file):
    f = open(file, "r")
    return f.read().strip()

def calibration(data):
    ls = data.split("\n")
    ns = [re.findall("\d", x) for x in ls]
    return sum(int(n[0] + n[-1]) for n in ns)

def calibration2(data):
    data = (
        data.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
    )
    return calibration(data)

# Challenge 1
print(f'Question 1: {calibration(read_file("input.txt"))}')

# Challenge 2
print(f'Question 2: {calibration2(read_file("input.txt"))}')
