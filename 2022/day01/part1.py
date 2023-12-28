def solution():
    best = 0
    curr = 0
    with open("day01/input.txt") as f:
        for line in f:
            clean_line = line.strip()
            if not clean_line:
                best = max(best, curr)
                curr = 0
            else:
                curr += int(clean_line)
        # check the last
        best = max(best, curr)
    print(f"{best=}")


solution()
