from pathlib import Path
import pytest

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"""
EXPECTED = 13


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def solution(s: str) -> int:
    head_x, head_y = tail_x, tail_y = 0, 0
    move = {"R": (1, 0), "U": (0, 1), "L": (-1, 0), "D": (0, -1)}
    commands = s.splitlines()
    vis: set[tuple[int, int]] = set()
    vis.add((0, 0))
    for command in commands:
        m, n = command.split()
        m = move[m]
        n = int(n)

        for _ in range(n):
            # Move head
            head_x += m[0]
            head_y += m[1]

            # Determine if tail needs to move
            # More efficient way to do this for sure..
            if abs(head_x - tail_x) == 2:
                tail_x += m[0]
                tail_y += head_y - tail_y
                vis.add((tail_x, tail_y))
            elif abs(head_y - tail_y) == 2:
                tail_y += m[1]
                tail_x += head_x - tail_x
                vis.add((tail_x, tail_y))

    return len(vis)


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
