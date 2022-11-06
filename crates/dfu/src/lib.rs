#![feature(const_for)]

mod schema;
mod util;

use crate::schema::v99;

use dfu_structures::types::{ConversionFunc, Types};
use lazy_static::lazy_static;

fn create_types() -> &'static Types<&'static ConversionFunc> {
    let mut types = Types::new();

    v99::define(&mut types);

    Box::leak(Box::new(types))
}

lazy_static! {
    pub static ref TYPES: &'static Types<&'static ConversionFunc> = create_types();
}
