from pathlib import Path
import pytest

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"""
EXPECTED = "MCD"


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: str) -> None:
    assert solution(input_s) == expected


def solution(s: str) -> str:
    # Parse the starting position of stacks
    lines = s.splitlines()
    num_stacks = (len(lines[0]) + 1) // 4
    s = ["" for _ in range(num_stacks)]
    it = iter(lines)
    while True:
        line = next(it)
        if line[1] == "1":
            break

        for i, stack in enumerate(line[1::4]):
            if not stack == " ":
                s[i] += stack

    # Skip the empty line
    next(it)

    while True:
        try:
            line = next(it)
        except StopIteration:
            break

        parts = line.split()
        count = int(parts[1])
        source = int(parts[3]) - 1
        target = int(parts[5]) - 1

        s[target] = s[source][:count] + s[target]
        s[source] = s[source][count:]

    top = "".join((stack[0] if stack else "" for stack in s))
    return top


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
