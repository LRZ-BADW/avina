<div align="center">

# avina

![](https://raw.githubusercontent.com/LRZ-BADW/avina/main/logo.png)

API bindings, CLI application, API-backend, and WASM web UI written in Rust for
LRZ-specific features of the Openstack-based LRZ Compute Cloud,
[https://cc.lrz.de](https://cc.lrz.de), first and foremost the budgeting
system.

[![audit](https://github.com/LRZ-BADW/avina/actions/workflows/audit.yml/badge.svg)](https://github.com/LRZ-BADW/avina/actions/workflows/audit.yml)
[![lint](https://github.com/LRZ-BADW/avina/actions/workflows/lint.yml/badge.svg)](https://github.com/LRZ-BADW/avina/actions/workflows/lint.yml)
[![test](https://github.com/LRZ-BADW/avina/actions/workflows/test.yml/badge.svg)](https://github.com/LRZ-BADW/avina/actions/workflows/test.yml)

![Crates.io](https://img.shields.io/crates/l/avina?link=https://crates.io/crates/avina)

</div>

## CLI Installation
See: [cli/README.md](./cli/README.md)

## Crates
- [api](api): backend API server application ![crates.io](https://img.shields.io/crates/v/avina-api?link=https://crates.io/crates/avina-api)
- [cli](cli): client-side CLI application ![crates.io](https://img.shields.io/crates/v/avina-cli?link=https://crates.io/crates/avina-cli)
- [lib](lib): client-side API-binding library ![crates.io](https://img.shields.io/crates/v/avina?link=https://crates.io/crates/avina)
- [test](test): shared test helpers and end-to-end tests
- [wire](wire): library for shared data structures used for API communication ![crates.io](https://img.shields.io/crates/v/avina-wire?link=https://crates.io/crates/avina-wire)
- [ui](ui): fullstack WASM web user interface ![crates.io](https://img.shields.io/crates/v/avina-ui?link=https://crates.io/crates/avina-ui)
