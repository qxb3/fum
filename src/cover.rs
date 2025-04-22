use std::collections::HashMap;

use image::{DynamicImage, GenericImageView, Pixel};
use ratatui::style::Color;
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};

/// Cover of a track.
pub struct Cover {
    /// The stateful protocol of the cover.
    pub stateful_protocol: StatefulProtocol,

    /// Average color of the cover.
    pub avg_color: Color,
}

impl Cover {
    pub fn new(bytes: Vec<u8>, picker: &Picker) -> Self {
        // Decode cover image.
        let cover = image::ImageReader::new(std::io::Cursor::new(bytes))
            .with_guessed_format()
            .expect("Unknown cover art image file type")
            .decode()
            .expect("Failed to decode cover art image");

        // Calculate avg color.
        let avg_color = Cover::calculate_avg_color(&cover);

        // Creates a image StatefulProtocol.
        let protocol = picker.new_resize_protocol(cover);

        Self {
            stateful_protocol: protocol,
            avg_color,
        }
    }

    /// Calculates the average color of a cover dynamic image.
    pub fn calculate_avg_color(cover: &DynamicImage) -> Color {
        let mut rgb_map: HashMap<(u8, u8, u8), u32> = HashMap::new();

        // Loop thru all the pixel to add the the sum.
        for (_, _, color) in cover.pixels() {
            let [r, g, b] = color.to_rgb().0;

            if let Some(count) = rgb_map.get_mut(&(r, g, b)) {
                *count += 1;
            } else {
                rgb_map.insert((r, g, b), 1);
            }
        }

        // Get the highest rgb color from the map.
        let (avg_r, avg_g, avg_b) = rgb_map
            .iter()
            .max_by_key(|(_, &count)| count)
            .expect("Rgb map should not be empty")
            .0;

        Color::Rgb(*avg_r, *avg_g, *avg_b)
    }
}
