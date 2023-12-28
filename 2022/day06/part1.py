from pathlib import Path
import pytest

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
"""
EXPECTED = 0


@pytest.mark.parametrize(
    ("input_s", "expected"),
    (
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def solution(s: str) -> int:
    for i in range(4, len(s)):
        window = s[i - 4 : i]
        if len(set(window)) == 4:
            return i


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
