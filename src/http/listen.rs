use super::request;
use crate::print::{self, print::GREEN};
use crate::shell::parse;
use std::collections::HashMap;
use urlencoding::decode;
use warp::{reject::Rejection, reply::Reply, Filter};

const IP_ARR: [i32; 4] = [0, 0, 0, 0];
const URL_KEY_NAME: &str = "url";
const REQUEST_HEADER_KEY_NAME: &str = "request_header";
const DATA_KEY_NAME: &str = "data";
const RESPONSE_HEADER_KEY_NAME: &str = "response_header";
const IS_ORIGINAL: &str = "original";

/**
 * 分割参数
 */
fn split_params(param: &str) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for pair in param.split('&') {
        let mut kv = pair.splitn(2, '=');
        if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
            map.insert(key.to_string(), value.to_string());
        }
    }
    map
}

/**
 * 获取参数解码数据
 */
fn get_param_decode_value(key: &str, query_map: &HashMap<String, String>) -> String {
    if let Some(value) = query_map.get(key) {
        let decoded_value = decode(value).unwrap_or_else(|_| value.to_string().into());
        decoded_value.to_string()
    } else {
        String::new()
    }
}

/**
 * 处理请求
 */
async fn handle_request(query_map: HashMap<String, String>) -> Result<impl Reply, Rejection> {
    // 请求地址
    let mut _url_str: String = get_param_decode_value(URL_KEY_NAME, &query_map);

    // 原始响应
    let mut _is_original_str: bool = get_param_decode_value(IS_ORIGINAL, &query_map).len() > 0;

    // 请求头
    let mut _request_header_str: String =
        get_param_decode_value(REQUEST_HEADER_KEY_NAME, &query_map);
    let mut _request_header_map: HashMap<String, String> = split_params(&_request_header_str);

    // POST数据
    let mut _data_str: String = get_param_decode_value(DATA_KEY_NAME, &query_map);
    let mut _data_map: HashMap<String, String> = split_params(&_data_str);

    // 响应头
    let mut _response_header_str: String =
        get_param_decode_value(RESPONSE_HEADER_KEY_NAME, &query_map);
    let mut _response_header_map: HashMap<String, String> = split_params(&_response_header_str);

    request::request(
        &_url_str,
        &_is_original_str,
        &_request_header_map,
        &_data_map,
        &_response_header_map,
    )
    .await
}

/**
 * 运行
 */
pub async fn run() {
    // IP
    let ip_str: String = IP_ARR
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(".");
    // 端口
    let port: u16 = parse::get_port();

    print::print::println(&format!("Listen: http://{}:{}", ip_str, port), &GREEN);

    // 监听路由
    let route = warp::get()
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handle_request);

    warp::serve(route).run(([0, 0, 0, 0], port)).await;
}
