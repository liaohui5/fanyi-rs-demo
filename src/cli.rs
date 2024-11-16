pub struct ArgsConfig {
    pub is_show_help: bool,
    pub is_show_version: bool,
    pub is_init_config: bool,
    pub is_show_config: bool,
    pub is_translate: bool,
    pub query: String,
}

impl ArgsConfig {
    pub fn new(args: &[String]) -> ArgsConfig {
        let len = args.len();
        let mut args_config = ArgsConfig {
            is_show_help: false,
            is_show_version: false,
            is_init_config: false,
            is_show_config: false,
            is_translate: false,
            query: String::new(),
        };

        // 如果参数有误, 如直接执行: `fanyi run`
        if len < 2 {
            args_config.is_show_help = true;
            return args_config;
        }

        // 判断是执行什么操作
        let input = args[1].clone();
        if input == "--help" || input == "-h" {
            args_config.is_show_help = true;
        } else if input == "--version" || input == "-v" {
            args_config.is_show_version = true;
        } else if input == "--init-config" {
            args_config.is_init_config = true;
        } else if input == "--show-config" {
            args_config.is_show_config = true;
        } else {
            args_config.is_translate = true;
            args_config.query = input;
        }

        args_config
    }
}

