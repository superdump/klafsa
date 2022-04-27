# klafsa

`klafsa` is a quick hack but hopefully useful tool for mass converting textures in glTF files from JPEG/PNG to compressed formats using external tools such as:
- `basisu` from [Binomial LLC's basis_universal](https://github.com/BinomialLLC/basis_universal)
- `kram` from [kram's GitHub repository](https://github.com/alecazam/kram)
- `toktx` from [the Khronos Group's KTX-Software](https://github.com/KhronosGroup/KTX-Software)

'klafsa' means something like 'squelch' in Swedish. I wanted to use a word that has perhaps not been used for texture compression tools previously, and the space of English words is rather heavily used already.

## Setup

Make sure one of the above tools is in your `PATH`. Usual Rust cargo tooling can be used to build / install `klafsa` and then command-line usage is as follows:

```
klafsa 0.1.0
Texture compression tool for converting JPEG/PNG to various compressed formats

USAGE:
    klafsa [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -b, --backend <BACKEND>        Which tool to use for compression [default: to-ktx] [possible
                                   values: basis-u, kram, to-ktx]
        --codec <CODEC>            Which codec to use for compression [possible values: astc,
                                   astc4x4, astc5x5, astc6x6, astc8x8, bc1, bc3, bc4, bc5, bc7,
                                   etc1s, etc2-r, etc2-rg, etc2-rgb, etc2-rgba, uastc]
        --container <CONTAINER>    Which container format to use [possible values: ktx2]
    -h, --help                     Print help information
    -V, --version                  Print version information

SUBCOMMANDS:
    gltf    Converts all JPEG/PNG textures referred to by a JSON-format glTF
    help    Print this message or the help of the given subcommand(s)
```

The following would parse `model.gltf` to identify textures compressed with JPEG/PNG and use the `kram` tool to convert them to `bc7` in `ktx2`, also outputting a `model_bc7_ktx2.gltf` file next to the original:
```
klafsa gltf --backend kram --codec bc7 --container ktx2 /path/to/model.gltf
```

## TODO

- [x] all codecs and containers for each backend
  - [x] basisu
  - [x] kram
  - [x] toktx
- [ ] in-process compression, as-in without spawning separate processes to allow usage in more online use cases?
- [ ] glb / embedded glTF support
- [ ] support converting individual image files outside of a gltf

## License

`klafsa` is free and open source! All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](docs/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](docs/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
