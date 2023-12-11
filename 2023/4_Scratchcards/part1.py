from common import input_lines, get_amount_of_matches

sum = 0

for line in input_lines:
    diff = get_amount_of_matches(line)
    if diff != 0:
        sum += 2 ** (diff - 1)

print(f"sum: {sum}")
