use crate::extension::shortcode::ZINORANDOMDEQUE;
use crate::model::File;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::time::{Duration, Instant};
use zino::{prelude::*, Cluster, Request, Response, Result};
use zino_core::extension::TomlValueExt;
use zino_core::model::Query;
use zino_core::Uuid;

// 加密上传
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

// 解密下载
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

// 文件分享状态
pub async fn share_file_status(req: Request) -> Result {
    let mut body = Map::new();
    let query_file_total_number = Query::new(json!({}));
    let query_file_active_number = Query::new(json!({
        "status":"active"
    }));
    let total_number: u64 = File::count(&query_file_total_number).await.unwrap_or(0);
    let active_number: u64 = File::count(&query_file_active_number).await.unwrap_or(0);
    let mut share_status = Map::new();
    share_status.upsert("total", total_number);
    share_status.upsert("active", active_number);

    body.upsert("share_status", share_status);
    let mut res = Response::default().context(&req);

    res.set_json_data(Map::data_entry(body));
    Ok(res.into())
}

// 文件分享
pub async fn share(mut req: Request) -> Result {
    let file = req.parse_file().await?;
    let mut body = Map::new();

    let dir = Cluster::shared_dir("uploads");

    let application_config = Cluster::config();
    let expirse_time_second = application_config["uploads"]["expires"].as_u32().unwrap();
    // println!("{:?}", expirse_time_second);

    let upload_time = DateTime::now().format("%Y-%m-%d");
    let upload_seconds = DateTime::now().timestamp_micros();
    let _ = fs::create_dir_all(
        dir.join(&upload_time)
            .join(upload_seconds.to_string().as_str()),
    );

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
        file.write(
            dir.join(&upload_time)
                .join(upload_seconds.to_string())
                .join(file_name),
        )
        .extract(&req)?;
        let file_name_full =
            upload_time + "/" + upload_seconds.to_string().as_str() + "/" + file_name;
        query.upsert("file_name", file_name_full.as_str());
        file_str.set_name(file_name);
        file_str.set_local_path(&file_name_full);
        let mut map = Map::new();
        map.upsert("field_name", file.field_name());
        map.upsert("file_name", file_name);
        map.upsert("content_type", file.content_type().map(|m| m.as_ref()));
        // map.upsert("url", format!("/file/decrypt?{}", query.to_query_string()));

        let short_code_str: String = {
            let random_code_lock = Arc::clone(&ZINORANDOMDEQUE);
            let mut random_code = random_code_lock.lock().unwrap();
            random_code.get_random_num()
        };

        // let short_code_str = util::gen_code(random_num);
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

// 大文件分享
pub async fn share_big_file(mut req: Request) -> Result {
    let (mut body, files) = req.parse_form_data::<Map>().await?;
    // println!("{:?}", body);
    let mut upload_status = 100;

    if let Some(chunks_number) = body.get_str("chunksnumber") {
        let total_number = body.get_str("totalNum").unwrap();
        let file_uuid_req = body.get_str("fileuuid").unwrap();
        let file_name = body.get_str("fileName").unwrap();
        let file_md5 = body.get_str("filemd5").unwrap();
        let file_ext = body.get_str("fileExt").unwrap();

        let mut file_uuid = "".to_string();

        if file_uuid_req == "" {
            file_uuid = Uuid::new_v4().to_string();
        } else {
            file_uuid = file_uuid_req.to_owned();
        }

        let dir = Cluster::shared_dir("uploads");
        let tmp_file_path_full = dir.join("tmp").join(file_uuid.to_owned() + "." + file_ext);
        // println!("{:?}", tmp_file_path_full);
        if chunks_number == "1" {
            let mut file = fs::File::create(tmp_file_path_full.clone()).expect("create failed");
            let _ = file.write_all(files[0].as_ref());
        } else {
            let mut file = OpenOptions::new()
                .append(true)
                .open(tmp_file_path_full.clone())
                .expect("cannot open file");
            let _ = file.write_all(files[0].as_ref());
        }
        if chunks_number != total_number {
            upload_status = 100;
        } else {
            let application_config = Cluster::config();
            let expirse_time_second = application_config["uploads"]["expires"].as_u32().unwrap();
            let expires = DateTime::now() + Duration::from_secs(expirse_time_second.into());

            let upload_time = DateTime::now().format("%Y-%m-%d");
            let upload_seconds = DateTime::now().timestamp_micros();
            let file_full_path = dir
                .join(&upload_time)
                .join(upload_seconds.to_string().as_str());
            let _ = fs::create_dir_all(file_full_path.clone());

            let mut uploads = Vec::new();

            let mut query = Map::new();
            let access_key_id = AccessKeyId::new();
            query.upsert("access_key_id", access_key_id.to_string());

            let secret_key = SecretAccessKey::new(&access_key_id);
            let security_token =
                SecurityToken::try_new(access_key_id, expires, &secret_key).extract(&req)?;
            query.upsert("security_token", security_token.to_string());

            let _ = fs::rename(
                tmp_file_path_full.to_str().unwrap(),
                file_full_path.join(file_name).to_str().unwrap(),
            );

            let mut file_str = File::new();

            let file_full_path_string =
                upload_time + "/" + upload_seconds.to_string().as_str() + "/" + file_name;
            query.upsert("file_name", file_full_path_string.clone());
            file_str.set_name(file_name);
            file_str.set_local_path(file_full_path_string.as_str());
            let mut map = Map::new();
            map.upsert("file_name", file_name);
            // map.upsert("url", format!("/file/decrypt?{}", query.to_query_string()));

            let short_code_str: String = {
                let random_code_lock = Arc::clone(&ZINORANDOMDEQUE);
                let mut random_code = random_code_lock.lock().unwrap();
                random_code.get_random_num()
            };

            // let short_code_str = util::gen_code(random_num);
            file_str.set_short_code(short_code_str.as_str());
            //file_str.set_short_code(&get_random_code());
            // file_str.set_md5(file.content_md5().as_str());
            file_str.set_local_url(&format!("/file/download?{}", query.to_query_string()));
            file_str.set_delete_time(expirse_time_second.into());
            map.upsert("short_code", file_str.get_short_code());

            file_str.insert().await.extract(&req)?;
            uploads.push(map);
            body.upsert("file", uploads);
            upload_status = 200;
        }
        body.upsert("file_uuid", file_uuid.to_string());
    } else {
        reject!(req, "chunks_number", "is't");
    }

    body.upsert("upload_status", upload_status);
    // println!("{:?}", upload_status);

    let mut res = Response::default().context(&req);

    res.set_json_data(Map::data_entry(body));
    Ok(res.into())
}

// 文件下载
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

// 下载文件信息获取
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
    file_obj[0].download_count_plus();
    let _ = file_obj[0].clone().update().await;
    let mut res = Response::default().context(&req);
    res.set_json_data(Map::data_entry(body));
    Ok(res.into())
}
