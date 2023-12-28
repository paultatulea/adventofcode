from pathlib import Path
import itertools
import pytest

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"""
EXPECTED = 24


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def solution(s: str) -> int:
    entry = (500, 0)
    last_rock = -1
    unavailable: set[tuple[int, int]] = set()
    for line in s.splitlines():
        coords = line.split(" -> ")
        points = [tuple(map(int, x.split(","))) for x in coords]
        p1, p2 = itertools.tee(points)
        next(p2, None)
        for start_coord, end_coord in zip(p1, p2):
            start_x, start_y = start_coord
            end_x, end_y = end_coord
            last_rock = max(last_rock, start_y, end_y)
            # Vertical line
            if start_x == end_x:
                miny, maxy = min(start_y, end_y), max(start_y, end_y)
                for i in range(miny, maxy + 1):
                    unavailable.add((start_x, i))
            else:
                minx, maxx = min(start_x, end_x), max(start_x, end_x)
                for i in range(minx, maxx + 1):
                    unavailable.add((i, start_y))

    # Outer loop representing one unit of sand
    steps = ((0, 1), (-1, 1), (1, 1))
    units = 0
    while True:
        position = entry
        while True:
            if position[1] == last_rock:
                return units
            for step in steps:
                can_move = False
                next_position = position[0] + step[0], position[1] + step[1]
                if next_position not in unavailable:
                    position = next_position
                    can_move = True
                    break

            if not can_move:
                unavailable.add(position)
                break

        units += 1


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
