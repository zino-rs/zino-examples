#![allow(async_fn_in_trait)]
#![allow(stable_features)]
#![feature(async_fn_in_trait)]
#![feature(lazy_cell)]
#![feature(let_chains)]

mod controller;
mod domain;
mod extension;
mod logic;
mod middleware;
mod model;
mod router;
mod schedule;
mod service;

use crate::model::File;
use crate::extension::shortcode::ZINORANDOMDEQUE;
use crate::extension::util;
use std::time::Duration;
use zino_core::model::Query;
use std::sync::Arc;
use tokio;
use zino::prelude::*;

fn main() {
    let init = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    init.block_on(async {
        let mut remove_random_num: Vec<u32> = Vec::new();
        let query_obj = Query::new(json!({
        "status":"active"
                }));
        let mut file_obj: Vec<File> = File::find(&query_obj).await.unwrap();
        for file in &mut file_obj {
            let short_code = file.get_short_code();
            let random_num = util::de_code(short_code);
            remove_random_num.push(random_num)
        }
        let random_code_lock = Arc::clone(&ZINORANDOMDEQUE);
        let mut random_code = random_code_lock.lock().unwrap();

        if !random_code.is_init(){
            random_code.init_once(remove_random_num);
        }
        
    });
    init.shutdown_timeout(Duration::from_secs_f64(60.0));

    zino::Cluster::boot()
        .register(router::routes())
        .register_debug(router::debug_routes())
        .spawn(schedule::job_scheduler())
        .run_with(schedule::async_job_scheduler())
}
