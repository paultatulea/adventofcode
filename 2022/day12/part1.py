from pathlib import Path
import sys
import pprint
import tabulate
import pytest

# from collections import deque
from queue import PriorityQueue

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"""
EXPECTED = 31


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def distance(row: int, col: int, end_row: int, end_col: int) -> int:
    return (row - end_row) ** 2 + (col - end_col) ** 2


def check_in_grid(row: int, col: int, width: int, height: int) -> bool:
    return row >= 0 and col >= 0 and row < height and col < width


def solution(s: str) -> int:
    grid = []
    start = None
    end = None
    for i, line in enumerate(s.splitlines()):
        row = []
        for j, c in enumerate(line):
            if c == "S":
                row.append(ord("a"))
                start = (i, j)
            elif c == "E":
                row.append(ord("z"))
                end = (i, j)
            else:
                row.append(ord(c))
        grid.append(row)

    height = len(grid)
    width = len(grid[0])

    visited: set[tuple[int, int]] = set()
    steps = [[sys.maxsize for _ in row] for row in grid]
    prev = [[None for _ in row] for row in grid]
    steps[start[0]][start[1]] = 0

    q = []
    for i in range(height):
        for j in range(width):
            q.append((i, j))
    # down, up, right, left
    neighbours = [(1, 0), (-1, 0), (0, 1), (0, -1)]

    while q:
        node = q[0]
        node_index = 0
        for i, item in enumerate(q):
            if steps[item[0]][item[1]] < steps[node[0]][node[1]]:
                node = item
                node_index = i
        q.pop(node_index)
        visited.add(node)
        row, col = node
        if node == end:
            return steps[row][col]

        for direction in neighbours:
            neighbour = row + direction[0], col + direction[1]
            x, y = neighbour

            # Not in grid
            if not check_in_grid(x, y, width, height):
                continue

            # Limit at most one elevation higher
            if grid[x][y] - grid[row][col] > 1:
                continue

            if neighbour in visited:
                continue

            calc = steps[row][col] + 1
            if calc < steps[x][y]:
                steps[x][y] = calc
                prev[x][y] = node

    assert False, "Could not find path to destination"


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
