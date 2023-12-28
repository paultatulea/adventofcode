from pathlib import Path
import io
import pytest

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"""
EXPECTED = 36


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def solution(s: str) -> int:
    n = 10
    start_x = start_y = 0
    knots = [[start_x, start_y] for _ in range(n)]
    move = {"R": (1, 0), "U": (0, 1), "L": (-1, 0), "D": (0, -1)}
    commands = s.splitlines()
    vis: set[tuple[int, int]] = set()
    vis.add((start_x, start_y))
    minx = miny = maxx = maxy = 0
    s = io.StringIO()

    # format_grid(s, minx, miny, maxx, maxy, vis, knots)

    for command in commands:
        s.write("\n")
        s.write(f" {command} ".center(20, "="))
        cmd, n = command.split()
        m = move[cmd]
        n = int(n)

        for _ in range(n):
            # Move head
            knots[0][0] += m[0]
            knots[0][1] += m[1]

            minx = min(minx, knots[0][0])
            miny = min(miny, knots[0][1])
            maxx = max(maxx, knots[0][0])
            maxy = max(maxy, knots[0][1])

            for i, (tail_x, tail_y) in enumerate(knots[1:], start=1):
                prev_x, prev_y = knots[i - 1]
                # Determine if tail needs to move
                # More efficient way to do this for sure..
                diff_x = prev_x - tail_x
                diff_y = prev_y - tail_y
                if abs(diff_x) < 2 and abs(diff_y) < 2:
                    break
                adj_x = abs(diff_x) // 2
                adj_y = abs(diff_y) // 2
                knots[i][0] += diff_x + (-adj_x if diff_x > 0 else adj_x)
                knots[i][1] += diff_y + (-adj_y if diff_y > 0 else adj_y)

                # format_grid(s, minx, miny, maxx, maxy, vis, knots)
                # print(s.getvalue())
                # s.truncate(0)
                # s.seek(0)
            # Add position of last knot at end of each move
            vis.add(tuple(knots[-1]))

    return len(vis)


def format_grid(
    s: io.StringIO,
    minx: int,
    miny: int,
    maxx: int,
    maxy: int,
    vis: set[tuple[int, int]],
    knots: list[tuple[int, int]],
) -> None:
    # Grossly inefficient visual formatting.
    range_x = max(maxx - minx, 10)
    range_y = max(maxy - miny, 10)
    grid = [["." for _ in range(range_x + 1)] for _ in range(range_y + 1)]
    for vis_x, vis_y in vis:
        grid[vis_y - miny][vis_x - minx] = "#"
    for i in range(len(knots) - 1, -1, -1):
        knot_x, knot_y = knots[i]
        label = "H" if i == 0 else str(i)
        grid[knot_y - miny][knot_x - minx] = label

    s.write("\n")
    s.write("\n".join("".join(row) for row in reversed(grid)))
    s.write("\n")
    s.write("-" * 40)


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
