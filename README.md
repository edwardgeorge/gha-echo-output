# GitHub Actions `echo-output` binary

A small binary for including in GitHub Actions to set an output in a shell script including escaping of multiline outputs.

### Usage

```
USAGE:
    echo-output <name>
```

Output is piped into the binary via stdin.

### Example

```
❯ echo-output foo <<- EOM
{                 
  "hello": "world"
}
EOM
::set-output name=foo::{%0A  "hello": "world"%0A}%0A
```
