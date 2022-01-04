# randompass

[![CI](https://github.com/mihaigalos/randompass/actions/workflows/ci.yaml/badge.svg)](https://github.com/mihaigalos/randompass/actions/workflows/ci.yaml)
[![CD](https://github.com/mihaigalos/randompass/actions/workflows/cd.yaml/badge.svg)](https://github.com/mihaigalos/randompass/actions/workflows/cd.yaml)
[![test coverage](https://codecov.io/gh/mihaigalos/randompass/branch/main/graph/badge.svg?token=WZPOJXZKCY)](https://codecov.io/gh/mihaigalos/randompass)
[![crates.io](https://img.shields.io/crates/d/randompass.svg)](https://crates.io/crates/randompass)
[![LoC](https://tokei.rs/b1/github/mihaigalos/randompass)](https://github.com/mihaigalos/randompass)

A simple static password generator.
Generated passwords are 16 characters, lower and uppercase and contain special characters by default.

### Why?

I got frustrated with generating passwords which met the required complexity.

There are a few implementations around, but their licensing meant they cannot be used in a professional setting.

Moreover, they require explicit flags to _enable_ a specific complication (i.e.: special chars, uppercase); `randompass` has them enabled by default and the user can _disable_ them instead.

Finally, other implementations do not guarantee that the required complexity has been met (i.e.: the password might not contain a required complication).

### Usage

##### Building from source

```bash
cargo install randompass
```

##### Using precompiled binaries

Precompiled binaries are avialable for multiple architectures in the [Releases](https://github.com/mihaigalos/randompass/releases).

```bash
randompass
```

For a specific length of, say `32`, use:
```bash
randompass --length 32
```

For full options, run:
```bash
randompass --help
```

### Docker

`aarch64` and `amd64` dockers are available.

Run the following command to pull the image and just generate a random password.

```bash
docker run --rm mihaigalos/randompass
```

### Similar work

[`pass-rs`](https://github.com/Jarusk/pass-rs), [`randpas`](https://github.com/ProCode2/randpas), [`randompassword`](https://github.com/pshc/randompassword).
