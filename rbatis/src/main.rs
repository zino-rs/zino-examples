#![allow(async_fn_in_trait)]
#![allow(stable_features)]
#![feature(async_fn_in_trait)]
#![feature(lazy_cell)]

mod controller;
mod extension;
mod model;
mod router;

use zino::prelude::*;

fn main() {
    zino::Cluster::boot()
        .register(router::routes())
        .run::<AsyncJobScheduler>(None)
}
