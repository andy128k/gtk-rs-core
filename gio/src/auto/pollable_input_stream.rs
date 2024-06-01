// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, InputStream};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GPollableInputStream")]
    pub struct PollableInputStream(Interface<ffi::GPollableInputStream, ffi::GPollableInputStreamInterface>) @requires InputStream;

    match fn {
        type_ => || ffi::g_pollable_input_stream_get_type(),
    }
}

impl PollableInputStream {
    pub const NONE: Option<&'static PollableInputStream> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::PollableInputStream>> Sealed for T {}
}

pub trait PollableInputStreamExt: IsA<PollableInputStream> + sealed::Sealed + 'static {
    #[doc(alias = "g_pollable_input_stream_can_poll")]
    fn can_poll(&self) -> bool {
        unsafe {
            from_glib(ffi::g_pollable_input_stream_can_poll(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_pollable_input_stream_is_readable")]
    fn is_readable(&self) -> bool {
        unsafe {
            from_glib(ffi::g_pollable_input_stream_is_readable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<PollableInputStream>> PollableInputStreamExt for O {}
