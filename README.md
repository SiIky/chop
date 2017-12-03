# `chop`

Chop long lines to fit in the terminal, either from `stdin`
or files passed as arguments.

```sh
chop file1 file2
cat file1 file2 | chop
dmesg | chop
```

`chop` is Unicode aware (thanks to Rust and [unicode-width][0])
and calculates the real width, not length in bytes.

[0]: https://github.com/unicode-rs/unicode-width
