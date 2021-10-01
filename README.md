# Of Course

[![GitMoji](https://img.shields.io/badge/Gitmoji-%F0%9F%8E%A8%20-FFDD67.svg)](https://gitmoji.dev)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
![Lines Of Code](https://img.shields.io/tokei/lines/github.com/UltiRequiem/ofCourse?color=blue&label=Total%20Lines)

Output a string repeatedly until killed.

Yet Another [`yes`](https://github.com/openbsd/src/blob/master/usr.bin/yes/yes.c) clone but in Rust.

## Usage

Just like [`yes`](<https://en.wikipedia.org/wiki/Yes_(Unix)>):

```bash
yeah
```

> This will print "y" until the process is canceled.

You can also pass a custom string to repeat:

```bash
yeah custom string
```

> This will print "custom string" until the process is canceled.

### License

This project is licensed under the [MIT License](./LICENSE.md).
