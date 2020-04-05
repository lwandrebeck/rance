<p align="center">
  <img src="https://raw.github.com/lwandrebeck/rance/master/rance.png" width="50%"/>
</p>

# rance: ansible in rust

[![Coverage Status](https://coveralls.io/repos/github/lwandrebeck/rance/badge.svg?branch=master)](https://coveralls.io/github/lwandrebeck/rance?branch=master)
[![codecov](https://codecov.io/gh/lwandrebeck/rance/branch/master/graph/badge.svg)](https://codecov.io/gh/lwandrebeck/rance)
[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Flwandrebeck%2France%2Fbadge%3Fref%3Dmaster&style=flat)](https://actions-badge.atrox.dev/lwandrebeck/rance/goto?ref=master)
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Minimum rustc version](https://img.shields.io/badge/rust-nightly20200121%2B-informational)](#rust-version-requirements)
[![dependency status](https://deps.rs/repo/github/lwandrebeck/rance/status.svg)](https://deps.rs/repo/github/lwandrebeck/rance)

## Quick introduction

rance aims to be (maybe one day) a (faster) clone of ansible.

Right now, rance does almost nothing. Skeleton is being built. It will (quite) soon relie on ansible to do work, and then modules will be converted one after each other.

## TODO:

(I won’t list every ansible modules (3387+ to this date), this README would be dead long)

- [x] Similar CLI as the one of ansible 2.9.6
- [ ] tweak files in etc to stick to rance
- [ ] Write documentation
- [ ] Write tests
- [ ] Packaging (rpm, deb, others ?)
- [ ] Github tweaking (issues labels, template…)
- [ ] Github actions tweaking (ms and mac (and others ?) builds to be added)
- [ ] Take care of config files that can be in JSON, TOML, YAML, HJSON, INI (partly done)
- [ ] Add sftp/scp/ssh mechanism
- [ ] Use pyO3 to link to ansible
- [ ] Everything
- [ ] and the rest.

## Building

You need Rust 1.42.0-nightly 2020-01-21 or newer due to PyO3
`rustup override set nightly` in rance directory and you should be good to go.

It’s as simple as:

    cargo build

## Running
    cargo run
