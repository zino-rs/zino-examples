use crate::extension::shortcode::ZINORANDDEQUE;
use crate::extension::util;
use crate::model::File;
use std::fs;
use std::sync::Arc;
use std::time::{Duration, Instant};
use zino::{prelude::*, Cluster, Request, Response, Result};
use zino_core::extension::TomlValueExt;
use zino_core::model::Query;

pub async fn upload(mut req: Request) -> Result {
    let (mut body, files) = req.parse_form_data::<Map>().await?;

    let dir = Cluster::shared_dir("uploads");
    let expires = DateTime::now() + Duration::from_secs(600);
    let mut encryption_duration = Duration::ZERO;
    let mut uploads = Vec::new();
    for mut file in files {
        let mut query = Map::new();
        let access_key_id = AccessKeyId::new();
        query.upsert("access_key_id", access_key_id.to_string());

        let secret_key = SecretAccessKey::new(&access_key_id);
        let security_token =
            SecurityToken::try_new(access_key_id, expires, &secret_key).extract(&req)?;
        query.upsert("security_token", security_token.to_string());

        let encryption_start_time = Instant::now();
        file.encrypt_with(secret_key.as_ref()).extract(&req)?;
        encryption_duration += encryption_start_time.elapsed();

        if let Some(file_name) = file.file_name() {
            file.write(dir.join(file_name)).extract(&req)?;
            query.upsert("file_name", file_name);

            let mut map = Map::new();
            map.upsert("field_name", file.field_name());
            map.upsert("file_name", file_name);
            map.upsert("content_type", file.content_type().map(|m| m.as_ref()));
            map.upsert("url", format!("/file/decrypt?{}", query.to_query_string()));
            uploads.push(map);
        }
    }
    body.upsert("files", uploads);

    let mut res = Response::default().context(&req);
    res.record_server_timing("enc", None, Some(encryption_duration));
    res.set_json_data(Map::data_entry(body));
    Ok(res.into())
}

pub async fn decrypt(req: Request) -> Result {
    let query = req.parse_query::<Map>()?;
    let access_key_id = req.parse_access_key_id()?;
    let secret_key = SecretAccessKey::new(&access_key_id);
    let security_token = req.parse_security_token(secret_key.as_ref())?;
    if security_token.is_expired() {
        reject!(req, forbidden, "the security token has expired");
    }

    let Some(file_name) = query.get_str("file_name") else {
        reject!(req, "file_name", "it should be specified");
    };
    let file_path = Cluster::shared_dir("uploads").join(file_name);

    let mut file = NamedFile::try_from_local(file_path).extract(&req)?;
    let decryption_start_time = Instant::now();
    file.decrypt_with(secret_key).extract(&req)?;

    let decryption_duration = decryption_start_time.elapsed();
    let mut res = Response::default().context(&req);
    res.record_server_timing("dec", None, Some(decryption_duration));
    res.send_file(file);
    Ok(res.into())
}

pub async fn share(mut req: Request) -> Result {
    let file = req.parse_file().await?;
    let mut body = Map::new();

    let dir = Cluster::shared_dir("uploads");

    let application_config = Cluster::config();
    let expirse_time_second = application_config["uploads"]["expires"].as_u32().unwrap();
    // println!("{:?}", expirse_time_second);

    let upload_time = DateTime::now().format("%Y-%m-%d");
    if !dir.join(&upload_time).exists() {
        let _ = fs::create_dir_all(dir.join(&upload_time));
    }

    let expires = DateTime::now() + Duration::from_secs(expirse_time_second.into());
    let mut encryption_duration = Duration::ZERO;
    let mut uploads = Vec::new();

    let mut query = Map::new();
    let access_key_id = AccessKeyId::new();
    query.upsert("access_key_id", access_key_id.to_string());

    let secret_key = SecretAccessKey::new(&access_key_id);
    let security_token =
        SecurityToken::try_new(access_key_id, expires, &secret_key).extract(&req)?;
    query.upsert("security_token", security_token.to_string());

    let encryption_start_time = Instant::now();
    // file.encrypt_with(secret_key.as_ref()).extract(&req)?;

    let mut file_str = File::new();
    if let Some(file_name) = file.file_name() {
        file.write(dir.join(&upload_time).join(file_name))
            .extract(&req)?;
        let file_name_full = upload_time + "/" + file_name;
        query.upsert("file_name", file_name_full.as_str());
        file_str.set_name(file_name);
        file_str.set_local_path(&file_name_full);
        let mut map = Map::new();
        map.upsert("field_name", file.field_name());
        map.upsert("file_name", file_name);
        map.upsert("content_type", file.content_type().map(|m| m.as_ref()));
        // map.upsert("url", format!("/file/decrypt?{}", query.to_query_string()));
        let random_num: u32 = {
            let random_code_lock = Arc::clone(&ZINORANDDEQUE);
            let mut random_code = random_code_lock.lock().unwrap();
            // println!("{:?}",random_code);
            let temp_num = random_code.pop_front().unwrap();
            temp_num + 1000000
        };

        let short_code_str = util::gen_code(random_num);
        file_str.set_short_code(short_code_str.as_str());
        //file_str.set_short_code(&get_random_code());
        // file_str.set_md5(file.content_md5().as_str());
        file_str.set_local_url(&format!("/file/download?{}", query.to_query_string()));
        file_str.set_delete_time(expirse_time_second.into());
        map.upsert("short_code", file_str.get_short_code());

        file_str.insert().await.extract(&req)?;
        uploads.push(map);
    }

    body.upsert("file", uploads);
    let mut res = Response::default().context(&req);
    encryption_duration += encryption_start_time.elapsed();
    res.record_server_timing("upload_time", None, Some(encryption_duration));
    res.set_json_data(Map::data_entry(body));
    Ok(res.into())
}

pub async fn download(req: Request) -> Result {
    let query = req.parse_query::<Map>()?;
    let access_key_id = req.parse_access_key_id()?;
    let secret_key = SecretAccessKey::new(&access_key_id);
    let security_token = req.parse_security_token(secret_key.as_ref())?;
    if security_token.is_expired() {
        reject!(req, forbidden, "the security token has expired");
    }

    let Some(file_name) = query.get_str("file_name") else {
        reject!(req, "file_name", "it should be specified");
    };
    let file_path = Cluster::shared_dir("uploads").join(file_name);
    // println!("{:?}", file_path);
    let file = NamedFile::try_from_local(file_path).extract(&req)?;
    //let decryption_start_time = Instant::now();
    // file.decrypt_with(secret_key).extract(&req)?;

    // let decryption_duration = decryption_start_time.elapsed();
    let mut res = Response::default().context(&req);
    // res.record_server_timing("dec", None, Some(decryption_duration));
    res.send_file(file);
    Ok(res.into())
}

pub async fn get_share_file(req: Request) -> Result {
    let query = req.parse_query::<Map>()?;
    let Some(short_code) = query.get_str("short_code") else {
        reject!(req, "short_code", "it isn't exits");
    };
    let legal_chars = "1234567890ABCDEFGHJKLMNPQRSTUVWXYZ";
    for c in short_code.chars() {
        if !legal_chars.contains(c) {
            reject!(req, "short_code", "it is unlegal");
        }
    }
    
    let current = DateTime::now();
    let current_time = &current.to_utc_timestamp();
    
    let mut body = Map::new();
    let query_file = Query::new(json!({
        "shortcode":short_code,
        "delete_at":{"$gt":current_time}
    }));

    let mut file_obj: Vec<File> = File::find(&query_file).await.unwrap();
    if file_obj.len() == 0 {
        reject!(req, "short_code error", "short code isn't exits");
    }
    body.upsert("file_name", file_obj[0].get_name());
    body.upsert("file_url", file_obj[0].get_localurl());
    let mut res = Response::default().context(&req);
    res.set_json_data(Map::data_entry(body));
    Ok(res.into())
}
