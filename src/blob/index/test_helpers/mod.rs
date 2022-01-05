use super::prelude::*;

pub(crate) const META_SIZE: usize = 100;
pub(crate) const META_VALUE: u8 = 17;

pub(crate) mod file_mock;
mod findex_builder;
mod findex_mother;
mod generic_scenarios;
mod inmem_builder;
mod key;
mod rh_builder;

pub(crate) use findex_builder::FileIndexBuilder;
pub(crate) use findex_mother::FindexMother;
#[allow(unused_imports)]
pub(crate) use generic_scenarios::*;
pub(crate) use inmem_builder::InMemoryIndexBuilder;
pub(crate) use key::KeyType;
pub(crate) use rh_builder::RecordHeaderBuilder;
