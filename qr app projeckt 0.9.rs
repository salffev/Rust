use image::{Rgb, RgbImage};
use qrcodegen::{QrCode, QrCodeEcc};
use quirc_sys::{Quirc, ImageBuffer};

// qr kod oluşturma fonksiyonu-----------------------------------------
fn generate_qr_code(text: &str) -> RgbImage {
    // QR kodunu oluştur-----------------------------------------------
    let qr = QrCode::encode_text(text, QrCodeEcc::Medium).unwrap();
    let image = qr.to_image().unwrap();

    // rgbImage'e dönüştür---------------------------------------------
    let mut rgb_image = RgbImage::new(image.width() as u32, image.height() as u32);
    for y in 0..image.height() {
        for x in 0..image.width() {
            let color = image.get_pixel(x, y);
            rgb_image.put_pixel(x as u32, y as u32, Rgb([color[0], color[1], color[2]]));
        }
    }
    rgb_image
}

// qr kodunu okuma fonksiyonu-------------------------------------------
fn read_qr_code(image: RgbImage) -> Vec<String> {
    let mut quirc = Quirc::new().unwrap();
    let mut result = Vec::new();

    // görüntüyü quirc formatına dönüştür-------------------------------
    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut gray_image = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = image.get_pixel(x as u32, y as u32);
        let gray_value = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;
        Rgb([gray_value as u8, gray_value as u8, gray_value as u8])
    });

    // qr kodları bul---------------------------------------------------
    let codes = quirc.identify(&mut gray_image).unwrap();
    for code in codes {
        result.push(String::from_utf8_lossy(&code.data).to_string());
    }

    result
}

fn main() {
    // örnek ------------------------------------------------------------
    let text = "Merhaba, QR kodu!";
    
    // qr kodu oluştur ve kaydet-----------------------------------------
    let qr_code_image = generate_qr_code(text);
    qr_code_image.save("qrcode.png").unwrap();
    println!("QR kodu oluşturuldu ve 'qrcode.png' dosyasına kaydedildi.");

    // qr kodunu oku------------------------------------------------------
    let qr_code_image = image::open("qrcode.png").unwrap().to_rgb8();
    let result = read_qr_code(qr_code_image);

    // sonuçları ekrana yazdır--------------------------------------------
    println!("QR kodları:");
    for (i, code) in result.iter().enumerate() {
        println!("{}: {}", i+1, code);
    }
}
