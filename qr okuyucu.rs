import camera;
import qrcode;

pub fn main() {
    let mut camera = camera.new();
    let mut qr_code_reader = qrcode.new();

    loop {
        let image = camera.capture();
        let maybe_qr_code = qr_code_reader.decode(image);

        match maybe_qr_code {
            Some(qr_code) => println!("QR Kod: {}", qr_code),
            None => println!("QR Kod Bulunamadı"),
        }
    }
}
