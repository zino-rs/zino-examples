mod controller;
mod model;
mod router;

use std::time::Duration;
use kip_sql::db::Database;
use zino::prelude::*;
use crate::controller::bench;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let database = Database::with_kipdb("./data")
            .await
            .expect("what happened");
        let _ = database.run("create table if not exists users (\
            id int primary key, \
            name varchar default 'unknown', \
            status varchar default 'active', \
            description varchar default '' \
        )").await.unwrap();

        let _ = bench::INSTANCE.set(database);
    });

    rt.shutdown_timeout(Duration::from_secs_f64(60.0));

    zino::Cluster::boot()
        .register(router::routes())
        .run()
}