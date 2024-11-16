use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::From;
use std::fs;
use toml as serde_toml;

// 请求翻译服务接口需要的参数
#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceAPIKeys {
    pub app_id: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceConfig {
    pub default_service: String,
    pub service: HashMap<String, ServiceAPIKeys>,
}

// 配置类
pub struct Config {
    pub is_initialized: bool,
    pub file_path: String,
    pub file_content: Option<String>,
    pub service_config: Option<ServiceConfig>,
}

// 配置文件路径: ~/.fanyi-config.toml
pub fn get_config_file_path() -> String {
    let mut file_path = String::from(env!("HOME"));
    file_path.push_str("/.fanyi-config.toml");
    file_path
}

// 配置
impl Config {
    pub fn new() -> Self {
        let mut config = Self {
            is_initialized: is_config_file_exists(),
            file_path: get_config_file_path(),
            file_content: None,
            service_config: None,
        };

        if config.is_initialized {
            let file_content = read_config_from_file(&config.file_path);
            let data = parse_config(&file_content);

            config.service_config = Some(data);
            config.file_content = Some(file_content);
        }

        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

// 配置文件是否存在
fn is_config_file_exists() -> bool {
    let file_path = get_config_file_path();
    let path_inst = std::path::Path::new(&file_path);
    path_inst.exists() && path_inst.is_file()
}

// 读取配置文件内容
fn read_config_from_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("[Error]failed to read config file content")
}

// 将配置文件解析为 struct
fn parse_config(content: &str) -> ServiceConfig {
    serde_toml::from_str(content).expect("[Error]failed to parse config")
}

// 默认的配置文件模板
pub fn gen_config_template() -> String {
    let default_config_template = r#"
# default config file for fanyi
default_service= "youdao"

[service.baidu]
# https://bobtranslate.com/service/translate/baidu.html
app_id = "your_app_id"
secret = "your_secret"

[service.youdao]
# https://bobtranslate.com/service/translate/youdao.html
app_id = "your_app_id"
secret = "your_secret"
"#;
    String::from(default_config_template)
}
