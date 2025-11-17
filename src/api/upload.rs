use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;

use axum::extract::{Extension, Multipart};
use axum::response::{IntoResponse, Redirect};
use log::{info, warn};
use sha2::{Digest, Sha256};
use tokio::fs;

use crate::server::SpustConfig;

struct Upload {
    email: Option<String>,
    password: Option<String>,
    data: Option<Vec<u8>>,
    file_name: Option<String>,
    content_type: Option<String>,
}

fn make_sha(input: &str, num: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let hash_string = hex::encode(result);
    hash_string[..num].to_owned()
}

pub async fn upload_handler(
    Extension(conf): Extension<Arc<SpustConfig>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut upload: Upload = Upload {
        email: None,
        password: None,
        data: None,
        file_name: None,
        content_type: None,
    };
    loop {
        match multipart.next_field().await {
            Ok(value) => match value {
                Some(field) => {
                    let field_name = field.name().unwrap_or_default();
                    info!("{:?}", field_name);
                    match field_name {
                        "email" => match field.bytes().await {
                            Ok(bytes) => {
                                if let Ok(decoded) = String::from_utf8(bytes.to_vec()) {
                                    upload.email = Some(decoded);
                                }
                            }
                            Err(e) => {
                                warn!("Error: {:?}", e);
                            }
                        },
                        "password" => match field.bytes().await {
                            Ok(bytes) => {
                                if let Ok(decoded) = String::from_utf8(bytes.to_vec()) {
                                    upload.password = Some(decoded);
                                }
                            }
                            Err(e) => {
                                warn!("Error: {:?}", e);
                            }
                        },
                        "file" => {
                            if let Some(filename) = field.file_name() {
                                upload.file_name = Some(filename.to_string());
                            }
                            if let Some(contenttype) = field.content_type() {
                                upload.content_type = Some(contenttype.to_string())
                            }
                            if let Ok(bytes) = field.bytes().await {
                                upload.data = Some(bytes.to_vec());
                            }
                        }
                        _ => {
                            warn!("{:?}", field_name);
                        }
                    }
                }
                None => break,
            },
            Err(e) => {
                println!("Error: {:?}", e);
                return Redirect::to("../send.html?error=all").into_response();
            }
        }
    }
    if upload.email.is_none() {
        return Redirect::to("../send.html?error=email").into_response();
    }
    if upload.password.is_none() {
        return Redirect::to("../send.html?error=password").into_response();
    }

    // TODO auth

    let data = match upload.data {
        Some(d) => d,
        None => return Redirect::to("../send.html?error=file").into_response(),
    };
    if upload.file_name.is_none() || upload.content_type.is_none() {
        return Redirect::to("../send.html?error=file").into_response();
    }

    // create filename
    let file_name = match upload.file_name {
        Some(e) => e,
        None => return Redirect::to("../send.html?error=file").into_response(),
    };
    let extension = match Path::new(&file_name).extension().and_then(OsStr::to_str) {
        Some(e) => e,
        None => return Redirect::to("../send.html?error=file").into_response(),
    };
    let correct_filename = make_sha(&file_name, 8);
    let time = chrono::offset::Local::now().timestamp() / 1000;
    let complete_filename = format!("{}{}.{}", time, correct_filename, extension);

    // save file
    if let Err(_err) = fs::write(Path::join(&conf.upload_dir, complete_filename), data).await {
        warn!("File not saved");
        return Redirect::to("../send.html?error=save").into_response();
    }
    Redirect::to("../send.html?success=true").into_response()
}
