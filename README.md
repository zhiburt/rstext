[![Crate](https://img.shields.io/crates/v/rstext)](https://crates.io/crates/rstext)

# rstext

Rust [GNU gettext](http://www.gnu.org/software/gettext/) utilities crate.
Inspired by [gotext](https://github.com/leonelquinteros/gotext) library.

## Overview

The library tries to be as lazy as possible in regard to reduce usage of memory.
Therefore if you didn't use some locale it won't be loaded into the memory.
The same goes with domains.

* There's no support of `.mo` files yet.
* There's no global function for working with the library as if you would work with virtual C library.

## Usage

The library automatically simplifies locales if the specific one is not found. The language codes are assumed to be [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) codes (2-letter codes).  

```rust
    let mut locale = rstext::Locale::new("example_locales", "en_UK")?;
    let domain = locale.load("domain1")?;
    let greeting = domain.get("greeting").map_or("greeting not found", |g| g);

    println!("{:?}", greeting);
```
