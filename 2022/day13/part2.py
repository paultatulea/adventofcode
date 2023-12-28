from pathlib import Path
import functools
import pytest


INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"""
EXPECTED = 140


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def compare(left, right) -> bool:
    if isinstance(left, int) and isinstance(right, int):
        if left == right:
            return None
        return left < right
    if isinstance(left, list) and not isinstance(right, list):
        return compare(left, [right])
    if isinstance(right, list) and not isinstance(left, list):
        return compare([left], right)
    # both lists
    for (left_ele, right_ele) in zip(left, right):
        result = compare(left_ele, right_ele)
        if result is not None:
            return result
    if len(left) == len(right):
        return None
    return len(left) < len(right)


def compare_key(left, right):
    result = compare(left, right)
    if result is None:
        return 0
    return -1 if result else 1


def solution(s: str) -> int:
    packets = [[[2]], [[6]]]
    for pair in s.split("\n\n"):
        s1, s2 = pair.split("\n")
        left = eval(s1)
        right = eval(s2)
        packets.append(left)
        packets.append(right)

    sorted_packets = sorted(packets, key=functools.cmp_to_key(compare_key))
    return (sorted_packets.index(packets[0]) + 1) * (
        sorted_packets.index(packets[1]) + 1
    )


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
