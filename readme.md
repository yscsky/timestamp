# timestamp

datetime and timestamp convert tool.

```sh
> timestamp
datetime and timestamp convert tool

Usage: timestamp <COMMAND>

Commands:
  now   show now datetime and timestamp
  dt    convert datetime to timestamp
  st    convert timestamp to datetime
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

# showcase

now:

```sh
> timestamp now
now datetime: 2026-02-04 10:07:07.056
now timestamp second: 1770170827 millisecond: 1770170827056
```

dt:

```sh
> timestamp dt "2026-02-04 10:07:07"
timestamp second: 1770170827 millisecond: 1770170827000

> timestamp dt "2026-02-04 10:07:07.056"
timestamp second: 1770170827 millisecond: 1770170827056
```

st:

```sh
> timestamp st 1770170747
datetime: 2026-02-04 10:05:47

> timestamp st 1770170747056
datetime: 2026-02-04 10:05:47.056
```
