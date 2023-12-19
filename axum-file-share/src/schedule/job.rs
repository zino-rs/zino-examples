use crate::extension::shortcode::ZINORANDOMDEQUE;
use crate::model::File;
use faster_hex;
use md5::{Digest, Md5};
use std::fs;
use std::io::Read;
use std::sync::Arc;
use zino::{prelude::*, Cluster};
use zino_core::model::Query;
use zino_model::User;

pub fn every_15s(job_id: Uuid, job_data: &mut Map, last_tick: DateTime) {
    let counter = job_data
        .get("counter")
        .map(|c| c.as_u64().unwrap_or_default() + 1)
        .unwrap_or_default();
    job_data.upsert("counter", counter);
    job_data.upsert("current", DateTime::now());
    job_data.upsert("last_tick", last_tick);
    job_data.upsert("job_id", job_id.to_string());
}

pub fn every_20s(job_id: Uuid, job_data: &mut Map, last_tick: DateTime) {
    let counter = job_data
        .get("counter")
        .map(|c| c.as_u64().unwrap_or_default() + 1)
        .unwrap_or_default();
    job_data.upsert("counter", counter);
    job_data.upsert("current", DateTime::now());
    job_data.upsert("last_tick", last_tick);
    job_data.upsert("job_id", job_id.to_string());
}

pub fn every_hour(job_id: Uuid, job_data: &mut Map, last_tick: DateTime) -> BoxFuture {
    let counter = job_data
        .get("counter")
        .map(|c| c.as_u64().unwrap_or_default() + 1)
        .unwrap_or_default();
    job_data.upsert("counter", counter);
    job_data.upsert("current", DateTime::now());
    job_data.upsert("last_tick", last_tick);
    job_data.upsert("job_id", job_id.to_string());
    Box::pin(async {
        let query = Query::default();
        let columns = [("*", true), ("roles", true)];
        if let Ok(mut map) = User::count_many(&query, &columns).await {
            job_data.append(&mut map);
        }
    })
}

pub fn clean_file(job_id: Uuid, job_data: &mut Map, _last_tick: DateTime) -> BoxFuture {
    let counter = job_data
        .get("counter")
        .map(|c| c.as_u64().unwrap_or_default() + 1)
        .unwrap_or_default();
    job_data.upsert("current", DateTime::now().to_string());
    job_data.upsert("counter", counter);
    tracing::debug!(
        job_data = format!("{job_data:?}"),
        "async clean job {job_id} is executed every 10 miniter"
    );

    Box::pin(async {
        let dir = Cluster::shared_dir("uploads");

        // let table_name = File::table_name();
        let current_time = &DateTime::now().to_utc_timestamp();
        let query_obj = Query::new(json!({
        "delete_at":{"$lt":current_time},
        "status":"active"
                }));
        let mut file_obj: Vec<File> = File::find(&query_obj).await.unwrap();
        {
            let random_code_lock = Arc::clone(&ZINORANDOMDEQUE);
            let mut random_code = random_code_lock.lock().unwrap();
            for file in &mut file_obj {
                let local_path = file.get_localpath();
                let _ = fs::remove_file(dir.join(local_path));
                let short_code = file.get_short_code();
                println!("recyle the short code: {:?}", short_code);
                random_code.push_random_num(short_code);
            }
        }
        for mut file in file_obj {
            file.set_unactive();
            let _ = file.update().await;
        }

        let no_md5_query_obj = Query::new(json!({
        "md5":"",
        "status":"active"
            }));
        let file_md5: Vec<File> = File::find(&no_md5_query_obj).await.unwrap();
        for mut file in file_md5 {
            let local_path = file.get_localpath();
            let mut f: fs::File = fs::File::open(dir.join(local_path)).unwrap();
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).unwrap();
            let mut hasher = Md5::new();
            hasher.update(buffer);
            let md5_str = hasher.finalize();
            let md5_string = faster_hex::hex_string(md5_str.as_ref());
            file.set_md5(&md5_string);
            let _ = file.update().await;
        }
    })
}
