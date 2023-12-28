from pathlib import Path
from typing import Iterable
import pytest
import io

INPUT_TXT = Path(__file__).parent.resolve() / "input.txt"


class Node:
    def __init__(
        self, name: str, parent: "Node", isdir: bool, size: int = None
    ) -> None:
        self.name = name
        self.parent = parent
        self.isdir = isdir
        self.size = size
        self.children: list["Node"] = []

    def add_child(self, node: "Node") -> None:
        self.children.append(node)

    def get_size(self) -> int:
        if self.size:
            return self.size
        return sum(child.get_size() for child in self.children)

    def format_tree(self, stream: io.StringIO, depth: int = 0, indent: int = 2) -> str:
        s = " " * indent * depth
        label = "dir" if self.isdir else f"file, size={self.get_size()}"
        s += f"- {self.name} ({label})"
        s += "\n"
        stream.write(s)
        for child in self.children:
            child.format_tree(stream, depth + 1, indent)


class Visitor:
    def __init__(self, min_space: int, unused: int) -> None:
        self.best = float("inf")
        self.best_node = None
        self.min_space = min_space
        self.unused = unused

    def visit(self, node: Node) -> int:
        if not node.isdir:
            return node.get_size()
        dirsize = 0
        for child in node.children:
            dirsize += self.visit(child)
        if self.unused + dirsize > self.min_space and dirsize < self.best:
            self.best = dirsize
            self.best_node = node
        return dirsize


INPUT_S = """\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"""
EXPECTED = 24933642


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test_case(input_s: str, expected: int) -> None:
    assert solution(input_s) == expected


def solution(s: str) -> int:
    assert s.startswith("$ cd /\n")
    it = iter(s.splitlines())
    root = Node(name="/", parent=None, isdir=True)
    cwd = root
    while True:
        try:
            line = next(it)
        except StopIteration:
            break

        if line.startswith("$ cd /"):
            while cwd.parent is not None:
                cwd = cwd.parent
        elif line.startswith("$ cd .."):
            cwd = cwd.parent
        elif line.startswith("$ cd"):
            _, _, dirname = line.split()
            cwd = next(node for node in cwd.children if node.name == dirname)
        # Don't need to do anything for $ ls
        elif line.startswith("$ ls"):
            pass
        elif line.startswith("dir"):
            _, dirname = line.split()
            cwd.add_child(Node(name=dirname, parent=cwd, isdir=True))
        else:
            size, fname = line.split()
            cwd.add_child(Node(name=fname, parent=cwd, isdir=False, size=int(size)))

    s = io.StringIO()
    root.format_tree(stream=s)
    max_size = 70_000_000
    # Visiting tree twice, should optimize
    unused = max_size - root.get_size()
    vis = Visitor(min_space=30_000_000, unused=unused)
    vis.visit(root)
    print(f"Best node: {vis.best_node.name}")
    return vis.best


def main() -> int:
    with open(INPUT_TXT) as f:
        s = f.read()
    print(solution(s))


if __name__ == "__main__":
    main()
