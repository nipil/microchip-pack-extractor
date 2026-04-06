#!/usr/bin/env python3

"""
CLAUDE PROMPT:

build me a python script which shows the aggregated possible xml structure of a bunch of xml sample files
* takes a list of xml files as arguments
* parses them using a sax parser for speed
* removes attribute contents, keep only attribute names
* removes all inner text, keeping only the xml elements
* add a specific attribute named __multiple__ to elements which are repeated, so we can differenciate from non-repeating elements
* cumulatively merges the elements of each file
* pretty prints the results to stdout for later analysis

Usage:
    python xml_structure.py file1.xml file2.xml [...]
    python xml_structure.py *.xml
"""

import sys
import xml.sax
import xml.sax.handler
from collections import OrderedDict


class StructureNode:
    """Represents a single element in the aggregated XML structure tree."""

    def __init__(self, tag: str):
        self.tag = tag
        self.attrs: set[str] = set()
        self.children: OrderedDict[str, "StructureNode"] = OrderedDict()

    def merge(self, other: "StructureNode") -> None:
        """Merge another StructureNode into this one (union of attrs and children)."""
        self.attrs |= other.attrs
        for name, child in other.children.items():
            if name in self.children:
                self.children[name].merge(child)
            else:
                self.children[name] = child

    def pretty_print(self, indent: int = 0) -> None:
        pad = "  " * indent
        regular = sorted(a for a in self.attrs if a != "__multiple__")
        special = ["__multiple__"] if "__multiple__" in self.attrs else []
        all_attrs = regular + special
        attr_str = (" " + " ".join(f'{a}=""' for a in all_attrs)) if all_attrs else ""
        if self.children:
            print(f"{pad}<{self.tag}{attr_str}>")
            for child in self.children.values():
                child.pretty_print(indent + 1)
            print(f"{pad}</{self.tag}>")
        else:
            print(f"{pad}<{self.tag}{attr_str} />")


class StructureHandler(xml.sax.handler.ContentHandler):
    """SAX handler that builds a StructureNode tree from an XML file."""

    def __init__(self):
        self.stack: list[StructureNode] = []
        self.root: StructureNode | None = None

    def startElement(self, name: str, attrs: xml.sax.xmlreader.AttributesImpl) -> None:
        node = StructureNode(name)
        node.attrs = set(attrs.getNames())
        if self.stack:
            parent = self.stack[-1]
            if name in parent.children:
                # Merge attrs seen in repeated elements at this level
                existing = parent.children[name]
                existing.attrs |= node.attrs
                existing.attrs.add("__multiple__")   # mark as repeating
                # Push the existing node so children are also merged
                self.stack.append(existing)
            else:
                parent.children[name] = node
                self.stack.append(node)
        else:
            self.stack.append(node)
            self.root = node

    def endElement(self, name: str) -> None:
        self.stack.pop()

    # Ignore all character data
    def characters(self, content: str) -> None:
        pass


def parse_file(path: str) -> StructureNode | None:
    handler = StructureHandler()
    try:
        xml.sax.parse(path, handler)
    except xml.sax.SAXParseException as e:
        print(f"[warning] Skipping {path}: {e}", file=sys.stderr)
        return None
    except OSError as e:
        print(f"[warning] Cannot open {path}: {e}", file=sys.stderr)
        return None
    return handler.root


def main() -> None:
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} file_list.txt", file=sys.stderr)
        sys.exit(1)

    list_path = sys.argv[1]

    try:
        with open(list_path, "r", encoding="utf-8") as f:
            files = [line.strip() for line in f if line.strip() and not line.startswith("#")]
    except OSError as e:
        print(f"[error] Cannot read file list {list_path}: {e}", file=sys.stderr)
        sys.exit(1)

    aggregated: StructureNode | None = None

    for path in files:
        print(f"Parsing: {path}", file=sys.stderr)
        tree = parse_file(path)
        if tree is None:
            continue
        if aggregated is None:
            aggregated = tree
        elif aggregated.tag == tree.tag:
            aggregated.merge(tree)
        else:
            # Different root tags — wrap both under a synthetic <root>
            if aggregated.tag != "__root__":
                wrapper = StructureNode("__root__")
                wrapper.children[aggregated.tag] = aggregated
                aggregated = wrapper
            if tree.tag in aggregated.children:
                aggregated.children[tree.tag].merge(tree)
            else:
                aggregated.children[tree.tag] = tree

    if aggregated is None:
        print("No valid XML files parsed.", file=sys.stderr)
        sys.exit(1)

    print()  # blank line separating stderr progress from stdout output
    aggregated.pretty_print()


if __name__ == "__main__":
    main()
