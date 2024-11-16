use crate::config::ServiceAPIKeys;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 百度翻译响应
#[derive(Serialize, Deserialize, Debug)]
pub struct BaiduTranslateItem {
    pub src: String,
    pub dst: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaiduTranslateResponse {
    pub from: String,
    pub to: String,
    pub trans_result: Vec<BaiduTranslateItem>,
}

// 百度翻译服务类
#[derive(Debug)]
pub struct BaiduTranslateService {
    pub app_id: String,
    pub secret: String,
    pub salt: String,
}

// 为百度翻译服务类实现翻译特性
impl BaiduTranslateService {
    // 实例化
    pub fn new(keys: &ServiceAPIKeys) -> BaiduTranslateService {
        let app_id = keys.app_id.clone();
        let secret = keys.secret.clone();
        let salt = String::from(&app_id[0..8]);
        BaiduTranslateService {
            app_id,
            secret,
            salt,
        }
    }

    // 生成接口需要的签名
    fn gen_sign(&self, q: &str) -> String {
        // sign = appid+query+salt+secret => md5
        let mut origin_str = String::new();

        origin_str.push_str(&self.app_id);
        origin_str.push_str(q);
        origin_str.push_str(&self.salt);
        origin_str.push_str(&self.secret);

        let sign_str = md5::compute(origin_str.as_bytes());

        format!("{:x}", sign_str)
    }

    // 生成接口需要的表单数据
    fn build_request_form(&self, q: &str) -> HashMap<String, String> {
        let mut form_data = HashMap::new();
        form_data.insert("q".to_string(), q.to_string());
        form_data.insert("from".to_string(), "auto".to_string());
        form_data.insert("to".to_string(), "zh".to_string());
        form_data.insert("appid".to_string(), self.app_id.clone());
        form_data.insert("salt".to_string(), self.salt.clone());

        let sign = self.gen_sign(q);
        form_data.insert("sign".to_string(), sign);

        form_data
    }

    // 发送翻译请求
    pub async fn translate(&self, query: String) {
        let api_url = "https://fanyi-api.baidu.com/api/trans/vip/translate";
        let form_data = self.build_request_form(&query);

        let response: BaiduTranslateResponse = reqwest::Client::new()
            .post(api_url)
            .form(&form_data)
            .send()
            .await
            .expect("[baidu translate service]: failed to send request")
            .json()
            .await
            .expect("[baidu translate service]: failed to parse response");

        println!("{}", self.format_output(response))
    }

    // 格式化请求的响应
    pub fn format_output(&self, res: BaiduTranslateResponse) -> String {
        let mut output = String::new();
        let empty_line = "\r\n";

        for item in res.trans_result {
            output.push_str(empty_line);
            output.push_str("原文:");
            output.push_str(empty_line);
            output.push_str(&item.src);
            output.push_str(empty_line);

            output.push_str(empty_line);
            output.push_str("译文:");
            output.push_str(empty_line);
            output.push_str(&item.dst);
            output.push_str(empty_line);
        }

        output
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn create_instance() -> BaiduTranslateService {
        let app_id = String::from("123456789");
        let secret = String::from("abcdefghi");
        let keys = ServiceAPIKeys { app_id, secret };
        BaiduTranslateService::new(&keys)
    }

    #[test]
    fn should_be_create_translate_instance() {
        let inst = self::create_instance();

        let expected_app_id = String::from("123456789");
        let expected_secret = String::from("abcdefghi");
        let expected_salt = String::from("12345678");
        assert_eq!(inst.app_id, expected_app_id);
        assert_eq!(inst.secret, expected_secret);
        assert_eq!(inst.salt, expected_salt);
    }

    #[test]
    fn should_generate_sign() {
        let inst = self::create_instance();

        // origin_str: 123456789hello12345678abcedfghi
        // signed_str: 0bb52b2236de734b0859185422382b72
        let sign_str = inst.gen_sign("hello");
        let expected = String::from("0bb52b2236de734b0859185422382b72");
        assert_eq!(sign_str, expected);
    }

    #[test]
    fn should_build_request_form_data() {
        let inst = self::create_instance();

        let query = String::from("hello");
        let sign = inst.gen_sign(&query);
        let form = inst.build_request_form(&query);

        assert_eq!(form.get("q").unwrap(), "hello");
        assert_eq!(form.get("from").unwrap(), "auto");
        assert_eq!(form.get("to").unwrap(), "zh");
        assert_eq!(form.get("appid").unwrap(), inst.app_id.as_str());
        assert_eq!(form.get("salt").unwrap(), inst.salt.as_str());
        assert_eq!(form.get("sign").unwrap(), &sign);
    }
}
