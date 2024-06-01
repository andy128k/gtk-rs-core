// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Layout, LayoutLine, LayoutRun, Rectangle};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LayoutIter(Boxed<ffi::PangoLayoutIter>);

    match fn {
        copy => |ptr| ffi::pango_layout_iter_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_layout_iter_free(ptr),
        type_ => || ffi::pango_layout_iter_get_type(),
    }
}

impl LayoutIter {
    #[doc(alias = "pango_layout_iter_at_last_line")]
    pub fn at_last_line(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_iter_at_last_line(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "pango_layout_iter_get_baseline")]
    #[doc(alias = "get_baseline")]
    pub fn baseline(&mut self) -> i32 {
        unsafe { ffi::pango_layout_iter_get_baseline(self.to_glib_none_mut().0) }
    }

    #[doc(alias = "pango_layout_iter_get_char_extents")]
    #[doc(alias = "get_char_extents")]
    pub fn char_extents(&mut self) -> Rectangle {
        unsafe {
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_char_extents(
                self.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            logical_rect
        }
    }

    #[doc(alias = "pango_layout_iter_get_cluster_extents")]
    #[doc(alias = "get_cluster_extents")]
    pub fn cluster_extents(&mut self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_cluster_extents(
                self.to_glib_none_mut().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[doc(alias = "pango_layout_iter_get_index")]
    #[doc(alias = "get_index")]
    pub fn index(&mut self) -> i32 {
        unsafe { ffi::pango_layout_iter_get_index(self.to_glib_none_mut().0) }
    }

    #[doc(alias = "pango_layout_iter_get_layout")]
    #[doc(alias = "get_layout")]
    pub fn layout(&mut self) -> Option<Layout> {
        unsafe { from_glib_none(ffi::pango_layout_iter_get_layout(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "pango_layout_iter_get_layout_extents")]
    #[doc(alias = "get_layout_extents")]
    pub fn layout_extents(&mut self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_layout_extents(
                self.to_glib_none_mut().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[doc(alias = "pango_layout_iter_get_line")]
    #[doc(alias = "get_line")]
    pub fn line(&mut self) -> Option<LayoutLine> {
        unsafe { from_glib_none(ffi::pango_layout_iter_get_line(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "pango_layout_iter_get_line_extents")]
    #[doc(alias = "get_line_extents")]
    pub fn line_extents(&mut self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_line_extents(
                self.to_glib_none_mut().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[doc(alias = "pango_layout_iter_get_line_readonly")]
    #[doc(alias = "get_line_readonly")]
    pub fn line_readonly(&mut self) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(ffi::pango_layout_iter_get_line_readonly(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "pango_layout_iter_get_line_yrange")]
    #[doc(alias = "get_line_yrange")]
    pub fn line_yrange(&mut self) -> (i32, i32) {
        unsafe {
            let mut y0_ = std::mem::MaybeUninit::uninit();
            let mut y1_ = std::mem::MaybeUninit::uninit();
            ffi::pango_layout_iter_get_line_yrange(
                self.to_glib_none_mut().0,
                y0_.as_mut_ptr(),
                y1_.as_mut_ptr(),
            );
            (y0_.assume_init(), y1_.assume_init())
        }
    }

    #[doc(alias = "pango_layout_iter_get_run")]
    #[doc(alias = "get_run")]
    pub fn run(&mut self) -> Option<LayoutRun> {
        unsafe { from_glib_none(ffi::pango_layout_iter_get_run(self.to_glib_none_mut().0)) }
    }

    #[cfg(feature = "v1_50")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_layout_iter_get_run_baseline")]
    #[doc(alias = "get_run_baseline")]
    pub fn run_baseline(&mut self) -> i32 {
        unsafe { ffi::pango_layout_iter_get_run_baseline(self.to_glib_none_mut().0) }
    }

    #[doc(alias = "pango_layout_iter_get_run_extents")]
    #[doc(alias = "get_run_extents")]
    pub fn run_extents(&mut self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_run_extents(
                self.to_glib_none_mut().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[doc(alias = "pango_layout_iter_get_run_readonly")]
    #[doc(alias = "get_run_readonly")]
    pub fn run_readonly(&mut self) -> Option<LayoutRun> {
        unsafe {
            from_glib_none(ffi::pango_layout_iter_get_run_readonly(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "pango_layout_iter_next_char")]
    pub fn next_char(&mut self) -> bool {
        unsafe { from_glib(ffi::pango_layout_iter_next_char(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "pango_layout_iter_next_cluster")]
    pub fn next_cluster(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_iter_next_cluster(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[doc(alias = "pango_layout_iter_next_line")]
    pub fn next_line(&mut self) -> bool {
        unsafe { from_glib(ffi::pango_layout_iter_next_line(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "pango_layout_iter_next_run")]
    pub fn next_run(&mut self) -> bool {
        unsafe { from_glib(ffi::pango_layout_iter_next_run(self.to_glib_none_mut().0)) }
    }
}
