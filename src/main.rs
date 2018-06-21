extern crate png;

use std::fs::File;
use std::io::prelude::*;

const ASAN_DEFAULT_OPTIONS: &'static [u8] = b"detect_odr_violation=1\0";

#[no_mangle]
pub extern "C" fn __asan_default_options() -> *const u8 {
    ASAN_DEFAULT_OPTIONS as *const [u8] as *const u8
}

#[inline(always)]
fn png_decode(data: &[u8]) -> Result<(png::OutputInfo, Vec<u8>), ()> {
    let decoder = png::Decoder::new(data);
    let (info, mut reader) = decoder.read_info().map_err(|_| ())?;

    if info.buffer_size() > 50_000_000 {
        return Err(());
    }

    let mut img_data = Vec::with_capacity(info.buffer_size());
    reader.next_frame(&mut img_data).map_err(|_| ())?;

    Ok((info, img_data))
}


fn main() {
    let mut file = File::open("in.png").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data);
    loop {
        let _result = png_decode(&data);
    }
}
