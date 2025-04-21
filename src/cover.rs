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
        // Sum of all the color pixel.
        let mut sum_r = 0;
        let mut sum_g = 0;
        let mut sum_b = 0;

        // Loop thru all the pixel to add the the sum.
        for (_, _, color) in cover.pixels() {
            let color = color.to_rgb().0;

            sum_r += color[0] as u32;
            sum_g += color[1] as u32;
            sum_b += color[2] as u32;
        }

        // Gets the total pixel.
        let total_pixel = cover.width() * cover.height();

        let avg_r = (sum_r / total_pixel) as u8;
        let avg_g = (sum_g / total_pixel) as u8;
        let avg_b = (sum_b / total_pixel) as u8;

        Color::Rgb(avg_r, avg_g, avg_b)
    }
}
