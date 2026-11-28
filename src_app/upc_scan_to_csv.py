#!/usr/bin/env python3
"""Convert the pi_upc sqlite3 database (upc_scan.db) to CSV files.

Writes one CSV per table into the same directory as the database file,
named <table>.csv. The database is opened read-only and never modified.

With --codes-only, only upc_codes.csv is written, containing the
deduplicated upc_code values (one per line, sorted).

Usage:
    python3 upc_scan_to_csv.py [--codes-only] [path/to/upc_scan.db]

If no path is given, upc_scan.db in the current working directory is used.
"""

import argparse
import csv
import sqlite3
import sys
from pathlib import Path


def _write_csv(out_path: Path, cur: sqlite3.Cursor, headers) -> None:
    try:
        with out_path.open("w", newline="", encoding="utf-8") as f:
            writer = csv.writer(f)
            writer.writerow(headers)
            writer.writerows(cur)
    finally:
        cur.close()
    print(f"wrote {out_path}")


def convert(db_path: Path, codes_only: bool) -> None:
    if not db_path.is_file():
        raise FileNotFoundError(f"database not found: {db_path}")

    out_dir = db_path.resolve().parent
    # mode=ro keeps the db untouched even if the file is writable
    uri = db_path.resolve().as_uri() + "?mode=ro"
    con = sqlite3.connect(uri, uri=True)
    try:
        if codes_only:
            cur = con.execute(
                "SELECT DISTINCT upc_code FROM upc_codes ORDER BY upc_code"
            )
            _write_csv(out_dir / "upc_codes.csv", cur, ["upc_code"])
            return

        tables = [
            row[0]
            for row in con.execute(
                "SELECT name FROM sqlite_master "
                "WHERE type = 'table' AND name NOT LIKE 'sqlite_%' "
                "ORDER BY name"
            )
        ]
        if not tables:
            print(f"no tables found in {db_path}")
            return
        for table in tables:
            cur = con.execute(f'SELECT * FROM "{table}"')
            _write_csv(
                out_dir / f"{table}.csv", cur, [d[0] for d in cur.description]
            )
    finally:
        con.close()


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Convert upc_scan.db to CSV files in its directory."
    )
    parser.add_argument(
        "db",
        nargs="?",
        default="upc_scan.db",
        help="path to the sqlite database (default: upc_scan.db)",
    )
    parser.add_argument(
        "--codes-only",
        action="store_true",
        help="write only upc_codes.csv with deduplicated upc_code values",
    )
    args = parser.parse_args()
    try:
        convert(Path(args.db), args.codes_only)
    except (FileNotFoundError, sqlite3.Error) as exc:
        print(f"error: {exc}", file=sys.stderr)
        raise SystemExit(1)


if __name__ == "__main__":
    main()
