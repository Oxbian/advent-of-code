import re

def open_file(filename):
    f = open(filename, "r")
    return f.readlines()

def game(data):
    result_q1, result_q2 = 0, 0
    
    for line in data:
        blue = max(map(int, re.findall("(\d+) blue", line)))
        red = max(map(int, re.findall("(\d+) red", line)))
        green = max(map(int, re.findall("(\d+) green", line)))

        if red <= 12 and green <= 13 and blue <= 14:
            result_q1 += int(re.match("Game (\d+):", line).group(1))

        result_q2 += blue * green * red

    print(f"Question 1: {result_q1}")
    print(f"Question 2: {result_q2}")

game(open_file('input.txt'))
