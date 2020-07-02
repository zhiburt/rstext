# rstext
Rust [GNU gettext](http://www.gnu.org/software/gettext/) utilities crate.
Inspired by [gotext](https://github.com/leonelquinteros/gotext) library.

## Overview

The library tries to be as lazy as possible in regard to reduce usage of memory.
Therefore if you didn't use some locale it won't be loaded into the memory.
The same goes with domains.

There's no support of `.mo` files yet.
There's no global function for working with the library as if you would work with virtual C library.

## Usage

```rust
    let mut locale = Translator::new("example_locales")?;
    let domain = locale.domain("domain1")?;
    let locale = domain.locale("de")?;
    let greeting = locale.get("greeting");

    println!("{:?}", greeting);
```
