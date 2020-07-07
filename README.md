# BulkSSRF (bssrf)
![release](https://github.com/junnlikestea/bulkssrf/workflows/release/badge.svg)
[![Build status](https://github.com/junnlikestea/bulkssrf/workflows/Continuous%20integration/badge.svg)](https://github.com/junnlikestea/bulkssrf/actions)

This is a Rust port of [m4ll0k](https://twitter.com/m4ll0k2) `ssrf.py` script. 

### Installation
Precompiled binaries for bssrf are available in the [releases](https://github.com/junnlikestea/bulkssrf/releases) tab. Just pick your platform and extract the archive that contains the binary.

## Building it yourself 
If you want to build it yourself you will need to install Rust, you can get the [official installation](https://www.rust-lang.org/tools/install) from the Rust website.

To build BulkSSRF:
```
$ git clone https://github.com/junnlikestea/bulkssrf
$ cd bulkssrf
$ cargo build --release
$ ./target/release/bssrf --version
```

### Usage
With a list of urls in a file:
```
bssrf -f path/to/file.txt -l xyzburpcollaborator.com
```

Using a tool like `gau` to feed bssrf urls:
```
gau hackerone.com | bssrf -l xyzburpcollaborator.com
```

By default the program has timeout set to 5 seconds, but you can alter that with the `-t` flag, you can also add various debugging
info with the `-v` for verbose flag:
```
gau hackerone.com | bssrf -v -t 10 -l xyzburpcollaborator.com 
```

If you want to feed the traffic through an http proxy you can use the `-p` or `-replay-proxy` flag.
```
gau hackerone.com | bssrf -l xyzburpcollaborator.com -p http://127.0.0.1:8080
```

### Common error
BulkSSRF uses async concurrent http requests under the hood. If you encounter an error similar to "Too many open files" it means that there isn't enough available file descriptors on your system. You can fix this by increasing the limits available. There are lots of different guides available to increase the limits, [but here is one for linux](https://www.tecmint.com/increase-set-open-file-limits-in-linux/). I actually ran into this problem while writing the code.

### Disclaimer
Developers have/has no responsibility or authority over any kind of:
* Legal or Law infringement by third parties and users.
* Malicious use capable of causing damage to third parties.
* Illegal or unlawful use of bulkssrf.

Thanks to [0xatul](https://twitter.com/atul_hax) for the feedback!
