import pytest
from pathlib import Path

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
30373
25512
65332
33549
35390
"""
EXPECTED = 21


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def is_valid(row: int, col: int, width: int, height: int) -> bool:
    return row >= 0 and col >= 0 and row < width and col < height


def solution(s: str) -> int:
    grid = [[int(tree) for tree in row] for row in s.splitlines()]
    width = len(grid[0])
    height = len(grid)

    def visit(
        row: int,
        col: int,
        vis: set[tuple[int, int]],
        dp: list[list[int]],
        x: int,
        y: int,
    ) -> None:
        vis.add((row, col))
        neighbour_x, neighbour_y = row + x, col + y
        # Have not yet seen node, mark for visit
        if not is_valid(neighbour_x, neighbour_y, width, height):
            return
        if (neighbour_x, neighbour_y) not in vis:
            visit(neighbour_x, neighbour_y, vis, dp, x, y)
        dp[row][col] = max(dp[neighbour_x][neighbour_y], grid[neighbour_x][neighbour_y])

    all_dp = []
    # right, left, down, up
    for direction in (0, 1), (0, -1), (1, 0), (-1, 0):
        dp = [[0 for _ in row] for row in grid]
        visited = set()
        x, y = direction
        for i in range(height):
            for j in range(width):
                if (i, j) not in visited:
                    visit(i, j, visited, dp, x, y)
        all_dp.append(dp)

    # All outside
    visible = 0
    for i, row in enumerate(grid):
        for j, tree in enumerate(row):
            # Check if on edge
            if i == 0 or j == 0 or i == height - 1 or j == width - 1:
                visible += 1
            elif any(tree > dp[i][j] for dp in all_dp):
                visible += 1

    return visible


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
