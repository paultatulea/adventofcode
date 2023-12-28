from pathlib import Path
import pytest

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"""
EXPECTED = 0


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


HEIGHT = 6
WIDTH = 40
SPRITE_WIDTH = 3
SPRITE_PIXEL = "#"
DARK_PIXEL = "."


def is_sprite(sprite: int, crt_pos: int) -> bool:
    return sprite - 1 <= crt_pos <= sprite + 1


def solution(s: str) -> int:
    x = 1  # sprite center position
    cycle_count = 0
    rows = []
    row = ""
    for op in s.splitlines():
        parts = op.split()
        instruction = parts[0]
        cycles_left = 1 if instruction == "noop" else 2
        while cycles_left:
            cycle_count += 1
            crt_position = cycle_count % WIDTH
            # Check if need to jump to next line
            if cycle_count > 1 and crt_position == 1:
                rows.append(row)
                row = ""

            pixel = SPRITE_PIXEL if is_sprite(x, len(row)) else DARK_PIXEL
            row += pixel

            cycles_left -= 1

        if instruction == "addx":
            x += int(parts[1])

    # add the last row
    rows.append(row)
    print("\n\n")
    print("\n".join(rows[-HEIGHT:]))
    return 0


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
