# `ta-lib`

High level Rust wrapper around the Technical Analysis library.

This crate only exposes the functions I need myself from the original TA lib.
Which has around 200 of them. Feel free to expose more (easy via the included
macros) and open a PR.

I sugget using the [`ta`](https://crates.io/crates/ta/) crate instead, which is
a rewrite in Rust of *some* parts of TA but with better ergonomics than this
crate here.

However, if you quickly need to use an indicator missing from `ta`, this crate
may get you there quickly, as a workaround.