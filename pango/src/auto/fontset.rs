// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Font, FontMetrics};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "PangoFontset")]
    pub struct Fontset(Object<ffi::PangoFontset, ffi::PangoFontsetClass>);

    match fn {
        type_ => || ffi::pango_fontset_get_type(),
    }
}

impl Fontset {
    pub const NONE: Option<&'static Fontset> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Fontset>> Sealed for T {}
}

pub trait FontsetExt: IsA<Fontset> + sealed::Sealed + 'static {
    #[doc(alias = "pango_fontset_foreach")]
    fn foreach<P: FnMut(&Fontset, &Font) -> bool>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&Fontset, &Font) -> bool>(
            fontset: *mut ffi::PangoFontset,
            font: *mut ffi::PangoFont,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let fontset = from_glib_borrow(fontset);
            let font = from_glib_borrow(font);
            let callback = user_data as *mut P;
            (*callback)(&fontset, &font).into_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::pango_fontset_foreach(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as *mut _,
            );
        }
    }

    #[doc(alias = "pango_fontset_get_font")]
    #[doc(alias = "get_font")]
    fn font(&self, wc: u32) -> Font {
        unsafe {
            from_glib_full(ffi::pango_fontset_get_font(
                self.as_ref().to_glib_none().0,
                wc,
            ))
        }
    }

    #[doc(alias = "pango_fontset_get_metrics")]
    #[doc(alias = "get_metrics")]
    fn metrics(&self) -> FontMetrics {
        unsafe {
            from_glib_full(ffi::pango_fontset_get_metrics(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<Fontset>> FontsetExt for O {}
