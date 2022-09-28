# llvm-stackmap
This is a library that can be used to parse stack maps emitted by the experimental [stack maps feature](https://llvm.org/docs/StackMaps.html) provided by LLVM

## Example
Parsing a stack map embedded into the binary `objdump` can be done via the following snippet:
```rust
use llvm_stackmap::StackMap;
use std::path::PathBuf;

let path_to_elf = PathBuf::from_str("objdump").unwrap();
let sm = StackMap::from_path(path_to_elf).unwrap();
```
