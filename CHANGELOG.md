# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-XX-XX

### Added
- GitHub Releases with pre-built binaries for Linux x64 and ARM64
- Automated CI/CD pipeline with GitHub Actions
- Remote installation support via `curl | bash`
- FFI library (`libkitchn_ffi.so`) included in release packages
- C header (`kitchn.h`) for FFI bindings

### Changed
- Install script now supports three modes: source, package, and remote
- Release packages include default configuration files

### Fixed
- TBD

## [0.1.0] - Initial Release

### Added
- Core `kitchn` CLI for theme and ingredient management
- `kitchn-log` utility for structured logging
- Sweet Dracula color theme
- TOML-based configuration system
- FFI bindings for C/C++ and Python
- Ingredient templating with Tera
