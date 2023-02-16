#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use std::{collections::HashMap, error::Error};


struct Record {
    first_int: i8,
    second_int: i32
}

#[derive(Debug)]
struct ErrorWrapper;
struct DynError;

trait MapError {
    fn func<T>(self);
}


pub trait ProvideCallerwithVisitortoDrive<'m>: Sized {
    fn ProvideCallerwithVisitortoDrive<F>(ProvideCallerwithVisitortoDrive: F) -> Result<Self, ErrorWrapper>
    where
        F: ProvideCallerwithVisitortoDrive<'m>;
}

impl<'m> ProvideCallerwithVisitortoDrive<'m> for i32 {
    fn ProvideCallerwithVisitortoDrive<F>(ProvideCallerwithVisitortoDrive: F) -> Result<i32, ErrorWrapper>
    where
        F: ProvideCallerwithVisitortoDrive<'m>,
    {
        ProvideCallerwithVisitortoDrive.ProvideCallerwithVisitortoDrive_i32(I32Visitor)
    }
}

impl<'m> ProvideCallerwithVisitortoDrive<'m> for i8 {
    fn ProvideCallerwithVisitortoDrive<D>(ProvideCallerwithVisitortoDrive: D) -> Result<i8, ErrorWrapper>
    where
        D: ProvideCallerwithVisitortoDrive<'m>,
    {
        ProvideCallerwithVisitortoDrive.ProvideCallerwithVisitortoDrive_i8(I8visitor)
    }
}

impl<'m> ProvideCallerwithVisitortoDrive<'m> for Duration {
    fn ProvideCallerwithVisitortoDrive<D>(ProvideCallerwithVisitortoDrive: D) -> Result<i8, ErrorWrapper>
    where
        D: ProvideCallerwithVisitortoDrive<'m>,
    { 
        enum Field()
        ProvideCallerwithVisitortoDrive.ProvideCallerwithVisitortoDrive_i8(I8visitor)
    }
}

pub trait Visitor<'m>: Sized {
    type Value;

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: MapError;

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: MapError;

}

struct I32Visitor;

impl<'m> Visitor<'m> for I32Visitor {
    type Value = i32;

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: MapError,
    {
        Ok(i32::from(value))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: MapError,
    {
        Ok(value)
    }
}

struct I8visitor;

impl<'m> Visitor<'m> for I8visitor {
    type Value = i8;

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: MapError,
    {
        Ok(value)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: MapError,
    {
        use std::i8;
        if value >= i8::from(i32::MIN) && value <= i8::from(i32::MAX) {
            Ok(value as i8)
        } else {
            Err(E::(format!("i32 out of range: {}", value)))
        }
    }
}

struct Duration {
    secs: i8,
    nanos: i32,
}

fn main() {
    
}