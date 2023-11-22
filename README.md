# Entity Generator

[![CI]][CILink]

[CI]: https://github.com/hiroaki-yamamoto/egen/actions/workflows/test.yml/badge.svg
[CILink]: https://github.com/hiroaki-yamamoto/egen/actions/workflows/test.yml

This is a simple entity generator for Rust and Typescript.

# How to use

1. Download the binary.
2. Create a yml file like this:

`sample.yml`
```yml
type: struct
rust:
  derive: [Debug, Clone, "::serde::Serialize"]
  attrs: ['serde(rename_all = "camelCase")']
members:
  int128: i128
  uint128: u128
  boolean: bool
  text: string
  optText:
    type: string
    optional: true
```
Also, there's type definition schema: [definition.schema.json](definition.schema.json)
Note that that schema is not always up-to-date. If you need the latest "shape" of
input, refer [src/entities/inputs/root.rs](src/entities/inputs/root.rs).

3. Run the binary like this:

```bash
egen -i sample.yml -j yaml -o build -p rust
```

4. You'll get the following files:
  - `build/sample.rs`
5. Done :+1:

# Command Line Options

Refer [src/cmd/mod.rs](src/cmd/mod.rs). Also, you will see what type of
input/output is supported there.

# Contribution

This software is architected in SOLID principle. So, please obey it when you
send PR here. Plus, please add test cases for your PR.

# License
Refer [LICENSE](LICENSE.md).
