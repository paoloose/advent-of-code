from common import input_lines, get_amount_of_matches

matches_num = [get_amount_of_matches(line) for line in input_lines]
cards_num = [1] * len(matches_num)

for i, (matches, cards) in enumerate(zip(matches_num, cards_num)):
    for m in range(i + 1, i + matches + 1):
        cards_num[m] += cards

print(f"sum: {sum(cards_num)}")
