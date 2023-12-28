def check(best: list[int], curr: int) -> list[int]:
    i = -1
    while i != len(best) - 1 and curr > best[i + 1]:
        i += 1
    if i == -1:
        pass
    else:
        best = best[1 : i + 1] + [curr] + best[i + 1 :]
    return best


def solution():
    best = [0, 0, 0]
    curr = 0
    with open("day01/input.txt") as f:
        for line in f:
            clean_line = line.strip()
            print(clean_line)
            if not clean_line:
                best = check(best, curr)
                curr = 0
            else:
                curr += int(clean_line)
        # Check the last line as well
        best = check(best, curr)
    print(f"{best=}, {sum(best)=}")


solution()
