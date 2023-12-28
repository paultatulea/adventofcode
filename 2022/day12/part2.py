from pathlib import Path
import sys
import pytest
from queue import PriorityQueue

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"""
EXPECTED = 29


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def check_in_grid(row: int, col: int, width: int, height: int) -> bool:
    return row >= 0 and col >= 0 and row < height and col < width


def solution(s: str) -> int:
    grid = []
    end = None
    for i, line in enumerate(s.splitlines()):
        row = []
        for j, c in enumerate(line):
            if c == "S":
                row.append(ord("a"))
            elif c == "E":
                row.append(ord("z"))
                end = (i, j)
            else:
                row.append(ord(c))
        grid.append(row)

    height = len(grid)
    width = len(grid[0])

    # down, up, right, left
    neighbours = [(1, 0), (-1, 0), (0, 1), (0, -1)]
    visited: set[tuple[int, int]] = set()
    q = PriorityQueue()
    q.put_nowait((0, end))
    while not q.empty():
        item = q.get_nowait()
        steps, node = item
        if node in visited:
            continue
        visited.add(node)
        row, col = node
        if grid[row][col] == ord("a"):
            return steps

        for direction in neighbours:
            neighbour = row + direction[0], col + direction[1]
            x, y = neighbour

            # Not in grid
            if not check_in_grid(x, y, width, height):
                continue

            # Limit at most one elevation higher
            if grid[x][y] - grid[row][col] < -1:
                continue

            if neighbour in visited:
                continue

            q.put_nowait((steps + 1, neighbour))


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
