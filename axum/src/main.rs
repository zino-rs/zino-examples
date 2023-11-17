#![allow(async_fn_in_trait)]
#![allow(stable_features)]
#![feature(async_fn_in_trait)]
#![feature(lazy_cell)]

mod controller;
mod model;
mod router;

use zino::prelude::*;

fn main() {
    zino::Cluster::boot()
        .register(router::routes())
        .run(StaticRecord::new())
}
