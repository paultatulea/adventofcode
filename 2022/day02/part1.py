from enum import Enum
from pathlib import Path
import pytest

PATH = Path(__file__).parent.resolve() / "input.txt"


class Choice(str, Enum):
    ROCK = "ROCK"
    PAPER = "PAPER"
    SCISSORS = "SCISSORS"


class Outcome(Enum):
    LOSE = -1
    DRAW = 0
    WIN = 1


choice_lookup = {
    "A": Choice.ROCK,
    "B": Choice.PAPER,
    "C": Choice.SCISSORS,
    "X": Choice.ROCK,
    "Y": Choice.PAPER,
    "Z": Choice.SCISSORS,
}

point_choice_lookup = {
    Choice.ROCK: 1,
    Choice.PAPER: 2,
    Choice.SCISSORS: 3,
}

point_outcome_lookup = {
    Outcome.LOSE: 0,
    Outcome.DRAW: 3,
    Outcome.WIN: 6,
}


def get_result(choice: Choice, other: Choice) -> Outcome:
    if choice == other:
        return Outcome.DRAW
    if choice == Choice.ROCK:
        return Outcome.WIN if other == Choice.SCISSORS else Outcome.LOSE
    if choice == Choice.PAPER:
        return Outcome.WIN if other == Choice.ROCK else Outcome.LOSE
    if choice == Choice.SCISSORS:
        return Outcome.WIN if other == Choice.PAPER else Outcome.LOSE


INPUT_S = """\
A Y
B X
C Z"""
EXPECTED = 15


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def solution(s: str) -> int:
    score = 0
    for line in s.splitlines():
        opp, me = line.split()
        opp = Choice(choice_lookup[opp])
        me = Choice(choice_lookup[me])
        choice_score = point_choice_lookup[me]
        result = get_result(me, opp)
        result_score = point_outcome_lookup[result]
        round_score = choice_score + result_score
        print(f"{opp=} {me=} {round_score=}")
        score += round_score
    return score


def main() -> None:
    with open(PATH) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
