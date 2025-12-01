import os

def part1(data):
    position = 50
    zeros = 0
    for move in data:
        if abs(move) >= 100:
            move = abs(move) % 100 * (1 if move >= 0 else -1)
        position += move
        if position < 0:
            position += 100
        if position > 99:
            position -= 100
        if position == 0:
            zeros += 1
    return zeros

def part2(data):
    position = 50
    zeros = 0
    for move in data:
        if abs(move) >= 100:
            zeros += abs(move) // 100
            move = abs(move) % 100 * (1 if move >= 0 else -1)
        prev = position
        position += move

        if position < 0:
            position += 100
            if position != 0 and prev != 0:
                zeros += 1

        if position > 99:
            position -= 100
            if position != 0 and prev != 0:
                zeros += 1

        if position == 0:
            zeros += 1 
    return zeros

if __name__ == "__main__":
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    with open("day_01.txt", "r") as f:
        data = [int(line[1:]) * (-1 if line[0] == "L" else 1) for line in f.read().splitlines()]
        print("Answer: ", part1(data))
        print("Answer: ", part2(data))
    