use std::fs;
use std::path::Path;

fn push_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn build_ico() -> Vec<u8> {
    const SIZE: usize = 32;
    const XOR_BYTES: usize = SIZE * SIZE * 4;
    const MASK_BYTES: usize = SIZE * (SIZE / 8);
    const DIB_SIZE: u32 = 40;
    const IMAGE_SIZE: u32 = (XOR_BYTES + MASK_BYTES) as u32;
    const IMAGE_OFFSET: u32 = 22;

    let mut pixels = vec![[20u8, 105u8, 62u8, 255u8]; XOR_BYTES / 4];

    // Simple aviation-green NATRA mark: a white N built into the icon.
    for y in 5..27 {
        for x in 5..27 {
            let left = x == 5 || x == 6;
            let right = x == 25 || x == 26;
            let diagonal = x as isize == 5 + ((y - 5) as isize * 20 / 21);
            if left || right || diagonal {
                pixels[y * SIZE + x] = [255, 255, 255, 255];
            }
        }
    }

    let mut out = Vec::with_capacity(IMAGE_OFFSET as usize + DIB_SIZE as usize + IMAGE_SIZE as usize);

    // ICO header.
    push_u16(&mut out, 0);
    push_u16(&mut out, 1);
    push_u16(&mut out, 1);

    // One 32x32, 32-bit image.
    out.push(SIZE as u8);
    out.push(SIZE as u8);
    out.push(0);
    out.push(0);
    push_u16(&mut out, 1);
    push_u16(&mut out, 32);
    push_u32(&mut out, IMAGE_SIZE);
    push_u32(&mut out, IMAGE_OFFSET);

    // BITMAPINFOHEADER. ICO DIB height is doubled for the AND mask.
    push_u32(&mut out, DIB_SIZE);
    push_u32(&mut out, SIZE as u32);
    push_u32(&mut out, (SIZE * 2) as u32);
    push_u16(&mut out, 1);
    push_u16(&mut out, 32);
    push_u32(&mut out, 0);
    push_u32(&mut out, XOR_BYTES as u32);
    push_u32(&mut out, 0);
    push_u32(&mut out, 0);
    push_u32(&mut out, 0);
    push_u32(&mut out, 0);

    // XOR bitmap is stored bottom-up and uses BGRA pixels.
    for y in (0..SIZE).rev() {
        for x in 0..SIZE {
            let [r, g, b, a] = pixels[y * SIZE + x];
            out.extend_from_slice(&[b, g, r, a]);
        }
    }

    // AND transparency mask: all zero means fully opaque.
    out.resize(out.len() + MASK_BYTES, 0);
    out
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let icons_dir = Path::new("icons");
    fs::create_dir_all(icons_dir).expect("failed to create src-tauri/icons");
    fs::write(icons_dir.join("icon.ico"), build_ico())
        .expect("failed to generate src-tauri/icons/icon.ico");

    tauri_build::build();
}
