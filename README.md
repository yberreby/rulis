# rulis

`rulis` is a tiny Lisp interpreter based on [Build Your Own
Lisp](http://www.buildyourownlisp.com/)'s grammar. It comes with a very simple
REPL that you can run with `cargo run`.

The interesting parts:

- `rulis` uses [LALRPOP](https://github.com/nikomatsakis/lalrpop) for parsing.
- It only took about 1 1/2 hour and 128 lines of code (including the LALRPOP
  grammar) to go from nothing to a working albeit stripped down REPL, a success
  which I mostly attribute to LALRPOP and Rust's pattern matching.

## License

Licensed under either of

 * Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.