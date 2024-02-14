# dotsql

Generate a [Dot file](https://graphviz.org/doc/info/lang.html) from an
SQL database schema. This allows you to generate a graph of the
relationships in your database.

All the work is really done by
[sqlparser-rs](https://github.com/nick96/sqlparser-rs) (currently my
fork which adds some syntax used in `mysql-dump` that isn't in
mainline).

## Usage

To generate anything useful, you'll need to have GraphViz installed
(e.g. `brew install graphviz`).

```
cargo run -- schema.sql | dot -Tpdf >schema.pdf
```

Output to a PDF is nice (rather than the common `dot` example of
writing a PNG) because it is searchable meaning you can easily search
for a table of interest.
