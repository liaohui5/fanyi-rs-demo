use crate::config::ServiceAPIKeys;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// 有道翻译服务API响应
#[derive(Serialize, Deserialize, Debug)]
pub struct YoudaoTranslateResponse {
    query: Option<String>,
    translation: Option<Vec<String>>,
    #[serde(rename = "errorCode")]
    error_code: String,
}

// 有道翻译服务
pub struct YoudaoTranslateService {
    pub app_id: String,
    pub secret: String,
    pub salt: String,
    pub time: String,
}

impl YoudaoTranslateService {
    pub fn new(keys: &ServiceAPIKeys) -> Self {
        let app_id = keys.app_id.clone();
        let secret = keys.secret.clone();
        let salt = String::from(&app_id[0..10]);
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("[youdao translate service]Failed to get timestamp")
            .as_secs()
            .to_string();

        Self {
            app_id,
            secret,
            salt,
            time,
        }
    }

    pub fn calc_sign_input(&self, query: &str) -> String {
        let len = query.len();
        if len > 20 {
            let prefix = String::from(&query[0..10]);
            let endfix = String::from(&query[len - 10..len]);
            return format!("{}{}{}", prefix, len, endfix);
        }
        String::from(query)
    }

    pub fn gen_sign(&self, query: &str) -> String {
        // sign = app_id+input+salt+curtime+secret => sha256
        let input = self.calc_sign_input(query);

        let mut origin_str = String::new();
        origin_str.push_str(&self.app_id);
        origin_str.push_str(&input);
        origin_str.push_str(&self.salt);
        origin_str.push_str(&self.time);
        origin_str.push_str(&self.secret);

        let mut hasher = Sha256::new();
        hasher.update(origin_str.as_bytes());

        let signed_str = format!("{:x}", hasher.finalize());

        // println!("curtime: {}", &self.time);
        // println!("origin_str: {}", origin_str);
        // println!("signed_str: {}", signed_str);
        signed_str
    }

    pub fn build_request_form(&self, query: String) -> HashMap<String, String> {
        let sign = self.gen_sign(&query);

        let mut form_data = HashMap::new();
        form_data.insert(String::from("q"), query);
        form_data.insert(String::from("from"), String::from("auto"));
        form_data.insert(String::from("to"), String::from("zh-CHS"));
        form_data.insert(String::from("appKey"), self.app_id.clone());
        form_data.insert(String::from("salt"), self.salt.clone());
        form_data.insert(String::from("signType"), String::from("v3"));
        form_data.insert(String::from("curtime"), self.time.clone());
        form_data.insert(String::from("sign"), sign);
        form_data
    }

    pub fn format_output(&self, res: YoudaoTranslateResponse) -> String {
        let mut output = String::new();
        let empty_line = "\r\n";

        if res.error_code != "0" {
            output.push_str("failed to translate, error code is ");
            output.push_str(&res.error_code);
            return output;
        }

        output.push_str(empty_line);
        output.push_str("原文:");
        output.push_str(empty_line);
        output.push_str(&res.query.unwrap());
        output.push_str(empty_line);

        output.push_str(empty_line);
        output.push_str("译文:");
        output.push_str(empty_line);

        let items = res.translation.unwrap();
        for item in items {
            output.push_str(&item);
            output.push_str(empty_line);
        }

        output
    }

    pub async fn translate(&self, query: String) {
        let api_url = "https://openapi.youdao.com/api";
        let form_data = self.build_request_form(query);

        let response: YoudaoTranslateResponse = reqwest::Client::new()
            .post(api_url)
            .form(&form_data)
            .send()
            .await
            .expect("[youdao translate service]: failed to send request")
            .json()
            .await
            .expect("[youdao translate service]: failed to parse response");

        println!("{}", self.format_output(response))
    }
}
