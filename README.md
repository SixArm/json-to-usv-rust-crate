# json-to-usv

Convert JavaScript Object Notation (JSON) to [Unicode Separated Values (USV)](https://github.com/sixarm/usv).

Syntax:

```sh
stdin | json-to-usv [options] | stdout
```

Example:

```sh
cat example.json | json-to-usv
```

Example with output to a file:

```sh
cat example.json | json-to-usv > example.usv
```


## Options

* -d, --delimiter <delimiter> : Set the delimiter character [default: ";"]

* -h, --help : Print help

* -V, --version : Print version

* -v, --verbose... : Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace. Example: --verbose …

* --test : Print test output for debugging, verifying, tracing, and the like. Example: --test


## Install

Install:

```sh
cargo install json-to-usv
```

Link: [https://crates.io/crates/json-to-usv](https://crates.io/crates/json-to-usv)


## Example

Suppose file example.json contains:

```json
[
    ["a","b","c"],
    ["d","e","f"],
    ["g","h","i"]
]
```

Run:

```sh
cat example.json | json-to-usv
```

Output:

```usv
a␟b␟c␟␞
d␟e␟f␟␞
g␟h␟i␟␞
```


## FAQ

### When to use this command?

Use this command when you want to convert from JSON to USV.

A typical use case is when you have JSON data, such as a web request result,
and you want to convert it to USV, such as to make the data easier to view,
or edit, or maintain.

Our real-world use case is converting a bunch of JSON web API RPC results
from a variety of programs, including Excel, to USV so we're better-able to
handle quoting, and multi-line data units, and Unicode characters in a wide
variety of human languages.

### Is there a similar command to convert from USV to JSON?

Yes: [usv-to-json](https://crates.io/crates/usv-to-json).

### Why use USV instead of JSON?

See the documentation for [USV](https://github.com/sixarm/usv).

### Is USV aiming to become a standard?

Yes, USV is submitted to IETF.org as an Internet-Draft work in progress:
[link](https://datatracker.ietf.org/doc/draft-unicode-separated-values/).

## Help wanted

Constructive feedback welcome. Pull requests and feature requests welcome.

## Tracking

* Package: json-to-usv-rust-crate
* Version: 1.2.0
* Created: 2024-03-09-13:33:20Z
* Updated: 2024-03-17T19:24:53Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@sixarm.com)
