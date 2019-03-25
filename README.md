# slice-ext

Slice extensions for rust, this is intended to be extended as more useful primitives are identified.

## Status

[![GitHub tag](https://img.shields.io/github/tag/ryankurte/rust-slice-ext.svg)](https://github.com/ryankurte/rust-slice-ext)
[![Build Status](https://travis-ci.com/ryankurte/rust-slice-ext.svg?branch=master)](https://travis-ci.com/ryankurte/rust-slice-ext)
[![Crates.io](https://img.shields.io/crates/v/slice-ext.svg)](https://crates.io/crates/slice-ext)
[![Docs.rs](https://docs.rs/slice-ext/badge.svg)](https://docs.rs/slice-ext)

Work in progress, would be good to genericise over `AsRef<T>` instead of `&'a [T]`, should work tho.

## Features
- split_before to split using a predicate while including the matched item at the start of each subslice if found
- split_after to split using a predicate while including the matched item at the end of the subslice if found
