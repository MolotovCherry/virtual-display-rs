use eframe::IconData;
use windows::{
    core::w,
    Win32::{
        Graphics::Gdi::{
            CreateCompatibleDC, DeleteDC, GetDIBits, GetObjectA, SelectObject, BITMAP, BITMAPINFO,
            BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            GetIconInfo, LoadImageW, HICON, ICONINFO, IMAGE_ICON, LR_DEFAULTCOLOR,
        },
    },
};

// Grab the icon from the exe and hand it over to egui
pub fn load_app_icon() -> IconData {
    let (mut buffer, width, height) = unsafe {
        let h_instance = GetModuleHandleW(None).expect("Failed to get HINSTANCE");
        let icon = LoadImageW(
            h_instance,
            w!("virtual"),
            IMAGE_ICON,
            512,
            512,
            LR_DEFAULTCOLOR,
        )
        .expect("Failed to load icon");

        let mut icon_info = ICONINFO::default();
        let res = GetIconInfo(HICON(icon.0), &mut icon_info as *mut _);
        if res.is_err() {
            panic!("Failed to load icon info");
        }

        let mut bitmap = BITMAP::default();
        GetObjectA(
            icon_info.hbmColor,
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bitmap as *mut _ as *mut _),
        );

        let width = bitmap.bmWidth;
        let height = bitmap.bmHeight;

        let b_size = (width * height * 4) as usize;
        let mut buffer = Vec::<u8>::with_capacity(b_size);

        let h_dc = CreateCompatibleDC(None);
        let h_bitmap = SelectObject(h_dc, icon_info.hbmColor);

        let mut bitmap_info = BITMAPINFO::default();
        bitmap_info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bitmap_info.bmiHeader.biWidth = width;
        bitmap_info.bmiHeader.biHeight = height;
        bitmap_info.bmiHeader.biPlanes = 1;
        bitmap_info.bmiHeader.biBitCount = 32;
        bitmap_info.bmiHeader.biCompression = BI_RGB.0;
        bitmap_info.bmiHeader.biSizeImage = 0;

        let res = GetDIBits(
            h_dc,
            icon_info.hbmColor,
            0,
            height as u32,
            Some(buffer.spare_capacity_mut().as_mut_ptr() as *mut _),
            &mut bitmap_info,
            DIB_RGB_COLORS,
        );
        if res == 0 {
            panic!("Failed to get RGB DI bits");
        }

        SelectObject(h_dc, h_bitmap);
        DeleteDC(h_dc);

        assert_eq!(
            bitmap_info.bmiHeader.biSizeImage as usize, b_size,
            "returned biSizeImage must equal to b_size"
        );

        // set the new size
        buffer.set_len(bitmap_info.bmiHeader.biSizeImage as usize);

        (buffer, width as u32, height as u32)
    };

    // RGBA -> BGRA
    for pixel in buffer.as_mut_slice().chunks_mut(4) {
        pixel.swap(0, 2);
    }

    // Flip the image vertically
    let row_size = width as usize * 4; // number of pixels in each row
    let row_count = buffer.len() / row_size; // number of rows in the image
    for row in 0..row_count / 2 {
        // loop through half of the rows
        let start = row * row_size; // index of the start of the current row
        let end = (row_count - row - 1) * row_size; // index of the end of the current row
        for i in 0..row_size {
            buffer.swap(start + i, end + i);
        }
    }

    IconData {
        rgba: buffer,
        width,
        height,
    }
}
