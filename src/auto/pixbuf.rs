// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Colorspace;
use Error;
use InterpType;
use PixbufFormat;
use PixbufRotation;
use ffi;
use gio;
use gio_ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Pixbuf(Object<ffi::GdkPixbuf>): [
        gio::Icon => gio_ffi::GIcon,
        gio::LoadableIcon => gio_ffi::GLoadableIcon,
    ];

    match fn {
        get_type => || ffi::gdk_pixbuf_get_type(),
    }
}

impl Pixbuf {
    pub fn new(colorspace: Colorspace, has_alpha: bool, bits_per_sample: i32, width: i32, height: i32) -> Pixbuf {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_new(colorspace.to_glib(), has_alpha.to_glib(), bits_per_sample, width, height))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn new_from_bytes(data: &glib::Bytes, colorspace: Colorspace, has_alpha: bool, bits_per_sample: i32, width: i32, height: i32, rowstride: i32) -> Pixbuf {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_new_from_bytes(data.to_glib_none().0, colorspace.to_glib(), has_alpha.to_glib(), bits_per_sample, width, height, rowstride))
        }
    }

    //pub fn new_from_data<'a, P: Into<Option<&'a /*Unimplemented*/PixbufDestroyNotify>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(data: &[u8], colorspace: Colorspace, has_alpha: bool, bits_per_sample: i32, width: i32, height: i32, rowstride: i32, destroy_fn: P, destroy_fn_data: Q) -> Pixbuf {
    //    unsafe { TODO: call ffi::gdk_pixbuf_new_from_data() }
    //}

    #[cfg_attr(feature = "v2_32", deprecated)]
    pub fn new_from_inline(data: &[u8], copy_pixels: bool) -> Result<Pixbuf, Error> {
        let data_length = data.len() as i32;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_new_from_inline(data_length, data.to_glib_none().0, copy_pixels.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_resource(resource_path: &str) -> Result<Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_new_from_resource(resource_path.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_resource_at_scale(resource_path: &str, width: i32, height: i32, preserve_aspect_ratio: bool) -> Result<Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_new_from_resource_at_scale(resource_path.to_glib_none().0, width, height, preserve_aspect_ratio.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_stream<'a, P: IsA<gio::InputStream>, Q: Into<Option<&'a gio::Cancellable>>>(stream: &P, cancellable: Q) -> Result<Pixbuf, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_new_from_stream(stream.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_stream_at_scale<'a, P: IsA<gio::InputStream>, Q: Into<Option<&'a gio::Cancellable>>>(stream: &P, width: i32, height: i32, preserve_aspect_ratio: bool, cancellable: Q) -> Result<Pixbuf, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_new_from_stream_at_scale(stream.to_glib_none().0, width, height, preserve_aspect_ratio.to_glib(), cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_xpm_data(data: &[&str]) -> Pixbuf {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_new_from_xpm_data(data.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_36_8", feature = "dox"))]
    pub fn calculate_rowstride(colorspace: Colorspace, has_alpha: bool, bits_per_sample: i32, width: i32, height: i32) -> i32 {
        unsafe {
            ffi::gdk_pixbuf_calculate_rowstride(colorspace.to_glib(), has_alpha.to_glib(), bits_per_sample, width, height)
        }
    }

    //#[cfg_attr(feature = "v2_32", deprecated)]
    //pub fn from_pixdata(pixdata: /*Ignored*/&Pixdata, copy_pixels: bool) -> Result<Pixbuf, Error> {
    //    unsafe { TODO: call ffi::gdk_pixbuf_from_pixdata() }
    //}

    pub fn get_formats() -> Vec<PixbufFormat> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_pixbuf_get_formats())
        }
    }
}

pub trait PixbufExt {
    fn add_alpha(&self, substitute_color: bool, r: u8, g: u8, b: u8) -> Option<Pixbuf>;

    fn apply_embedded_orientation(&self) -> Option<Pixbuf>;

    fn composite(&self, dest: &Pixbuf, dest_x: i32, dest_y: i32, dest_width: i32, dest_height: i32, offset_x: f64, offset_y: f64, scale_x: f64, scale_y: f64, interp_type: InterpType, overall_alpha: i32);

    fn composite_color(&self, dest: &Pixbuf, dest_x: i32, dest_y: i32, dest_width: i32, dest_height: i32, offset_x: f64, offset_y: f64, scale_x: f64, scale_y: f64, interp_type: InterpType, overall_alpha: i32, check_x: i32, check_y: i32, check_size: i32, color1: u32, color2: u32);

    fn composite_color_simple(&self, dest_width: i32, dest_height: i32, interp_type: InterpType, overall_alpha: i32, check_size: i32, color1: u32, color2: u32) -> Option<Pixbuf>;

    fn copy(&self) -> Option<Pixbuf>;

    fn copy_area(&self, src_x: i32, src_y: i32, width: i32, height: i32, dest_pixbuf: &Pixbuf, dest_x: i32, dest_y: i32);

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn copy_options(&self, dest_pixbuf: &Pixbuf) -> bool;

    fn fill(&self, pixel: u32);

    fn flip(&self, horizontal: bool) -> Option<Pixbuf>;

    fn get_bits_per_sample(&self) -> i32;

    fn get_byte_length(&self) -> usize;

    fn get_colorspace(&self) -> Colorspace;

    fn get_has_alpha(&self) -> bool;

    fn get_height(&self) -> i32;

    fn get_n_channels(&self) -> i32;

    fn get_option(&self, key: &str) -> Option<String>;

    //#[cfg(any(feature = "v2_32", feature = "dox"))]
    //fn get_options(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 };

    fn get_rowstride(&self) -> i32;

    fn get_width(&self) -> i32;

    fn new_subpixbuf(&self, src_x: i32, src_y: i32, width: i32, height: i32) -> Option<Pixbuf>;

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    fn read_pixel_bytes(&self) -> Option<glib::Bytes>;

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn remove_option(&self, key: &str) -> bool;

    fn rotate_simple(&self, angle: PixbufRotation) -> Option<Pixbuf>;

    fn saturate_and_pixelate(&self, dest: &Pixbuf, saturation: f32, pixelate: bool);

    //fn save<'a, P: Into<Option<&'a Error>>>(&self, filename: &str, type_: &str, error: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool;

    //fn save_to_buffer<'a, P: Into<Option<&'a Error>>>(&self, type_: &str, error: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Vec<u8>>;

    //fn save_to_callback<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a Error>>>(&self, save_func: /*Unknown conversion*//*Unimplemented*/PixbufSaveFunc, user_data: P, type_: &str, error: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool;

    //fn save_to_callbackv<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, save_func: /*Unknown conversion*//*Unimplemented*/PixbufSaveFunc, user_data: P, type_: &str, option_keys: &[&str], option_values: &[&str]) -> Result<(), Error>;

    //fn save_to_stream<'a, 'b, P: IsA<gio::OutputStream>, Q: Into<Option<&'a gio::Cancellable>>, R: Into<Option<&'b Error>>>(&self, stream: &P, type_: &str, cancellable: Q, error: R, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool;

    //fn save_to_stream_async<'a, P: IsA<gio::OutputStream>, Q: Into<Option<&'a gio::Cancellable>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, stream: &P, type_: &str, cancellable: Q, callback: R, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //#[cfg(feature = "futures")]
    //fn save_to_stream_async_future<P: IsA<gio::OutputStream> + Clone + 'static>(&self, stream: &P, type_: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    fn scale(&self, dest: &Pixbuf, dest_x: i32, dest_y: i32, dest_width: i32, dest_height: i32, offset_x: f64, offset_y: f64, scale_x: f64, scale_y: f64, interp_type: InterpType);

    fn scale_simple(&self, dest_width: i32, dest_height: i32, interp_type: InterpType) -> Option<Pixbuf>;

    fn set_option(&self, key: &str, value: &str) -> bool;

    fn get_property_pixel_bytes(&self) -> Option<glib::Bytes>;

    //fn get_property_pixels(&self) -> /*Unimplemented*/Fundamental: Pointer;

    fn connect_property_bits_per_sample_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_colorspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_n_channels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixel_bytes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rowstride_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Pixbuf> + IsA<glib::object::Object>> PixbufExt for O {
    fn add_alpha(&self, substitute_color: bool, r: u8, g: u8, b: u8) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_add_alpha(const_override(self.to_glib_none().0), substitute_color.to_glib(), r, g, b))
        }
    }

    fn apply_embedded_orientation(&self) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_apply_embedded_orientation(self.to_glib_none().0))
        }
    }

    fn composite(&self, dest: &Pixbuf, dest_x: i32, dest_y: i32, dest_width: i32, dest_height: i32, offset_x: f64, offset_y: f64, scale_x: f64, scale_y: f64, interp_type: InterpType, overall_alpha: i32) {
        unsafe {
            ffi::gdk_pixbuf_composite(const_override(self.to_glib_none().0), dest.to_glib_none().0, dest_x, dest_y, dest_width, dest_height, offset_x, offset_y, scale_x, scale_y, interp_type.to_glib(), overall_alpha);
        }
    }

    fn composite_color(&self, dest: &Pixbuf, dest_x: i32, dest_y: i32, dest_width: i32, dest_height: i32, offset_x: f64, offset_y: f64, scale_x: f64, scale_y: f64, interp_type: InterpType, overall_alpha: i32, check_x: i32, check_y: i32, check_size: i32, color1: u32, color2: u32) {
        unsafe {
            ffi::gdk_pixbuf_composite_color(const_override(self.to_glib_none().0), dest.to_glib_none().0, dest_x, dest_y, dest_width, dest_height, offset_x, offset_y, scale_x, scale_y, interp_type.to_glib(), overall_alpha, check_x, check_y, check_size, color1, color2);
        }
    }

    fn composite_color_simple(&self, dest_width: i32, dest_height: i32, interp_type: InterpType, overall_alpha: i32, check_size: i32, color1: u32, color2: u32) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_composite_color_simple(const_override(self.to_glib_none().0), dest_width, dest_height, interp_type.to_glib(), overall_alpha, check_size, color1, color2))
        }
    }

    fn copy(&self) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_copy(const_override(self.to_glib_none().0)))
        }
    }

    fn copy_area(&self, src_x: i32, src_y: i32, width: i32, height: i32, dest_pixbuf: &Pixbuf, dest_x: i32, dest_y: i32) {
        unsafe {
            ffi::gdk_pixbuf_copy_area(const_override(self.to_glib_none().0), src_x, src_y, width, height, dest_pixbuf.to_glib_none().0, dest_x, dest_y);
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn copy_options(&self, dest_pixbuf: &Pixbuf) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_copy_options(self.to_glib_none().0, dest_pixbuf.to_glib_none().0))
        }
    }

    fn fill(&self, pixel: u32) {
        unsafe {
            ffi::gdk_pixbuf_fill(self.to_glib_none().0, pixel);
        }
    }

    fn flip(&self, horizontal: bool) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_flip(const_override(self.to_glib_none().0), horizontal.to_glib()))
        }
    }

    fn get_bits_per_sample(&self) -> i32 {
        unsafe {
            ffi::gdk_pixbuf_get_bits_per_sample(const_override(self.to_glib_none().0))
        }
    }

    fn get_byte_length(&self) -> usize {
        unsafe {
            ffi::gdk_pixbuf_get_byte_length(const_override(self.to_glib_none().0))
        }
    }

    fn get_colorspace(&self) -> Colorspace {
        unsafe {
            from_glib(ffi::gdk_pixbuf_get_colorspace(const_override(self.to_glib_none().0)))
        }
    }

    fn get_has_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_get_has_alpha(const_override(self.to_glib_none().0)))
        }
    }

    fn get_height(&self) -> i32 {
        unsafe {
            ffi::gdk_pixbuf_get_height(const_override(self.to_glib_none().0))
        }
    }

    fn get_n_channels(&self) -> i32 {
        unsafe {
            ffi::gdk_pixbuf_get_n_channels(const_override(self.to_glib_none().0))
        }
    }

    fn get_option(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_get_option(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v2_32", feature = "dox"))]
    //fn get_options(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
    //    unsafe { TODO: call ffi::gdk_pixbuf_get_options() }
    //}

    fn get_rowstride(&self) -> i32 {
        unsafe {
            ffi::gdk_pixbuf_get_rowstride(const_override(self.to_glib_none().0))
        }
    }

    fn get_width(&self) -> i32 {
        unsafe {
            ffi::gdk_pixbuf_get_width(const_override(self.to_glib_none().0))
        }
    }

    fn new_subpixbuf(&self, src_x: i32, src_y: i32, width: i32, height: i32) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_new_subpixbuf(self.to_glib_none().0, src_x, src_y, width, height))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    fn read_pixel_bytes(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_read_pixel_bytes(const_override(self.to_glib_none().0)))
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn remove_option(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_remove_option(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn rotate_simple(&self, angle: PixbufRotation) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_rotate_simple(const_override(self.to_glib_none().0), angle.to_glib()))
        }
    }

    fn saturate_and_pixelate(&self, dest: &Pixbuf, saturation: f32, pixelate: bool) {
        unsafe {
            ffi::gdk_pixbuf_saturate_and_pixelate(const_override(self.to_glib_none().0), dest.to_glib_none().0, saturation, pixelate.to_glib());
        }
    }

    //fn save<'a, P: Into<Option<&'a Error>>>(&self, filename: &str, type_: &str, error: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi::gdk_pixbuf_save() }
    //}

    //fn save_to_buffer<'a, P: Into<Option<&'a Error>>>(&self, type_: &str, error: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Vec<u8>> {
    //    unsafe { TODO: call ffi::gdk_pixbuf_save_to_buffer() }
    //}

    //fn save_to_callback<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a Error>>>(&self, save_func: /*Unknown conversion*//*Unimplemented*/PixbufSaveFunc, user_data: P, type_: &str, error: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi::gdk_pixbuf_save_to_callback() }
    //}

    //fn save_to_callbackv<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, save_func: /*Unknown conversion*//*Unimplemented*/PixbufSaveFunc, user_data: P, type_: &str, option_keys: &[&str], option_values: &[&str]) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::gdk_pixbuf_save_to_callbackv() }
    //}

    //fn save_to_stream<'a, 'b, P: IsA<gio::OutputStream>, Q: Into<Option<&'a gio::Cancellable>>, R: Into<Option<&'b Error>>>(&self, stream: &P, type_: &str, cancellable: Q, error: R, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi::gdk_pixbuf_save_to_stream() }
    //}

    //fn save_to_stream_async<'a, P: IsA<gio::OutputStream>, Q: Into<Option<&'a gio::Cancellable>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, stream: &P, type_: &str, cancellable: Q, callback: R, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gdk_pixbuf_save_to_stream_async() }
    //}

    //#[cfg(feature = "futures")]
    //fn save_to_stream_async_future<P: IsA<gio::OutputStream> + Clone + 'static>(&self, stream: &P, type_: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        //use gio::GioFuture;
        //use send_cell::SendCell;

        //let stream = stream.clone();
        //let type_ = String::from(type_);
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = SendCell::new(send);
        //    let obj_clone = SendCell::new(obj.clone());
        //    obj.save_to_stream_async(
        //         &stream,
        //         &type_,
        //         Some(&cancellable),
        //         ,
        //         move |res| {
        //             let obj = obj_clone.into_inner();
        //             let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
        //             let _ = send.into_inner().send(res);
        //         },
        //    );

        //    cancellable
        //})
    //}

    fn scale(&self, dest: &Pixbuf, dest_x: i32, dest_y: i32, dest_width: i32, dest_height: i32, offset_x: f64, offset_y: f64, scale_x: f64, scale_y: f64, interp_type: InterpType) {
        unsafe {
            ffi::gdk_pixbuf_scale(const_override(self.to_glib_none().0), dest.to_glib_none().0, dest_x, dest_y, dest_width, dest_height, offset_x, offset_y, scale_x, scale_y, interp_type.to_glib());
        }
    }

    fn scale_simple(&self, dest_width: i32, dest_height: i32, interp_type: InterpType) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_scale_simple(const_override(self.to_glib_none().0), dest_width, dest_height, interp_type.to_glib()))
        }
    }

    fn set_option(&self, key: &str, value: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_set_option(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0))
        }
    }

    fn get_property_pixel_bytes(&self) -> Option<glib::Bytes> {
        unsafe {
            let mut value = Value::from_type(<glib::Bytes as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pixel-bytes".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    //fn get_property_pixels(&self) -> /*Unimplemented*/Fundamental: Pointer {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "pixels".to_glib_none().0, value.to_glib_none_mut().0);
    //        value.get().unwrap()
    //    }
    //}

    fn connect_property_bits_per_sample_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::bits-per-sample",
                transmute(notify_bits_per_sample_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_colorspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::colorspace",
                transmute(notify_colorspace_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_has_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::has-alpha",
                transmute(notify_has_alpha_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::height",
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_n_channels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::n-channels",
                transmute(notify_n_channels_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pixel_bytes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pixel-bytes",
                transmute(notify_pixel_bytes_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pixels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pixels",
                transmute(notify_pixels_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_rowstride_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::rowstride",
                transmute(notify_rowstride_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::width",
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_bits_per_sample_trampoline<P>(this: *mut ffi::GdkPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pixbuf> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Pixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_colorspace_trampoline<P>(this: *mut ffi::GdkPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pixbuf> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Pixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_has_alpha_trampoline<P>(this: *mut ffi::GdkPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pixbuf> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Pixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::GdkPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pixbuf> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Pixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_n_channels_trampoline<P>(this: *mut ffi::GdkPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pixbuf> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Pixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixel_bytes_trampoline<P>(this: *mut ffi::GdkPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pixbuf> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Pixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixels_trampoline<P>(this: *mut ffi::GdkPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pixbuf> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Pixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_rowstride_trampoline<P>(this: *mut ffi::GdkPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pixbuf> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Pixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::GdkPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pixbuf> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Pixbuf::from_glib_borrow(this).downcast_unchecked())
}
