# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.4](https://github.com/BobG1983/rantz_random/compare/v0.4.3...v0.4.4) - 2024-06-19

### Other
- Updated CI to use windows and caching of build artifacts

## [0.4.3](https://github.com/BobG1983/rantz_random/compare/v0.4.2...v0.4.3) - 2024-06-19

### Other
- Reverting version change so that release-plz can handle it
- Added release-plz
- RandomContainer and RandomWeightedContainer now return Options to better handle empty tables
- Fixed more doc comments.
- Fixed docs
- WeightedTable IntoIter now iters over values only. Shuffle now depends on RandomContainer, and RandomContainer was expanded.
