// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// When denying at the crate level, be sure to not get random warnings from the
// injected intrinsics by the compiler.

#![deny(attribute_usage)]
#![deny(unused_attribute)]
#![allow(dead_code)]

#[abi="stdcall"] extern {} //~ ERROR: obsolete attribute
                           //~^ ERROR: unused attribute

#[fixed_stack_segment] fn f() {} //~ ERROR: obsolete attribute
                                 //~^ ERROR: unused attribute

fn main() {}
