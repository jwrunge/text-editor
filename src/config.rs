use ratatui::layout::Alignment;

pub enum ConditionallyEnabled {
    Enabled,
    Disabled,
    HeightBased,
}

pub struct Config {
    pub status_bar: StatusBarConfig,
    pub header_bar: HeaderBarConfig,
}

pub struct StatusBarConfig {
    pub enabled: bool
}

pub struct HeaderBarConfig {
    pub enabled: ConditionallyEnabled,
    pub height_cutoff: u16,
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
    Config {
        status_bar: StatusBarConfig {
            enabled: true
        },
        header_bar: HeaderBarConfig {
            enabled: ConditionallyEnabled::HeightBased,
            height_cutoff: 20,
            show_title: true,
            title_alignment: Alignment::Right,
            show_version: true,
            version_alignment: Alignment::Right,
            custom_header_text: Some("It's gonna be a good day!"),
            custom_header_alignment: Alignment::Left
        }
    }
}