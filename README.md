rndtester
=========

Crate rndtester is a collection of Rust functions to support the generation
of random unit tests. The objective of this crate was to be used in the
context of some task assignments. In case you are looking for good 
random testing tool for Rust, please consider to use QuickCheck 
( https://github.com/BurntSushi/quickcheck ) instead. 

### Installation

To add the rndtester to your crate, just add the following
dependency to your Cargo.toml file:

```toml
[dependencies.rndtester]
git = "https://github.com/christoffetzer/rndtester"
```

### Documentation


After adding rndtester to your crate, generate the documentation with
cargo by executing "cargo doc". The documention will, as always, be in 
your target/doc directory.

To see how rndtester can be used, check out crate 'sort' ( see 
git = "https://github.com/christoffetzer/sort" ).



