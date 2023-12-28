from dataclasses import dataclass
from typing import Callable
from pathlib import Path
import functools
import pytest

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"

INPUT_S = """\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"""
EXPECTED = 2713310158


@dataclass
class Monkey:
    n: int
    items: list[int]
    op: Callable[[int], int]
    test_case: int
    true_case: int
    false_case: int


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def solution(s: str) -> int:
    monkeys: list[Monkey] = []
    rounds = 10_000
    for i, item in enumerate(s.split("\n\n")):
        lines = item.splitlines()
        items = [int(x) for x in lines[1].rpartition(": ")[2].split(", ")]
        opstr = lines[2].rpartition("= ")[2]
        op = eval("lambda old: " + opstr)
        test_case = int(lines[3].split()[-1])
        true_case = int(lines[4].split()[-1])
        false_case = int(lines[5].split()[-1])
        monkey = Monkey(i, items, op, test_case, true_case, false_case)
        monkeys.append(monkey)

    inspected = [0 for _ in monkeys]
    all_factor = functools.reduce(lambda x, y: x * y, (m.test_case for m in monkeys), 1)

    for nround in range(rounds):
        for i, monkey in enumerate(monkeys):
            inspected[i] += len(monkey.items)
            for item in monkey.items:
                after = monkey.op(item) % all_factor
                next_monkey = (
                    monkey.true_case
                    if not after % monkey.test_case
                    else monkey.false_case
                )
                monkeys[next_monkey].items.append(after)
            monkey.items = []

        if not nround % 1000:
            print(f"== After round {nround} ==")
            for i, monkey in enumerate(inspected):
                print(f"Monkey {i} inspected items {monkey} times.")

    inspected.sort()
    print(inspected)
    return inspected[-1] * inspected[-2]


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
