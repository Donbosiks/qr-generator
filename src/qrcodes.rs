use fast_qr::convert::ConvertError;
use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;

pub fn render_qr(_link: &str) -> Result<(), ConvertError> {
        // QRBuilder::new can fail if content is too big for version,
        // please check before unwrapping.
        let rend_link = format!("https://yuniversia.eu/{}", _link);
        let qrcode = QRBuilder::new(&*rend_link)
            .build()
            .unwrap();

        let path = format!("./tmp/qrcode/{}.png", _link);

        let _img = ImageBuilder::default()
            .shape(Shape::Square)
            .background_color([255, 255, 255, 255]) // Handles transparency
            .fit_width(600)
            .to_file(&qrcode, &path);

        Ok(())
}