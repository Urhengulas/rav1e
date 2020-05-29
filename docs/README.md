# rav1e docs

<details>
<summary><b>Table of Content</b></summary>

- [`../README.md`](#readmemd)
- [`CODING_STYLE.md`](#coding_stylemd)
- [`FRAME_TYPE_SELECTION.md`](#frame_type_selectionmd)
- [`GLOSSARY.md`](#glossarymd)
- [`PROFILING.md`](#profilingmd)
- [`regress_log-bitrate_wrt_log-quantizer.ipynb`](#regress_log-bitrate_wrt_log-quantizeripynb)
- [`STRUCTURE.md`](#structuremd)
- [`VERSIONING`](#versioning)
</details>

## [`../README.md`](https://github.com/xiph/rav1e/tree/master/README.md)
Main README of rav1e:
- Overview
- Features
- Releases
- Windows builds
- Building
- Compressing video
- Decompressing video
- Configuring
- Using the AOMAnalyzer
- Design
- Contributing
- Getting in Touch

## [`CODING_STYLE.md`](CODING_STYLE.md)

## [`FRAME_TYPE_SELECTION.md`](FRAME_TYPE_SELECTION.md)
- Current Features/Process
- Detection Algorithm
- Desired Improvements

## [`GLOSSARY.md`](GLOSSARY.md)
Explaination of various special terms.

## [`PROFILING.md`](PROFILING.md)
List of various prfiling tools:
- Cargo integrations
- Generic profiling
- Tracing
- Codegen Inspection

## [`regress_log-bitrate_wrt_log-quantizer.ipynb`](regress_log-bitrate_wrt_log-quantizer.ipynb)

## [`STRUCTURE.md`](STRUCTURE.md)
- High-level directory structure
- Overview of `src/*`

## `VERSIONING`
rav1e follows Cargo's versioning scheme: https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field

Because rav1e is not yet at version 1.0.0, all changes that break the API require a minor-version bump.

The API is defined as:
- public functions in src/api.rs
- command line parameters to the rav1e binary
