pub struct Config {
    pub status_bar: StatusBarConfig,
    pub header_bar: HeaderBarConfig,
}

pub struct StatusBarConfig {
    pub enabled: bool
}

pub struct HeaderBarConfig {
    pub enabled: bool
}

pub const fn get_config() -> Config {
    return default_config();
}

pub const fn default_config() -> Config {
    Config {
        status_bar: StatusBarConfig {
            enabled: true
        },
        header_bar: HeaderBarConfig {
            enabled: true
        }
    }
}