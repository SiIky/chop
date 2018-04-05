# `chop`

Chop long lines to fit in the terminal, either from `stdin`
or files passed as arguments.

```sh
chop file1 file2
cat file1 file2 | chop
dmesg | chop
```

When `chop` doesn't play well (such as with `watch(1)`),
you can specify a width manually.

```sh
cat file1 file2 | chop -60
chop -60 file1 file2 -30 file3 file4
for I in `seq 1 5`; do echo hello | chop -$I; done
```

`chop` is Unicode aware (thanks to Rust and [unicode-width][0])
and calculates the real width, not length in bytes.

[0]: https://github.com/unicode-rs/unicode-width
