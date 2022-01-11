# tak

A Git tagging helper that shows the next version to use. Works best when using [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/).

```shell
Determine the next version

USAGE:
    tak next [OPTIONS] [INCREMENT]

ARGS:
    <INCREMENT>    The type of version increment to use [default: auto] [possible values: patch,
                   minor, major, auto]

OPTIONS:
    -h, --help         Print help information
    -n, --no-prefix    Don't use the 'v' prefix
```
