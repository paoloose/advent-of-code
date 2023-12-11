import re

input_lines = open('day4.input').readlines()
sum = 0

def get_amount_of_matches(line):
    nums_list = re.split(r'\:\s+', line.rstrip())[1]
    left, right = re.split(r'\s+\|\s+', nums_list)

    winning_numbers = re.split(r'\s+', left)
    numbers_you_have = re.split(r'\s+', right)

    merged = set(winning_numbers + numbers_you_have)
    return len(winning_numbers) + len(numbers_you_have) - len(merged)
