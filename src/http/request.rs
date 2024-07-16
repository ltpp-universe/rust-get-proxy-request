use super::response::Response;
use crate::print::print::{self, BLUE, CYAN, RED, YELLOW};
use crate::utils::time;

use bytes::{Bytes, BytesMut};
use hyper::StatusCode;
use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};
use serde::de::value;
use serde::Serialize;
use serde_json::to_string;
use std::collections::HashMap;
use std::str;
use warp::{
    http::Response as WarpResponse,
    reject::Rejection,
    reply::{Json, Reply},
    Filter,
};

const FAILED_TO_SEND_REQUEST: &str = "FAILED_TO_SEND_REQUEST";
const INVALID_URL: &str = "INVALID_URL";
const REQUEST_SUCCESS: &str = "REQUEST_SUCCESS";
const REQUEST_FAILED: &str = "REQUEST_FAILED";
const CONTENT_LENGTH: &str = "content-length";
const CONTENT_TYPE: &str = "content-type";
const CONTENT_TYPE_VALUE: &str = "application/json; charset=utf-8";

/**
 * 检测URL是否合法
 */
fn is_valid_url(url: &str) -> bool {
    if let Ok(res) = Regex::new(r#"^(https?|ftp)://[^\s/$.?#].[^\s]*$"#) {
        res.is_match(url)
    } else {
        false
    }
}

/**
 * 将 warp::reply::json(&Response {}) 转换为 Bytes
 */
fn reply_json_to_bytes(response: &Response) -> Bytes {
    let json_str: String = to_string(response).unwrap_or_else(|_| String::new());
    Bytes::from(json_str)
}

/**
 * &str转Bytes
 */
fn str_to_bytes(s: &str) -> Bytes {
    Bytes::copy_from_slice(s.as_bytes())
}

/**
 * 将 Bytes 转换为 String
 */
fn bytes_to_string(bytes: &Bytes) -> String {
    match str::from_utf8(bytes) {
        Ok(res) => res.to_owned(),
        _ => String::new(),
    }
}

/**
 * 发送请求
 */
pub async fn request(
    url_str: &str,
    is_original_str: &bool,
    request_header_map: &HashMap<String, String>,
    data_map: &HashMap<String, String>,
    response_header_map: &HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let mut res_builder = WarpResponse::builder();
    if !is_valid_url(url_str) {
        // 设置响应头
        for (key, value) in response_header_map {
            let key_str: String = key.to_lowercase();
            if key_str == CONTENT_LENGTH || key_str == CONTENT_TYPE {
                continue;
            }
            res_builder = res_builder.header(key_str, value);
        }
        res_builder = res_builder.header(CONTENT_TYPE, CONTENT_TYPE_VALUE);

        // 返回原始响应
        if *is_original_str {
            print::println(&REQUEST_FAILED, &RED);
            return Ok(res_builder
                .body(str_to_bytes(REQUEST_FAILED))
                .unwrap_or_else(|_| WarpResponse::new(Bytes::new())));
        }

        // 包装JSON响应
        let response: Response = Response {
            status: 200,
            url: url_str.to_owned(),
            request_header: request_header_map.clone(),
            response: REQUEST_FAILED.to_owned(),
            response_header: HashMap::new(),
            time: time::format_now_time(),
        };
        let response_bytes: Bytes = reply_json_to_bytes(&response);
        print::println(&bytes_to_string(&response_bytes), &RED);

        // 响应
        return Ok(res_builder
            .body(response_bytes)
            .unwrap_or_else(|_| WarpResponse::new(Bytes::new())));
    }

    // 创建请求客户端
    let client: Client = Client::new();

    // 创建请求头
    let mut headers: HeaderMap = HeaderMap::new();
    for (key, value) in request_header_map.iter() {
        match HeaderName::from_bytes(key.as_bytes()) {
            Ok(header_name) => match HeaderValue::from_str(value) {
                Ok(header_value) => {
                    headers.insert(header_name, header_value);
                }
                Err(e) => {}
            },
            Err(e) => {}
        }
    }

    // 创建请求体
    let mut form_data: HashMap<&str, &str> = HashMap::new();
    for (key, value) in data_map.iter() {
        form_data.insert(key.as_str(), value.as_str());
    }

    let is_get: bool = data_map.is_empty();

    // 发送请求
    let response: reqwest::Response = match is_get {
        true =>
        // GET请求
        {
            client
                .get(url_str)
                .headers(headers)
                .send()
                .await
                .expect(FAILED_TO_SEND_REQUEST)
        }
        _ =>
        // POST请求
        {
            client
                .post(url_str)
                .headers(headers)
                .form(&form_data)
                .send()
                .await
                .expect(FAILED_TO_SEND_REQUEST)
        }
    };

    // 获取响应头
    let mut response_headers: HeaderMap = HeaderMap::new();
    for (key, value) in response.headers().iter() {
        response_headers.insert(key.clone(), value.clone());
    }

    // 合并响应头
    let mut combined_headers: HashMap<String, String> = HashMap::new();
    for (key, value) in response_headers.iter() {
        let key_str: String = key.to_string().to_lowercase();
        let value_str: String = if let Ok(str) = value.to_str() {
            str.to_string()
        } else {
            String::new()
        };
        combined_headers.insert(key_str, value_str);
    }
    for (key, value) in response_header_map.iter() {
        combined_headers.insert(key.to_lowercase(), value.clone());
    }

    // 设置响应头
    for (key, value) in combined_headers.iter() {
        let key_str: String = key.to_lowercase();
        if key_str == CONTENT_LENGTH {
            continue;
        }
        res_builder = match !*is_original_str && key_str == CONTENT_TYPE {
            true => res_builder.header(CONTENT_TYPE, CONTENT_TYPE_VALUE),
            _ => res_builder.header(key_str, value),
        };
    }

    // 返回原始响应
    if *is_original_str {
        let body_bytes: Bytes = response.bytes().await.unwrap_or_else(|_| Bytes::new()); // 获取响应体的Bytes
        print::println(&bytes_to_string(&body_bytes), &BLUE);
        return Ok(res_builder
            .body(body_bytes)
            .unwrap_or_else(|_| WarpResponse::new(Bytes::new())));
    }

    // 包装JSON响应
    let response: Response = Response {
        status: response.status().into(),
        url: url_str.to_owned(),
        request_header: request_header_map.clone(),
        response: response.text().await.unwrap_or(String::new()),
        response_header: combined_headers,
        time: time::format_now_time(),
    };
    let response_bytes: Bytes = reply_json_to_bytes(&response);
    print::println(&format!("{:?}", response), &YELLOW);

    // 响应
    return Ok(res_builder
        .body(response_bytes)
        .unwrap_or_else(|_| WarpResponse::new(Bytes::new())));
}
