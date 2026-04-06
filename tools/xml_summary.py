#!/usr/bin/env python3

"""
PROMPT:
build me a python script which shows the aggregated possible xml structure of a bunch of xml sample files
* takes a list of xml files as arguments
* parses them using a sax parser for speed
* use startElementNS to keep all namespaces
* removes attribute contents, keep only attribute names
* removes all inner text, keeping only the xml elements
* add a specific attribute named __multiple__ to elements which are repeated, so we can differenciate from non-repeating elements
* cumulatively merges the elements of each file
* pretty prints the results to stdout for later analysis
* handle the namespaces elements and attributes according to namespace url, not only prefix name

Usage:
    python xml_structure.py file1.xml file2.xml [...]
    python xml_structure.py *.xml
"""

import sys
import xml.sax
import xml.sax.handler
from collections import OrderedDict


class StructureNode:
    def __init__(self, tag: str):
        self.tag = tag
        self.attrs: set[str] = set()
        self.children: OrderedDict[str, "StructureNode"] = OrderedDict()

    def merge(self, other: "StructureNode") -> None:
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

        def fmt(a: str) -> str:
            return a if "=" in a else f'{a}=""'

        attr_str = (" " + " ".join(fmt(a) for a in all_attrs)) if all_attrs else ""

        if self.children:
            print(f"{pad}<{self.tag}{attr_str}>")
            for child in self.children.values():
                child.pretty_print(indent + 1)
            print(f"{pad}</{self.tag}>")
        else:
            print(f"{pad}<{self.tag}{attr_str} />")


class StructureHandler(xml.sax.handler.ContentHandler):
    def __init__(self):
        super().__init__()
        self.stack: list[StructureNode] = []
        self.root: StructureNode | None = None

        self._pending_ns = []
        self.uri_to_prefix: dict[str, str] = {}

    def startPrefixMapping(self, prefix, uri):
        # normalize prefix (None → "")
        prefix = prefix or ""

        # register only first occurrence
        if uri not in self.uri_to_prefix:
            self.uri_to_prefix[uri] = prefix

        self._pending_ns.append((prefix, uri))

    def resolve_qname(self, uri, local, qname):
        if qname:
            return qname

        if uri in self.uri_to_prefix:
            prefix = self.uri_to_prefix[uri]
            if prefix:
                return f"{prefix}:{local}"
            else:
                return local

        # unknown namespace → drop prefix
        return local

    def startElementNS(self, name, qname, attrs):
        uri, local = name

        tag = self.resolve_qname(uri, local, qname)
        node = StructureNode(tag)

        node.attrs = set()

        # --- namespaces ---
        for prefix, uri in self._pending_ns:
            if prefix:
                node.attrs.add(f'xmlns:{prefix}="{uri}"')
            else:
                node.attrs.add(f'xmlns="{uri}"')
        self._pending_ns = []

        # --- attributes ---
        for (attr_uri, attr_local), value in attrs.items():
            attr_qname = attrs.getQNameByName((attr_uri, attr_local))
            attr_name = self.resolve_qname(attr_uri, attr_local, attr_qname)

            # skip xmlns (already handled)
            if attr_name.startswith("xmlns"):
                continue

            # ❗ critical fix: skip bogus None prefix
            if attr_name.startswith("None:"):
                continue

            node.attrs.add(attr_name)

        # --- tree ---
        if self.stack:
            parent = self.stack[-1]

            if tag in parent.children:
                existing = parent.children[tag]
                existing.attrs |= node.attrs
                existing.attrs.add("__multiple__")
                self.stack.append(existing)
            else:
                parent.children[tag] = node
                self.stack.append(node)
        else:
            self.stack.append(node)
            self.root = node

    def endElementNS(self, name, qname):
        self.stack.pop()

    def characters(self, content: str):
        pass


def parse_file(path: str) -> StructureNode | None:
    handler = StructureHandler()

    parser = xml.sax.make_parser()
    parser.setFeature(xml.sax.handler.feature_namespaces, True)
    parser.setContentHandler(handler)

    try:
        parser.parse(path)
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
        else:
            aggregated.merge(tree)

    if aggregated is None:
        print("No valid XML files parsed.", file=sys.stderr)
        sys.exit(1)

    print()
    aggregated.pretty_print()


if __name__ == "__main__":
    main()
