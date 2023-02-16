
#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use std::{collections::HashMap, error::Error};

#[macro_use] mod macros;
mod lib;

macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let x: Vec<u32> = vec![1, 2, 3];
}

