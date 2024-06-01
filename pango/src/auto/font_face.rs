// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v1_46")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_46")))]
use crate::FontFamily;
use crate::{ffi, FontDescription};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "PangoFontFace")]
    pub struct FontFace(Object<ffi::PangoFontFace, ffi::PangoFontFaceClass>);

    match fn {
        type_ => || ffi::pango_font_face_get_type(),
    }
}

impl FontFace {
    pub const NONE: Option<&'static FontFace> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::FontFace>> Sealed for T {}
}

pub trait FontFaceExt: IsA<FontFace> + sealed::Sealed + 'static {
    #[doc(alias = "pango_font_face_describe")]
    fn describe(&self) -> FontDescription {
        unsafe {
            from_glib_full(ffi::pango_font_face_describe(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_face_get_face_name")]
    #[doc(alias = "get_face_name")]
    fn face_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::pango_font_face_get_face_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_46")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_font_face_get_family")]
    #[doc(alias = "get_family")]
    fn family(&self) -> FontFamily {
        unsafe {
            from_glib_none(ffi::pango_font_face_get_family(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_face_is_synthesized")]
    fn is_synthesized(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_font_face_is_synthesized(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_face_list_sizes")]
    fn list_sizes(&self) -> Vec<i32> {
        unsafe {
            let mut sizes = std::ptr::null_mut();
            let mut n_sizes = std::mem::MaybeUninit::uninit();
            ffi::pango_font_face_list_sizes(
                self.as_ref().to_glib_none().0,
                &mut sizes,
                n_sizes.as_mut_ptr(),
            );
            FromGlibContainer::from_glib_full_num(sizes, n_sizes.assume_init() as _)
        }
    }
}

impl<O: IsA<FontFace>> FontFaceExt for O {}
