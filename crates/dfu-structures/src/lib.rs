use crate::types::{ConversionFunc, Types};
use quartz_nbt::NbtTag;
use std::error::Error;

pub mod types;

pub fn call_closure_with<'a, T, F: Fn(&'a mut NbtTag, usize, usize) -> Result<T, Box<dyn Error>>>(
    f: F,
    value: &'a mut NbtTag,
    from_ver: usize,
    to_ver: usize,
) -> Result<T, Box<dyn Error>> {
    f(value, from_ver, to_ver)
}

pub struct SchemaInfo<'d> {
    pub references: &'d mut Types<&'static ConversionFunc>,
    pub version: usize,
}

impl<'d> SchemaInfo<'d> {
    pub fn new(references: &'d mut Types<&'static ConversionFunc>, version: usize) -> Self {
        Self { references, version }
    }
}
