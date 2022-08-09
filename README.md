# plop

[![crates.io](https://img.shields.io/crates/v/plop-cli.svg)](https://crates.io/crates/plop-cli)

plop is a command-line tool to upload and share files via AWS S3 presigned URIs.

## Install plop

With crates.io:
```bash
cargo install plop-cli
```

From source:
```bash
cargo install --path /path/to/plop/repo
```

Or, download and run the binary from [the latest release](https://github.com/raylas/plop/releases).

## Use plop

```bash
Upload and share files via S3 presigned URIs

USAGE:
    plop [FLAGS] [OPTIONS] <file> <bucket>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

OPTIONS:
    -e, --expire <expire>    Lifetime, in seconds, of presigned URI [default: 1800]
    -p, --prefix <prefix>    Prefix for uploaded objects [default: plop/]
    -r, --region <region>    AWS Region to use [env: AWS_REGION=]

ARGS:
    <file>      Local file to upload
    <bucket>    Name of the bucket
```
