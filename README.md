# My Personal Playground for MIRAI

MIRAI is a static analyzer for Rust, but it is still experimental and contains a lot of bugs, especially with its heap model. This repo is built for finding and / or validating some potential bugs and interesting issues about MIRAI checker. Currently, I am focusing on `std::vec::Vec`.

MIRAI works very badly on vector types, possibly because std has a lot of traits and implementations that cannot be fully verified by MIRAI? I am afraid tools from std won't work very well, either.

## MIRAI version

https://github.com/facebookexperimental/MIRAI at branch `new_nightly`.
