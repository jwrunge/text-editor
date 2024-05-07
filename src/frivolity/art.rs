use crossterm::style::Color;

pub fn get_art() -> String {
    match std::fs::read_to_string("assets/mylodon.txt") {
        Ok(art) => art,
        Err(_) => String::from("Couldn't load the art file!"),
    }
}

// pub fn color_wave(delta: i32, pixel_width: i32, pixel_height: i32) {
//     let mut wave = String::new();
//     for y in 0..pixel_height {
//         for x in 0..pixel_width {
//             let color = Color::Fixed((x + y + delta) as u8 % 16).paint(" ");
//             wave.push_str(&format!("\x1b[48;5;{}m ", color));
//         }
//         wave.push_str("\x1b[0m\n");
//     }
//     print!("{}", wave);
// }