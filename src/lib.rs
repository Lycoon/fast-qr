#[cfg(test)]
mod tests;

pub mod comptime;

use comptime::qrcode::QRCode;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn array_to_vec<const N: usize>(mat: [[bool; N]; N]) -> Vec<u8> {
    const MODULE_PAD_WIDTH: usize = 4;
    const MODULE_PAD_HEIGHT: usize = 4;
    const MODULE_WIDTH: usize = 8;
    const WHITE_PIXEL: [u8; 4] = [255, 255, 255, 255];
    const BLACK_PIXEL: [u8; 4] = [0, 0, 0, 255];

    let mut vecu8 = Vec::<u8>::with_capacity((8 + N) * 8 * (8 + N) * 8 * 4);

    fn pad_width(vec: &mut Vec<u8>) {
        // for _ in 0..MODULE_PAD_WIDTH {
        //     for _ in 0..MODULE_WIDTH {
        //         vec.extend_from_slice(&WHITE_PIXEL);
        //     }
        // }

        vec.extend(std::iter::repeat(255).take(MODULE_PAD_WIDTH * MODULE_WIDTH * 4));
    }

    fn pad_height<const N: usize>(vec: &mut Vec<u8>) {
        vec.extend(std::iter::repeat(255).take(
            MODULE_WIDTH
                * MODULE_WIDTH
                * MODULE_PAD_WIDTH
                * MODULE_PAD_HEIGHT
                * (N + (MODULE_PAD_WIDTH + MODULE_PAD_HEIGHT)),
        ));
    }

    pad_height::<N>(&mut vecu8);
    for i in 0..N {
        for _ in 0..MODULE_WIDTH {
            pad_width(&mut vecu8);
            for j in 0..N {
                for _ in 0..MODULE_WIDTH {
                    let pixels = match mat[i][j] {
                        true => &BLACK_PIXEL,
                        false => &WHITE_PIXEL,
                    };

                    vecu8.extend_from_slice(pixels);
                }
            }
            pad_width(&mut vecu8);
        }
    }
    pad_height::<N>(&mut vecu8);

    return vecu8;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn qr(content: &str) -> Vec<u8> {
    let qrcode: Option<QRCode> = QRCode::new(content.as_bytes(), None, None, None);

    if let Some(q) = qrcode {
        return match q {
            QRCode::V1(m) => array_to_vec(m),
            QRCode::V2(m) => array_to_vec(m),
            QRCode::V3(m) => array_to_vec(m),
            QRCode::V4(m) => array_to_vec(m),
            QRCode::V5(m) => array_to_vec(m),
            QRCode::V6(m) => array_to_vec(m),
            QRCode::V7(m) => array_to_vec(m),
            QRCode::V8(m) => array_to_vec(m),
            QRCode::V9(m) => array_to_vec(m),
            QRCode::V10(m) => array_to_vec(m),
            QRCode::V11(m) => array_to_vec(m),
            QRCode::V12(m) => array_to_vec(m),
            QRCode::V13(m) => array_to_vec(m),
            QRCode::V14(m) => array_to_vec(m),
            QRCode::V15(m) => array_to_vec(m),
            QRCode::V16(m) => array_to_vec(m),
            QRCode::V17(m) => array_to_vec(m),
            QRCode::V18(m) => array_to_vec(m),
            QRCode::V19(m) => array_to_vec(m),
            QRCode::V20(m) => array_to_vec(m),
            QRCode::V21(m) => array_to_vec(m),
            QRCode::V22(m) => array_to_vec(m),
            QRCode::V23(m) => array_to_vec(m),
            QRCode::V24(m) => array_to_vec(m),
            QRCode::V25(m) => array_to_vec(m),
            QRCode::V26(m) => array_to_vec(m),
            QRCode::V27(m) => array_to_vec(m),
            QRCode::V28(m) => array_to_vec(m),
            QRCode::V29(m) => array_to_vec(m),
            QRCode::V30(m) => array_to_vec(m),
            QRCode::V31(m) => array_to_vec(m),
            QRCode::V32(m) => array_to_vec(m),
            QRCode::V33(m) => array_to_vec(m),
            QRCode::V34(m) => array_to_vec(m),
            QRCode::V35(m) => array_to_vec(m),
            QRCode::V36(m) => array_to_vec(m),
            QRCode::V37(m) => array_to_vec(m),
            QRCode::V38(m) => array_to_vec(m),
            QRCode::V39(m) => array_to_vec(m),
            QRCode::V40(m) => array_to_vec(m),
        };
    }

    return Vec::new();
}
