import re
from glob import glob
from pathlib import Path


def sed_file(filename):
    lines = open(filename).readlines()
    changed = False
    for i, line in enumerate(lines):
        m = TABLE_RE.match(line) or BARE_RE.match(line)
        if not m:
            continue
        lib, old_version = m.group(2), m.group(3)
        new_version = list_of_crates.get(lib)
        print("File:", filename, lib, old_version, "->", new_version)
        if new_version is None or new_version == old_version:
            if new_version is None:
                print("WARNING: no kellnr version found for dependency", lib, "in", filename)
            continue
        lines[i] = line[: m.start(3)] + new_version + line[m.end(3):]
        changed = True
    if changed:
        with open(filename, "w") as f:
            f.writelines(lines)


TABLE_RE = re.compile(r'^(\s*)(mk_lib_\w+)\s*=\s*\{[^}]*?\bversion\s*="([^"]+)"')
BARE_RE = re.compile(r'^(\s*)(mk_lib_\w+)\s*=\s*"([^"\n]+)"\s*$')

# find all current kellnr versions and bump level
list_of_crates = {}
root = Path(__file__).resolve().parent.parent
for filename in sorted(glob(str(root / 'src/mk_lib_*/Cargo.toml'))):
    text = open(filename).read()
    pkg_match = re.search(r'\[package\](.*?)(?=^\[|\Z)', text, flags=re.M | re.S)
    name_m = ver_m = None
    if pkg_match:
        block = pkg_match.group(1)
        name_m = re.search(r'(?m)^name\s*=\s*"([^"]+)"', block)
        ver_m = re.search(r'(?m)^version\s*="\K[^"]+', block)
    if not (pkg_match and name_m and ver_m):
        print("WARNING: could not parse [package] name/version in", filename, "- skipped")
        continue
    crate_name = name_m.group(1)
    old_version = ver_m.group()
    major_minor, patch = old_version.rsplit(".", 1)
    new_version = major_minor + "." + str(int(patch) + 1)
    list_of_crates[crate_name] = new_version
    print("Bump:", filename, crate_name, old_version, "->", new_version)
    with open(filename, "w") as f:
        f.write(re.sub(r'(?m)^version\s*="\K[^"]+', new_version, text, count=1))

print("My Kellnr Cargos: ", list_of_crates)

# loop through librarys
for filename in sorted(glob(str(root / 'src/mk_lib*/Cargo.toml'))):
    sed_file(filename)

# loop through docker images
for filename in sorted(glob(str(root / 'docker/core/mk*/Cargo.toml'))):
    sed_file(filename)

# loop through app directory
for filename in sorted(glob(str(root / 'src_app/*/Cargo.toml'))):
    sed_file(filename)
