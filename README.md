# randompass

A simple static password generator.
Generated passwords are 16 characters, lower and uppercase and contain special characters by default.

### Why?

I got frustrated with generating passwords which met the required complexity.

There are a few implementations around, but their licensing meant they cannot be used in a professional setting.

Moreover, they require explicit flags to _enable_ a specific complication (i.e.: special chars, uppercase); `randompass` has them enabled by default and the user can _disable_ them instead.

Finally, other implementations do not guarantee that the required complexity has been met (i.e.: the password might not contain a required complication).

### Usage

```bash
randompass
```

For a specific length of, say `32`, use
```bash
randompass --length 32
```

For full options, run:
```bash
randompass --help
```

### Similar work

[`pass-rs`](https://github.com/Jarusk/pass-rs), [`randpas`](https://github.com/ProCode2/randpas), [`randompassword`](https://github.com/pshc/randompassword).