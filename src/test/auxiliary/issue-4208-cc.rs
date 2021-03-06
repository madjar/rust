// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[link(name = "numeric",
       vers = "0.1")];
#[crate_type = "lib"];

pub trait Trig<T> {
    fn sin(&self) -> T;
}

pub fn sin<T:Trig<R>, R>(theta: &T) -> R { theta.sin() }

pub trait Angle<T>: Trig<T> {}
