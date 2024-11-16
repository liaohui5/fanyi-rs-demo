use crate::cli::ArgsConfig;
use crate::config::{gen_config_template, Config};
use crate::services::baidu::BaiduTranslateService;
use crate::services::youdao::YoudaoTranslateService;
use std::fs;

// 运行命令的类
pub struct Action {
    config: Config,
}

impl Action {
    // 实例化
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // 执行入口
    pub async fn run(&self, args_config: ArgsConfig) {
        if args_config.is_show_help {
            self.print_help();
        } else if args_config.is_show_version {
            self.print_version();
        } else if args_config.is_show_config {
            self.print_config();
        } else if args_config.is_init_config {
            self.init_config();
        } else if args_config.is_translate {
            self.translate(args_config.query).await;
        }
    }

    // 输出帮助信息
    pub fn print_help(&self) {
        let help_message = r#"
mini translate tool for command line interface, write in rust

Usage: fanyi [OPTIONS]
       fanyi "your need translation string"

Options:
  -h, --help                Print help info and exit
  -v, --version             Print version info and exit
      --init-config         Initialize config file and exit
      --show-config         Print config file content and exit
        "#;
        println!("{}", help_message)
    }

    // 输出版本信息
    pub fn print_version(&self) {
        println!("fanyi v1.2.3");
    }

    // 查看配置文件内容
    pub fn print_config(&self) {
        if !self.config.is_initialized {
            let err_msg = "config file is not exists, Please execute `fanyi --init-config` first";
            eprintln!("{}", err_msg);
            return;
        }

        if let Some(config_content) = self.config.file_content.clone() {
            println!("\r\n{}", config_content);
        } else {
            eprintln!("failed to read config file content");
        }
    }

    // 初始化配置文件
    pub fn init_config(&self) {
        if self.config.is_initialized {
            eprintln!("config file already exists");
            return;
        }
        let config_template = gen_config_template();
        let config_filepath = &self.config.file_path;
        if let Err(e) = fs::write(config_filepath, config_template) {
            eprintln!("failed to write config file, {}", e);
        } else {
            println!("config initialized successfully: {}", config_filepath);
        }
    }

    // 执行翻译
    pub async fn translate(&self, query: String) {
        println!("translating...");
        if let Some(config) = &self.config.service_config {
            let service_name = &config.default_service;
            let service_keys = config.service.get(service_name).expect("not found default service in your config");

            match service_name.as_str() {
                "baidu" => {
                    let service_inst = BaiduTranslateService::new(service_keys);
                    service_inst.translate(query).await;
                }
                "youdao" => {
                    let service_inst = YoudaoTranslateService::new(service_keys);
                    service_inst.translate(query).await;
                }
                _ => {
                    panic!(
                        "unknown service: {}, please check your config file",
                        service_name
                    );
                }
            };
        }
    }
}
