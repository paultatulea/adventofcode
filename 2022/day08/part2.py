import pytest
from functools import reduce
from pathlib import Path


INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
30373
25512
65332
33549
35390
"""
EXPECTED = 8


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def is_valid(row: int, col: int, width: int, height: int) -> bool:
    return row >= 0 and col >= 0 and row < width and col < height


def is_edge(row: int, col: int, width: int, height: int) -> bool:
    return row == 0 or col == 0 or row == height - 1 or col == width - 1


def solution(s: str) -> int:
    grid = [[int(tree) for tree in row] for row in s.splitlines()]
    width = len(grid[0])
    height = len(grid)

    distance = [[[0] * 4 for _ in row] for row in grid]
    for row in range(1, height - 1):
        for col in range(1, width - 1):
            for i, (x, y) in enumerate(((0, 1), (0, -1), (1, 0), (-1, 0))):
                count = 0
                tree_x, tree_y = row + x, col + y
                while is_valid(tree_x, tree_y, width, height):
                    count += 1
                    if grid[tree_x][tree_y] >= grid[row][col]:
                        break
                    tree_x += x
                    tree_y += y
                distance[row][col][i] = count

    best = 0
    for row in distance:
        for col in row:
            score = reduce(lambda x, y: x * y, (d for d in col))
            best = max(best, score)
    return best


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
