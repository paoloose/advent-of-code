import re

part1_regex = r'\b(\d+)\1\b'
part2_regex = r'\b(\d+)\1+\b'

day1 = lambda regex: sum(sum(x) for x in ([list(num for num in r if re.findall(regex, str(num))) for r in map(lambda r: range(*tuple(map(lambda p: int(p[1]) + 1 if p[0] == 1 else int(p[1]), enumerate(r.split('-'))))), open('input.txt').read().split(','))]))

print('part1:', day1(part1_regex), 'part2:', day1(part2_regex))
