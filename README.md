rust-minilzo
---

A pure Rust implementation of the [miniLZO](http://www.oberhumer.com/opensource/lzo/)
compression library. Initially tranlated using [c2rust](https://github.com/immunant/c2rust)
but refactored to meet certain requirements, and will continue to be a pet-project to 
better understand both LZO and de/compression methodology; _long term_ goal to continue
refactoring into a 100% safe rust implementation, but for now, there is _a lot_ of 
unsafe usage in this project.
