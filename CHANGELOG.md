# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4](https://github.com/KarpelesLab/cacrt/compare/v0.1.3...v0.1.4) - 2026-10-23

### Other

- Add a blocklist covering disabled roots and compromised intermediates ([#11](https://github.com/KarpelesLab/cacrt/pull/11))
- Archive compromised eMudhra intermediates and their private keys as evidence ([#10](https://github.com/KarpelesLab/cacrt/pull/10))
- Document root cause of eMudhra intermediate compromise: trivially small private keys ([#8](https://github.com/KarpelesLab/cacrt/pull/8))

## [0.1.3](https://github.com/KarpelesLab/cacrt/compare/v0.1.2...v0.1.3) - 2026-10-20

### Other

- Disable emSign ECC Root CA - G3 after intermediate key compromise ([#6](https://github.com/KarpelesLab/cacrt/pull/6))

## [0.1.2](https://github.com/KarpelesLab/cacrt/compare/v0.1.1...v0.1.2) - 2026-06-05

### Other

- Fix stale subject-hash headers; guard + example.com/.org anchor test ([#5](https://github.com/KarpelesLab/cacrt/pull/5))
- Remove Cargo.lock from version control ([#4](https://github.com/KarpelesLab/cacrt/pull/4))
- Forbid unsafe code across all package targets ([#2](https://github.com/KarpelesLab/cacrt/pull/2))

## [0.1.1](https://github.com/KarpelesLab/cacrt/compare/v0.1.0...v0.1.1) - 2026-06-04

### Other

- Add Mozilla Root Store Policy and CA change-request links to CURATION.md
- Add README badges (crates.io, docs.rs, CI, license) and fix repo URL
