[![ci](https://github.com/kbzt/icer/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/kbzt/icer/actions/workflows/ci.yml) [![license](https://img.shields.io/github/license/kbzt/icer)](https://opensource.org/licenses/MIT)

# The ICER Image Compressor

**(WIP)** Implementation of the ICER Progressive Wavelet Image Compressor [published by NASA](https://ipnpr.jpl.nasa.gov/progress_report/42-155/155J.pdf) used for the Mars Exploration Rover.

This repository is split into two projects:

| Crate       | Description                                                                    |
|-------------|--------------------------------------------------------------------------------|
| **libicer** | Library for using this ICER implentation with your own Rust application.       |
| **icer**    | CLI-based reference application using the libicer crate.                       |

## Features

### Algorithm
- Error correction optimized for deep-space channels
- Progessive compression leads to graceful image degradation compared to other techniques
- Designed to only use integer arithmetic

### Implementation
- Uses the `image` Rust crate to support [various image types](https://github.com/image-rs/image/blob/master/README.md#supported-image-formats)
- Currently works with grayscale images, RGB support planned for the future
- Only stable and `safe` Rust, with complete error handling

## Disclaimer

This is a WIP implementation and thus will have breaking changes until the `v1.0.0` release.
