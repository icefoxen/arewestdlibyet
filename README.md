A list of useful and relatively mature crates for various tasks, for you need a de-facto-standard way
of doing stuff.

Basically, if there's an obvious answer to a common "how do I do X" question, or if someone might ask
"Why isn't X in std?", X should go here.  This shouldn't become [awesome-rust](https://github.com/kud1ing/awesome-rust), we have awesome-rust for that.

Rules:

* At most two crates that do more or less the same thing
* If there's no halfway decent docs and readme, leave it out
* If it's not a go-to solution for the task it's doing, leave it out
* If you're not sure, leave it out

To do: Short descriptions, links to crates, note whether or not it is pure rust, better tables...

If you have a suggestion, amendment, flames, or other communication, open an issue.  Or better, a pull
request.

# Async I/O

* futures-rs
* mio

# Bit mongling/endianness stuff

* bitflags
* byteorder

# Command line parsing

* argparse
* clap

# Compression

* bzip2
* flate2
* zip

# Control flow abstraction

* itertools

# Cryptography

* openssl
* ring (instead of openssl?)
* crypto-hashes

# Data structures

* arrayvec
* ndarray
* phf
* typemap
* petgraph
* smallvec

# File/directory stuff

* seek_bufread
* tempdir


# Graphics

## Low-level

* gfx
* glium

## Higher-level

* conrod?

## Windowing/context creation

* glfw
* glutin
* sdl2

## Font loading/rendering

* rusttype

## Misc

* genmesh

# Image loading/saving

* image

# Kitchen sink

* monster
* odds

# Languages

## Lua

* hlua

## Other

* dyon
* gluon

# Logging

* emit
* env_logger
* fern
* log
* loggerv
* mowl
* simplelog
* slog
* ulog

# Math

* num (traits, complex/bigint/rational number types, iterators, all sorts of nice stuff)

## Numerical traits/generics

* try_from

## Vector algebra/computer graphics/physics

* nalgebra
* cgmath

## Misc

* noise

# Memory management

* typed_arena

# Misc stuff

* semver

# Parsing/formatting

## JSON

* json
* serde_json

## TOML

* toml

## Parser generators

# Platform-specific interfacing

## Windows

* winapi

## Linux

## Mac

## Misc

* num_cpus

# Parallelism

* threadpool

# Random numbers

* rand

# Reference/lifetime mongling

* lazy_static
* owning_ref
* rental
* thread_local

# Serialization

* rustc_serialize
* serde

# Sound

## Decoding

## Playing

* alto
* ears

# Text/string/templating stuff

* handlebars
* regex
* symtern
* tera

# Text/terminal user interface

* rustyline
* termion

# Time

* chrono
* time

# Web stuff

## HTTP libraries

* hyper
* solicit

## Web frameworks

* iron
* pencil

## Misc

* cookie
* formdata
* idna

