use ratatui::layout::Alignment;
use const_random::const_random;

const CUSTOM_STRINGS: [&str; 1] = [
    "It's gonna be a good day!",
];

pub struct Config {
    pub status_bar: StatusBarConfig,
    pub header_bar: HeaderBarConfig,
}

pub struct StatusBarConfig {
    pub enabled: bool
}

pub struct HeaderBarConfig {
    pub enabled: bool,
    pub show_title: bool,
    pub show_version: bool,
    pub custom_header_text: Option<&'static str>,
    pub custom_header_alignment: Alignment,
    pub title_alignment: Alignment,
    pub version_alignment: Alignment,
}

pub const fn get_config() -> Config {
    return default_config();
}

pub const fn default_config() -> Config {
    const random_index: u8 = const_random!(u8);
   
    Config {
        status_bar: StatusBarConfig {
            enabled: true
        },
        header_bar: HeaderBarConfig {
            enabled: true,
            show_title: true,
            title_alignment: Alignment::Right,
            show_version: true,
            version_alignment: Alignment::Right,
            custom_header_text: Some(CUSTOM_STRINGS[random_index]),
            custom_header_alignment: Alignment::Left
        }
    }
}