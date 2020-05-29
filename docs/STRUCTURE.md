
# File Structure of rav1e 0.4.0

<details>
<summary><b>Table of Content</b></summary>

- [High-level directory structure](#high-level-directory-structure)
- [Overview of `src/*`](#overview-of-src)
</details>

## High-level directory structure

![Image](structure.png)


##  Overview of `src/*`

The below table gives a brief overview of design of [`src/*`](https://github.com/xiph/rav1e/tree/master/src/)

| Filename                                                                                     | Functionality                                                                                              |
| -------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| [activity.rs](https://github.com/xiph/rav1e/tree/master/src/activity.rs)                     | Implementation of Activity masking functions for planes                                                    |
| [api/*.rs](https://github.com/xiph/rav1e/tree/master/src/api/)                               | Contains public API of rav1e, for more information check [documentation](https://docs.rs/rav1e/)           |
| [arm/32/*.S](https://github.com/xiph/rav1e/tree/master/src/arm/32/)                          | ARM optimised functions for different encoding tools imported from dav1d by release                        |
| [arm/64/*.S](https://github.com/xiph/rav1e/tree/master/src/arm/64)                           | AArch64 optimised functions for different encoding tools imported from dav1d by release                    |
| [arm/asm.S](https://github.com/xiph/rav1e/tree/master/src/arm/asm.S)                         | Common functions used for Assembly implementation                                                          |
| [arm/table.S](https://github.com/xiph/rav1e/tree/master/src/arm/table.S)                     | Tables for various ARM optimised functions                                                                 |
| [asm/\*/*.rs](https://github.com/xiph/rav1e/tree/master/src/asm/)                            | High-level functions for binding rust and assembly functions for x86 and AArch64 Architecture              |
| [bin/common.rs](https://github.com/xiph/rav1e/tree/master/src/bin/common.rs)                 | Functions, enums, structures used command-line tool and debugging                                          |
| [bin/rav1e.rs](https://github.com/xiph/rav1e/tree/master/src/bin/rav1e.rs)                   | CLI Interface for encoding from y4m files with rav1e                                                       |
| [bin/stats.rs](https://github.com/xiph/rav1e/tree/master/src/bin/stats.rs)                   | Functions for displaying Frame summary, progress info, metrics of the encoding process                     |
| [bin/kv.rs](https://github.com/xiph/rav1e/tree/master/src/bin/kv.rs)                         | Serialisation configuration of Key-value strings                                                           |
| [bin/errror.rs](https://github.com/xiph/rav1e/tree/master/src/bin/error.rs)                  | Functions and enums to parse various errors and displaying                                                 |
| [bin/muxer/*.rs](https://github.com/xiph/rav1e/tree/master/src/bin/muxer/)                   | Contains IVF Muxer functions for header definition, writing frames and flushing                            |
| [bin/decoder/*.rs](https://github.com/xiph/rav1e/tree/master/src/bin/decoder/)               | Decoder related structures and functions                                                                   |
| [capi.rs](https://github.com/xiph/rav1e/tree/master/src/capi.rs)                             | C Compatible API for using rav1e as a library                                                              |
| [cdef.rs](https://github.com/xiph/rav1e/tree/master/src/cdef.rs)                             | CDEF Filter implementation for the encoder                                                                 |
| [context.rs](https://github.com/xiph/rav1e/tree/master/src/context.rs)                       | High-level functions that write symbols to the bitstream, and maintain context                             |
| [cpu_features/*.rs](https://github.com/xiph/rav1e/tree/master/src/cpu_features.rs)           | Functions to toggle CPU optimisations for different architectures                                          |
| [deblock.rs](https://github.com/xiph/rav1e/tree/master/src/deblock.rs)                       | Deblocking loop filter implementation for addressing blocking artifacts                                    |
| [dist.rs](https://github.com/xiph/rav1e/tree/master/src/dist.rs)                             | SAD and SATD functions and implementation for various encoder functions                                    |
| [ec.rs](https://github.com/xiph/rav1e/tree/master/src/ec.rs)                                 | Low-level implementation of the entropy coder, which directly writes the bitstream                         |
| [encoder.rs](https://github.com/xiph/rav1e/tree/master/src/encoder.rs)                       | Low-level implementation of the AV1 encoder tools functions and structures                                 |
| [entropymode.rs](https://github.com/xiph/rav1e/tree/master/src/entropymode.rs)               | Low-level implementation of entropy mode                                                                   |
| [ext/x86/x86inc.asm](https://github.com/xiph/rav1e/tree/master/src/ext/ext86/x86inc.asm)     | X86 Assembly header providing an easier way between different calling conventions (x86_32, win64, linux64) |
| [frame/*.rs](https://github.com/xiph/rav1e/tree/master/src/frame/)                           | Misc encoder specific frame and plane enums apart                                                          |
| [fuzzing.rs](https://github.com/xiph/rav1e/tree/master/src/fuzzing.rs)                       | Functions to initialise fuzz targets for encoder process                                                   |
| [header.rs](https://github.com/xiph/rav1e/tree/master/src/header.rs)                         | The enums and structs of bitstream headers for writing                                                     |
| [lib.rs](https://github.com/xiph/rav1e/tree/master/src/lib.rs)                               | The top level library, contains code to write headers, manage buffers, and iterate through each superblock |
| [lrf.rs](https://github.com/xiph/rav1e/tree/master/src/lrf.rs)                               | Low-level implementation of Loop restoration filter                                                        |
| [mc.rs](https://github.com/xiph/rav1e/tree/master/src/mc.rs)                                 | Low-level implementation of Motion Compensation of the encoding process                                    |
| [me.rs](https://github.com/xiph/rav1e/tree/master/src/me.rs)                                 | Motion Estimation related structures and functions of the encoding process                                 |
| [partition.rs](https://github.com/xiph/rav1e/tree/master/src/partition.rs)                   | Functions and enums to manage partitions (subdivisions of a superblock)                                    |
| [predict.rs](https://github.com/xiph/rav1e/tree/master/src/predict.rs)                       | Intra and inter prediction implementations                                                                 |
| [quantize.rs](https://github.com/xiph/rav1e/tree/master/src/quantize.rs)                     | Quantization and dequantization functions for coefficients                                                 |
| [rate.rs](https://github.com/xiph/rav1e/tree/master/src/rate.rs)                             | Low-level implementation of rate control API (Constant Quantizer)                                          |
| [rdo.rs](https://github.com/xiph/rav1e/tree/master/src/rdo.rs)                               | RDO-related structures and distortion computation functions                                                |
| [rdo_tables.rs](https://github.com/xiph/rav1e/tree/master/src/rdo_tables.rs)                 | Set of RDO rate values used for RDO related calculation                                                    |
| [recon_intra.rs](https://github.com/xiph/rav1e/tree/master/src/recon_intra.rs)               | Functions used for directional intra-prediction modes                                                      |
| [scan_order.rs](https://github.com/xiph/rav1e/tree/master/src/scan_order.rs)                 | Functions definitions for various block-level scan orders                                                  |
| [screenchange/*.rs](https://github.com/xiph/rav1e/tree/master/src/screenchange/)             | Low-level implementation of fast screen-cut detection b/w frames for adaptive keyframe selection           |
| [segmentation.rs](https://github.com/xiph/rav1e/tree/master/src/segmentation.rs)             | Top-level implementation of segmentation index coding                                                      |
| [test_encode_decode/*.rs](https://github.com/xiph/rav1e/tree/master/src/test_encode_decode/) | Various encoder-decoder tests using dav1d and aom                                                          |
| [tiling/*.rs](https://github.com/xiph/rav1e/tree/master/src/tiling/)                         | Implementation of tiling during encoding                                                                   |
| [token_cdfs.rs](https://github.com/xiph/rav1e/tree/master/src/token_cdfs.rs)                 | Token cdf header for entropy mode                                                                          |
| [transform/*.rs](https://github.com/xiph/rav1e/tree/master/src/transform)                    | Implementations of DCT and ADST transforms                                                                 |
| [util/*.rs](https://github.com/xiph/rav1e/tree/master/src/util/)                             | Misc utility code                                                                                          |
| [x86/*.rs](https://github.com/xiph/rav1e/tree/master/src/x86)                                | X86 optimised functions for various encoder functions along with functions imported from dav1d by release  |

