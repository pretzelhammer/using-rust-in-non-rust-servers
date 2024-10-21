use std::{alloc::Layout, mem, slice, str};

// Host calls this function to allocate space where
// it can safely write data to
#[no_mangle]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(
        size * mem::size_of::<u8>(),
        mem::align_of::<usize>(),
    );
    std::alloc::alloc(layout)
}

// after allocating a text buffer and output buffer,
// Host calls this function to generate the QR code PNG
#[no_mangle]
pub unsafe extern "C" fn generateQrCode(
    text_ptr: *const u8,
    text_len: usize,
    output_ptr: *mut u8,
    output_len: usize,
) -> usize {
    // read text from memory, where it was written to by the Host
    let text_slice = slice::from_raw_parts(text_ptr, text_len);
    let text = str::from_utf8_unchecked(text_slice);

    let qr_code = match qr_lib::generate_qr_code(text) {
        Ok(png_data) => png_data,
        // error: unable to generate QR code
        Err(_) => return 0,
    };

    if qr_code.len() > output_len {
        // error: output buffer is too small
        return 0;
    }

    // write generated QR code PNG to output buffer,
    // where the Host will read it from after this
    // function returns
    let output_slice = slice::from_raw_parts_mut(output_ptr, qr_code.len());
    output_slice.copy_from_slice(&qr_code);

    // return written length of PNG data
    qr_code.len()
}
