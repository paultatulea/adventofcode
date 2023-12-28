import io
import string

testinput = """vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"""

priority = dict((a, i) for i, a in enumerate(string.ascii_lowercase, start=1))
priority.update(dict((a, i) for i, a in enumerate(string.ascii_uppercase, start=27)))


def find_common(s: str) -> str:
    assert not len(s) % 2
    i = len(s) // 2
    first = set(s[:i])
    second = set(s[i:])
    common = first.intersection(second)
    assert len(common) == 1
    return next(iter(common))


def solution():
    score = 0
    # f = io.StringIO(testinput)
    f = open("day03/input.txt")
    i = 0
    for line in f:
        if i == 0:
            common = set(line.strip())
        else:
            common.intersection_update(line.strip())
        if i == 2:
            assert len(common) == 1
            badge = next(iter(common))
            score += priority[badge]
            i = 0
        else:
            i += 1
    print(score)


solution()
