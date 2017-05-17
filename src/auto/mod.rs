// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

mod context;
pub use self::context::Context;
pub use self::context::ContextExt;

mod font_face;
pub use self::font_face::FontFace;
pub use self::font_face::FontFaceExt;

mod font_family;
pub use self::font_family::FontFamily;
pub use self::font_family::FontFamilyExt;

mod font_map;
pub use self::font_map::FontMap;
pub use self::font_map::FontMapExt;

mod layout;
pub use self::layout::Layout;
pub use self::layout::LayoutExt;

mod layout_iter;
pub use self::layout_iter::LayoutIter;

mod layout_line;
pub use self::layout_line::LayoutLine;

mod enums;
pub use self::enums::Alignment;
pub use self::enums::BidiType;
pub use self::enums::Direction;
pub use self::enums::EllipsizeMode;
pub use self::enums::Gravity;
pub use self::enums::GravityHint;
pub use self::enums::Script;
pub use self::enums::Stretch;
pub use self::enums::Style;
pub use self::enums::Underline;
pub use self::enums::Variant;
pub use self::enums::Weight;
pub use self::enums::WrapMode;

mod flags;
pub use self::flags::FontMask;
pub use self::flags::FONT_MASK_FAMILY;
pub use self::flags::FONT_MASK_STYLE;
pub use self::flags::FONT_MASK_VARIANT;
pub use self::flags::FONT_MASK_WEIGHT;
pub use self::flags::FONT_MASK_STRETCH;
pub use self::flags::FONT_MASK_SIZE;
pub use self::flags::FONT_MASK_GRAVITY;

pub mod functions;

#[doc(hidden)]
pub mod traits {
    pub use super::ContextExt;
    pub use super::FontFaceExt;
    pub use super::FontFamilyExt;
    pub use super::FontMapExt;
    pub use super::LayoutExt;
}
