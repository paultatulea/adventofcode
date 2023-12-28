from pathlib import Path
import pytest
import re

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"""

EXPECTED = 26


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def solution(s: str) -> int:
    target = 2000000
    positions = set()
    c = re.compile(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
    )
    for line in s.splitlines():
        m = c.match(line)
        sensor_x, sensor_y, beacon_x, beacon_y = [int(x) for x in m.groups()]
        distance = abs(sensor_x - beacon_x) + abs(sensor_y - beacon_y)

        # distance to target line
        target_distance = abs(target - sensor_y)
        target_range = distance - target_distance
        for x in range(sensor_x - target_range, sensor_x + target_range):
            positions.add((x, target))

    return len(positions)


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
