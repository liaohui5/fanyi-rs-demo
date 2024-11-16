# <center>fanyi</center>

[toc]

## 介绍

学习 Rust 权威指南和一些第三方工具包之后的练手项目,
灵感来自 [这个项目](https://github.com/afc163/fanyi)

主要功能实现起来并不难, 但是需要学习一些第三方库, 如 `serde` `reqwest` `sha2` `md5` 等

## 主要功能和用法

0. 输出帮助信息
1. 初始化配置文件
2. 查看配置文件
3. 查看版本(这个暂时是写死的)

```sh
mini translate tool for command line interface, write in rust

Usage: fanyi [OPTIONS]                        # 用法1: 使用辅助功能
       fanyi "your need translation string"   # 用法2: 翻译文本

Options:
  -h, --help                Print help info and exit
  -v, --version             Print version info and exit
      --init-config         Initialize config file and exit
      --show-config         Print config file content and exit
```

## 配置文件说明

修改配置文件填入对应服务的 `app_id` 和 `secret`, 可以去到对应的教程中去查看如何申请

本来想支持多一些服务, 但是有些翻译服务的的api签名太变态了, 比如腾讯翻译

因为和其他服务的流程都是一样的, 就是签名比较难搞, 这写起来完全没有必要

```toml
# 默认使用哪个服务
default_service= "youdao"

[service.baidu]
# 服务开通教程: https://bobtranslate.com/service/translate/baidu.html
app_id = "your_app_id"
secret = "your_secret"

[service.youdao]
# 服务开通教程: https://bobtranslate.com/service/translate/youdao.html
app_id = "your_app_id"
secret = "your_secret"
```
