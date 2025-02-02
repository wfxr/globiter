# globiter &emsp; [![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/workflow/status/Xuanwo/globiter/CI/main
[actions]: https://github.com/Xuanwo/globiter/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/globiter.svg
[crates.io]: https://crates.io/crates/globiter

`globiter` turns glob range and set into Iterator.

## Quick Start

```rust
let pattern = Pattern::parse("https://example.com/{a,b,c}/file/{x,y,z}")?;
for i in pattern.iter() {
    println("{i}")
}
// Output:
// https://example.com/a/file/x
// https://example.com/a/file/y
// https://example.com/a/file/z
// ...
// https://example.com/c/file/z
```

## Contributing

Check out the [CONTRIBUTING.md](./CONTRIBUTING.md) guide for more details on getting started with contributing to this project.

## Getting help

Submit [issues](https://github.com/Xuanwo/globiter/issues/new/choose) for bug report or asking questions in [discussion](https://github.com/Xuanwo/globiter/discussions/new?category=q-a).

#### License

<sup>
Licensed under <a href="./LICENSE">Apache License, Version 2.0</a>.
</sup>
