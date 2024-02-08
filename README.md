[![ci](https://github.com/kbujari/icer/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/kbujari/icer/actions/workflows/ci.yml) [![license](https://img.shields.io/github/license/kbujari/icer)](https://opensource.org/licenses/MIT)

# The ICER Image Compressor

**(WIP)** Implementation of the ICER Progressive Wavelet Image Compressor [published by NASA](https://ipnpr.jpl.nasa.gov/progress_report/42-155/155J.pdf), used for the Mars Exploration Rover.

## Features

### Algorithm
- Error correction optimized for deep-space channels
- Progessive compression leads to graceful image degradation compared to other techniques
- Designed to only use integer arithmetic

### Implementation
- Uses the [`image`](https://github.com/image-rs/image) crate for converting between image formats
- Currently works with grayscale images, maybe RGB later
- Minimal allocations
- 100% safe Rust
