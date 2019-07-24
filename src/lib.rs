use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn base64png(png: &[u8]) -> String {
    static HEADER: &str = "data:image/png;base64,";

    let mut out = String::with_capacity(png.len() + png.len() / 2 + HEADER.len());

    out.push_str(HEADER);

    base64::encode_config_buf(png, base64::STANDARD, &mut out);

    out
}

fn qrcode_bytes(data: &[u8]) -> Option<String> {
    use qrcodegen::{QrCode, QrCodeEcc};
    use pixelate::{Image, Color, BLACK};

    let qr = QrCode::encode_binary(data, QrCodeEcc::Medium).ok()?;

    let palette = &[Color::Rgba(255,255,255,0), BLACK];
    let mut pixels = Vec::with_capacity((qr.size() * qr.size()) as usize);

    for y in 0..qr.size() {
        for x in 0..qr.size() {
            pixels.push(qr.get_module(x, y) as u8);
        }
    }

    let mut result = Vec::new();

    Image {
        palette,
        pixels: &pixels,
        width: qr.size() as usize,
        scale: 16,
    }.render(&mut result).ok()?;

    Some(base64png(&pixels))
}

#[wasm_bindgen]
pub fn string(data: &str) -> Option<String> {
    qrcode_bytes(data.as_bytes())
}

#[wasm_bindgen]
pub fn binary(data: &[u8]) -> Option<String> {
    qrcode_bytes(data)
}
