# lzc-sdk-rs

The unofficial Rust SDK of [LazyCat](https://lazycat.cloud) generated from:

- https://gitee.com/linakesi/lzc-boxservice-protos.git
- https://gitee.com/linakesi/lzc-sdk.git

## Development

You have to install `potobuf-compiler` before you developing or contributing to this crate.

And then, run `prepare.sh`.

## Patches

This crate patched some function name for avoid naming conflict:

- `cloud.lazycat.apis.sys.rs#connect` -> `cloud.lazycat.apis.sys.rs#connect_led`

## Copyrights

Because upstream dependencies doesn't have `LICENSE`, this crate only for learning and self using.

All rights reserved by LazyCat Inc,.
