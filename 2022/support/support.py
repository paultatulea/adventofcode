import os
from pathlib import Path

SKELETON = """\
from pathlib import Path
import pytest

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = '''\\
'''
EXPECTED = 0


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def solution(s: str) -> int:
    pass


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
"""


def initialize_day() -> None:
    cwd = Path(os.getcwd())
    base = cwd.stem
    if not base.startswith("day"):
        raise SystemExit("Expected directory to follow 'dayXX' convention.")

    init_py_path = cwd / "__init__.py"
    init_py_path.touch()

    part1_py_path = cwd / "part1.py"
    with open(part1_py_path, "w") as f:
        f.write(SKELETON)

    input_txt_path = cwd / "input.txt"
    input_txt_path.touch()
