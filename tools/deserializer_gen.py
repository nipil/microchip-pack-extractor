#!/usr/bin/env python3

import sys
from lxml import etree

RUST_KEYWORDS = {
    "as","break","const","continue","crate","else","enum","extern",
    "false","fn","for","if","impl","in","let","loop","match",
    "mod","move","mut","pub","ref","return","self","Self",
    "static","struct","super","trait","true","type","unsafe",
    "use","where","while","async","await","dyn",
}


def rust_safe(name: str) -> str:
    return name + "_" if name in RUST_KEYWORDS else name


def full_name(elem):
    q = etree.QName(elem.tag)
    return f"{elem.prefix}:{q.localname}" if elem.prefix else q.localname


def attr_name(elem, attr):
    if attr.startswith("{"):
        uri, local = attr[1:].split("}")
        for pfx, ns in elem.nsmap.items():
            if ns == uri:
                return f"{pfx}:{local}" if pfx else local
        return local
    return attr


def struct_name(name: str) -> str:
    return "".join(part.capitalize() for part in name.replace(":", "_").replace("-", "_").split("_"))


def field_name(name: str) -> str:
    name = name.replace(":", "_").replace("-", "_")
    out = ""
    for i, c in enumerate(name):
        if c.isupper() and i:
            out += "_"
        out += c.lower()
    return rust_safe(out)


class ElementDef:
    def __init__(self, name):
        self.name = name
        self.attributes = set()
        # name -> { def: ElementDef, multiple: bool }
        self.children = {}


def merge_defs(target: ElementDef, source: ElementDef):
    target.attributes |= source.attributes

    for cname, sentry in source.children.items():
        if cname not in target.children:
            # deep copy not needed for multiple flag safety anymore
            target.children[cname] = {
                "def": sentry["def"],
                "multiple": sentry["multiple"],
            }
        else:
            tentry = target.children[cname]

            # merge structure
            merge_defs(tentry["def"], sentry["def"])

            # 🔥 CRITICAL: only OR, never overwrite
            if sentry["multiple"]:
                tentry["multiple"] = True


def analyze(elem, defs):
    name = full_name(elem)

    if name not in defs:
        defs[name] = ElementDef(name)

    current = defs[name]

    # attributes
    for a in elem.attrib:
        if a == "__multiple__":
            continue
        current.attributes.add(attr_name(elem, a))

    # children
    for child in elem:
        cname = full_name(child)
        child_def = analyze(child, defs)

        is_multiple = "__multiple__" in child.attrib  # ✅ correct rule

        if cname not in current.children:
            current.children[cname] = {
                "def": child_def,
                "multiple": is_multiple,
            }
        else:
            entry = current.children[cname]

            # merge structure
            merge_defs(entry["def"], child_def)

            # 🔥 CRITICAL FIX: accumulate multiplicity ONLY upward
            if is_multiple:
                entry["multiple"] = True

    return current


def generate(defn: ElementDef):
    out = []

    sname = struct_name(defn.name)

    out.append("#[derive(Debug, Deserialize)]")
    out.append(f"pub struct {sname} {{")

    for attr in sorted(defn.attributes):
        fname = field_name(attr)
        out.append(f'    #[serde(rename = "@{attr}")]')
        out.append(f"    pub {fname}: String,")

    for cname in sorted(defn.children):
        entry = defn.children[cname]

        fname = field_name(cname)
        tname = struct_name(cname)

        if entry["multiple"]:
            out.append(f'    #[serde(rename = "{cname}", default)]')
            out.append(f"    pub {fname}: Vec<{tname}>,")
        else:
            out.append(f'    #[serde(rename = "{cname}")]')
            out.append(f"    pub {fname}: {tname},")

    out.append("}\n")
    return "\n".join(out)


def main():
    if len(sys.argv) != 2:
        print("usage: script.py file.xml", file=sys.stderr)
        sys.exit(1)

    tree = etree.parse(sys.argv[1])
    root = tree.getroot()

    defs = {}
    analyze(root, defs)

    print("use serde::Deserialize;\n")

    for name in sorted(defs):
        print(generate(defs[name]))


if __name__ == "__main__":
    main()