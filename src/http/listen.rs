use crate::print::{
    self,
    print::{GREEN, WHITE},
};
use std::env;
use std::{collections::HashMap, fmt::format};
use urlencoding::decode;
use warp::{filters, Filter};

const DEFAULT_PORT: u16 = 80;
const IP_ARR: [i32; 4] = [0, 0, 0, 0];
const PATH_METHOD: &str = "/Proxy/proxyRequest";
const SOURCE_URL_KEY_NAME: &str = "url";
const SOURCE_REQUEST_HEADER_KEY_NAME: &str = "request_header";
const SOURCE_RESPONSE_HEADER_KEY_NAME: &str = "response_header";
const SOURCE_DATA_KEY_NAME: &str = "data";

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
fn handle_request(query_map: &HashMap<String, String>) -> String {
    // 请求地址
    let mut source_url_str: String = get_param_decode_value(SOURCE_URL_KEY_NAME, query_map);

    // 请求头
    let mut source_request_header_str: String =
        get_param_decode_value(SOURCE_REQUEST_HEADER_KEY_NAME, query_map);
    let mut source_request_header_map: HashMap<String, String> =
        split_params(&source_request_header_str);

    // 响应头
    let mut source_response_header_str: String =
        get_param_decode_value(SOURCE_RESPONSE_HEADER_KEY_NAME, query_map);
    let mut source_response_header_map: HashMap<String, String> =
        split_params(&source_response_header_str);

    // POST数据
    let mut source_data_str: String = get_param_decode_value(SOURCE_DATA_KEY_NAME, query_map);
    let mut source_data_map: HashMap<String, String> = split_params(&source_data_str);

    // 记录日志
    let msg: String = format!(
        "Request url: {}\nRequest header: {:?}\nResponse header: {:?}\nRequest data: {:?}\n",
        source_url_str, source_request_header_map, source_response_header_map, source_data_map
    );
    print::print::println(&msg, &WHITE);
    String::new()
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
    let args: Vec<String> = env::args().collect();
    let port: u16 = args
        .get(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    print::print::println(&format!("Listen: http://{}:{}", ip_str, port), &GREEN);

    // 监听路由
    let route = warp::get()
        .and(warp::query::<HashMap<String, String>>())
        .map(|params: HashMap<String, String>| -> String { handle_request(&params) });

    warp::serve(route).run(([0, 0, 0, 0], port)).await;
}
