

// qrcodes.rs is main file when we work with qrcode generation and this function




use fast_qr::convert::ConvertError;
use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;
use std::env;

pub fn render_qr(_link: &str, _qrtype: &str) -> Result<(), ConvertError> {

        let mut domain_url = env::var("DOMAIN_URL").expect("DOMAIN_URL must be set");

        domain_url += _link;

        let mut _rend_link = format!("{}", domain_url);

        // Generate Offline Qr_code

        if _qrtype == "offline" {
                _rend_link = format!("{}", _link);
        }

        // Compile Qr_code to file

        let qrcode = QRBuilder::new(&*_rend_link)
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