[![Build Status](https://travis-ci.org/lwandrebeck/rance.svg?branch=master)](https://travis-ci.org/lwandrebeck/rance)
[![Coverage Status](https://coveralls.io/repos/github/lwandrebeck/rance/badge.svg?branch=master)](https://coveralls.io/github/lwandrebeck/rance?branch=master)
[![codecov](https://codecov.io/gh/lwandrebeck/rance/branch/master/graph/badge.svg)](https://codecov.io/gh/lwandrebeck/rance)
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.37+-blue.svg)](#rust-version-requirements)
[![dependency status](https://deps.rs/repo/github/lwandrebeck/rance/status.svg)](https://deps.rs/repo/github/lwandrebeck/rance)

<p align="center">
  <img src="https://raw.github.com/lwandrebeck/rance/master/rance.png" width="50%"/>
</p>

# rance: ansible in rust

## Quick introduction

rance aims to be (maybe one day) a (faster) clone of ansible.

Right now, rance is only a wrapper launching ansible, waiting for equivalent modules to be written in rust.

## TODO:

(I won’t list every ansible module, this README would be dead long)
[x] Similar CLI as the one of ansible 2.9.6
[ ] Everything
[ ] and the rest.

## Building

You need Rust 1.42.0-nightly 2020-01-21 or newer due to PyO3
`rustup override set nightly` in rance directory and you should be good to go.

It’s as simple as:

    cargo build

## Running
    cargo run

## To quit
    ctrl+d
