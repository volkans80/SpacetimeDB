// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct EveryPrimitiveStruct {
    pub a: u8,
    pub b: u16,
    pub c: u32,
    pub d: u64,
    pub e: u128,
    pub f: __sats::u256,
    pub g: i8,
    pub h: i16,
    pub i: i32,
    pub j: i64,
    pub k: i128,
    pub l: __sats::i256,
    pub m: bool,
    pub n: f32,
    pub o: f64,
    pub p: String,
    pub q: __sdk::Identity,
    pub r: __sdk::ConnectionId,
    pub s: __sdk::Timestamp,
    pub t: __sdk::TimeDuration,
}

impl __sdk::InModule for EveryPrimitiveStruct {
    type Module = super::RemoteModule;
}
