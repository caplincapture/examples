#![allow(unused)]

mod visitor;

use visitor::Visitor;

/// A struct of two integer values.
///
/// It's going to be an output of `Visitor` trait which is defined for the type
/// in `visitor.rs`.
#[derive(Default, Debug)]
pub struct TwoValuesStruct {
    a: i32,
    b: i32,
}

/// A struct of values array.
///
/// It's going to be an output of `Visitor` trait which is defined for the type
/// in `visitor.rs`.
#[derive(Default, Debug)]
pub struct TwoValuesArray {
    ab: [i32; 2],
}

pub trait Deserializer<T: Visitor> {
    fn create(output_struct: T) -> Self;
    fn parse_str(&self, data: &str) -> Result<T::Value, &str> {  
        Err("don't build for a bad reason")
    }
    fn parse_vec(&self, vector: Vec<i32>) -> Result<T::Value, &str> {
        Err("don't build for a bad reason")
    }
}

struct StringDeserializer<V: Visitor> {
    visitor: V,
}

impl<T:Visitor> Deserializer<T> for StringDeserializer<T>{
    fn create(output_struct: T) -> Self {
        Self {
            visitor: output_struct
        }
    }
    fn parse_str(&self, data: &str) -> Result<T::Value, &str> {
        let input_vec = data
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(self.visitor.visit_vec(input_vec)) 
    }

    fn parse_vec(&self, vector: Vec<i32>) -> Result<T::Value, &str> {
        Ok(self.visitor.visit_vec(vector))
    }
}

struct VecDeserializer<V: Visitor>{
    visitor: V
}

impl<T:Visitor> Deserializer<T> for VecDeserializer<T>{
    fn create(output_struct: T) -> Self {
        Self {
            visitor: output_struct
        }
    }
    
    fn parse_vec(&self, vector: Vec<i32>) -> Result<T::Value, &str> {
        Ok(self.visitor.visit_vec(vector))
    }
}

fn main() {
    let deserializer = StringDeserializer::create(TwoValuesStruct::default());
    let result = deserializer.parse_str("123 456");
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValuesStruct::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValuesArray::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    println!(
        "Error: {}",
        deserializer.parse_str("123 456").err().unwrap()
    )
}
