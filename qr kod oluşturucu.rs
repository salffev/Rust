use qrcode::{QrCode, Version, EcLevel};
use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn generate_qr_code(text: &str) -> QrCode {
    QrCode::new(text, Version::V1, EcLevel::L)
}

fn save_qr_code(qr_code: &QrCode, filename: &str) -> Result<(), io::Error> {
    let image = qr_code.render().build();
    image.save(filename)
}

fn main() {
    println!("QR kod için metni giriniz:");
    let text = get_input();

    let qr_code = generate_qr_code(&text);
    let result = save_qr_code(&qr_code, "qr_code.png");

    match result {
        Ok(()) => println!("QR kod başarıyla oluşturuldu!"),
        Err(error) => println!("Hata: {}", error),
    }
}
