extern mod core_foundation;
extern mod core_graphics;

use font_descriptor::{CTFontDescriptor, CTFontDescriptorRef, CTFontOrientation};
use font_descriptor::{CTFontSymbolicTraits, CTFontTraits, SymbolicTraitAccessors, TraitAccessors};

use core::libc::c_uint;
use core_foundation::array::{CFArrayRef};
use core_foundation::base::{AbstractCFTypeRef, CFIndex, CFOptionFlags, CFTypeID, CFTypeRef};
use core_foundation::base::{CFWrapper};
use core_foundation::data::{CFData, CFDataRef};
use core_foundation::dictionary::{CFDictionaryRef, UntypedCFDictionary};
use core_foundation::string::{CFStringGetTypeID, CFString, CFStringRef, UniChar};
use core_graphics::base::{CGAffineTransform, CGFloat};
use core_graphics::font::{CGGlyph, CGFont, CGFontRef};
use core_graphics::geometry::{CGRect, CGSize};

pub type CTFontUIFontType = u32;
// kCTFontNoFontType: CTFontUIFontType = -1;
pub const kCTFontUserFontType: CTFontUIFontType = 0;
pub const kCTFontUserFixedPitchFontType: CTFontUIFontType = 1;
pub const kCTFontSystemFontType: CTFontUIFontType = 2;
pub const kCTFontEmphasizedSystemFontType: CTFontUIFontType = 3;
pub const kCTFontSmallSystemFontType: CTFontUIFontType = 4;
pub const kCTFontSmallEmphasizedSystemFontType: CTFontUIFontType = 5;
pub const kCTFontMiniSystemFontType: CTFontUIFontType = 6;
pub const kCTFontMiniEmphasizedSystemFontType: CTFontUIFontType = 7;
pub const kCTFontViewsFontType: CTFontUIFontType = 8;
pub const kCTFontApplicationFontType: CTFontUIFontType = 9;
pub const kCTFontLabelFontType: CTFontUIFontType = 10;
pub const kCTFontMenuTitleFontType: CTFontUIFontType = 11;
pub const kCTFontMenuItemFontType: CTFontUIFontType = 12;
pub const kCTFontMenuItemMarkFontType: CTFontUIFontType = 13;
pub const kCTFontMenuItemCmdKeyFontType: CTFontUIFontType = 14;
pub const kCTFontWindowTitleFontType: CTFontUIFontType = 15;
pub const kCTFontPushButtonFontType: CTFontUIFontType = 16;
pub const kCTFontUtilityWindowTitleFontType: CTFontUIFontType = 17;
pub const kCTFontAlertHeaderFontType: CTFontUIFontType = 18;
pub const kCTFontSystemDetailFontType: CTFontUIFontType = 19;
pub const kCTFontEmphasizedSystemDetailFontType: CTFontUIFontType = 20;
pub const kCTFontToolbarFontType: CTFontUIFontType = 21;
pub const kCTFontSmallToolbarFontType: CTFontUIFontType = 22;
pub const kCTFontMessageFontType: CTFontUIFontType = 23;
pub const kCTFontPaletteFontType: CTFontUIFontType = 24;
pub const kCTFontToolTipFontType: CTFontUIFontType = 25;
pub const kCTFontControlContentFontType: CTFontUIFontType = 26;

pub type CTFontTableTag = u32;
// TODO: create bindings for enum with 'chars' values

pub type CTFontTableOptions = u32;
pub const kCTFontTableOptionsNoOptions: CTFontTableOptions = 0;
pub const kCTFontTableOptionsExcludeSynthetic: CTFontTableOptions = (1 << 0);

pub type CTFontOptions = CFOptionFlags;
pub const kCTFontOptionsDefault: CTFontOptions = 0;
pub const kCTFontOptionsPreventAutoActivation: CTFontOptions = (1 << 0);
pub const kCTFontOptionsPreferSystemFont: CTFontOptions = (1 << 2);

struct __CTFont { private: () }
pub type CTFontRef = *__CTFont;

impl AbstractCFTypeRef for CTFontRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    static pure fn type_id() -> CFTypeID {
        unsafe {
            CTFontGetTypeID()
        }
    }
}

pub type CTFont = CFWrapper<CTFontRef, (), ()>;

pub trait CTFontMethods {
    // Creation methods (statics below)
    fn copy_to_CGFont(&const self) -> CGFont;
    fn clone_with_font_size(&const self, size: float) -> CTFont;

    // Names
    pure fn family_name(&self) -> ~str;
    pure fn face_name(&self) -> ~str;
    pure fn unique_name(&self) -> ~str;
    pure fn postscript_name(&self) -> ~str;

    // Properties
    pure fn all_traits(&self) -> CTFontTraits;

    // Font metrics
    pure fn ascent(&self) -> CGFloat;
    pure fn descent(&self) -> CGFloat;
    pure fn underline_thickness(&self) -> CGFloat;
    pure fn underline_position(&self) -> CGFloat;
    pure fn bounding_box(&self) -> CGRect;
    pure fn leading(&self) -> CGFloat;
    pure fn x_height(&self) -> CGFloat;
    pure fn pt_size(&self) -> CGFloat;
    fn get_glyphs_for_characters(&self,
                                 characters: *UniChar,
                                 glyphs: *CGGlyph,
                                 count: CFIndex)
                              -> bool;
    fn get_advances_for_glyphs(&self,
                               orientation: CTFontOrientation,
                               glyphs: *CGGlyph,
                               advances: *CGSize,
                               count: CFIndex)
                            -> float;
    fn get_font_table(&self, tag: u32) -> Option<CFData>;
}

pub fn new_from_CGFont(cgfont: &CGFont, pt_size: float) -> CTFont {
    unsafe {
        let result = CTFontCreateWithGraphicsFont(*cgfont.borrow_ref(),
                                                  pt_size as CGFloat,
                                                  ptr::null(),
                                                  ptr::null());
        CFWrapper::wrap_owned(result)
    }
}

pub fn new_from_descriptor(desc: &CTFontDescriptor, pt_size: float) -> CTFont {
    unsafe {
        let result = CTFontCreateWithFontDescriptor(*desc.borrow_ref(),
                                                    pt_size as CGFloat,
                                                    ptr::null());
        CFWrapper::wrap_owned(result)
    }
}

pub fn new_from_name(name: ~str, pt_size: float) -> Result<CTFont, ()> {
    unsafe {
        let cfname = CFString::new(name);
        let result = CTFontCreateWithName(*cfname.borrow_ref(), pt_size as CGFloat, ptr::null());
        if result.is_null() { return Err(()); }

        return Ok(CFWrapper::wrap_owned(result));
    }
}

priv trait CTFontMethodsPrivate {
    pure fn symbolic_traits() -> CTFontSymbolicTraits;
}

pub impl CTFontMethodsPrivate for CTFont {
    // Properties
    priv pure fn symbolic_traits() -> CTFontSymbolicTraits {
        unsafe {
            CTFontGetSymbolicTraits(self.obj)
        }
    }
}

pub impl CTFontMethods for CTFont {
    // Creation methods
    fn copy_to_CGFont(&const self) -> CGFont {
        unsafe {
            let value = CTFontCopyGraphicsFont(self.obj, ptr::null());
            CFWrapper::wrap_owned(value)
        }
    }

    fn clone_with_font_size(&const self, size: float) -> CTFont {
        unsafe {
            let result = CTFontCreateCopyWithAttributes(self.obj,
                                                        size as CGFloat,
                                                        ptr::null(),
                                                        ptr::null());
            CFWrapper::wrap_owned(result)
        }
    }

    // Names
    pure fn family_name(&self) -> ~str {
        unsafe {
            let value = get_string_by_name_key(self, kCTFontFamilyNameKey);
            return option::expect(value, ~"Fonts should always have a family name.");
        }
    }

    pure fn face_name(&self) -> ~str {
        unsafe {
            let value = get_string_by_name_key(self, kCTFontSubFamilyNameKey);
            return option::expect(value, ~"Fonts should always have a face name.");
        }
    }

    pure fn unique_name(&self) -> ~str {
        unsafe {
            let value = get_string_by_name_key(self, kCTFontUniqueNameKey);
            return option::expect(value, ~"Fonts should always have a unique name.");
        }
    }

    pure fn postscript_name(&self) -> ~str {
        unsafe {
            let value = get_string_by_name_key(self, kCTFontPostScriptNameKey);
            return option::expect(value, ~"Fonts should always have a PostScript name.");
        }
    }

    pure fn all_traits(&self) -> CTFontTraits {
        unsafe {
            let result = CTFontCopyTraits(self.obj);
            CFWrapper::wrap_owned(result)
        }
    }

    // Font metrics
    pure fn ascent(&self) -> CGFloat {
        unsafe {
            CTFontGetAscent(self.obj)
        }
    }

    pure fn descent(&self) -> CGFloat {
        unsafe {
            CTFontGetDescent(self.obj)
        }
    }

    pure fn underline_thickness(&self) -> CGFloat {
        unsafe {
            CTFontGetUnderlineThickness(self.obj)
        }
    }

    pure fn underline_position(&self) -> CGFloat {
        unsafe {
            CTFontGetUnderlinePosition(self.obj)
        }
    }

    pure fn bounding_box(&self) -> CGRect {
        unsafe {
            CTFontGetBoundingBox(self.obj)
        }
    }

    pure fn leading(&self) -> CGFloat {
        unsafe {
            CTFontGetLeading(self.obj)
        }
    }

    pure fn x_height(&self) -> CGFloat {
        unsafe {
            CTFontGetXHeight(self.obj)
        }
    }

    pure fn pt_size(&self) -> CGFloat {
        unsafe {
            CTFontGetSize(self.obj)
        }
    }

    fn get_glyphs_for_characters(&self,
                                 characters: *UniChar,
                                 glyphs: *CGGlyph,
                                 count: CFIndex)
                              -> bool {
        unsafe {
            CTFontGetGlyphsForCharacters(self.obj, characters, glyphs, count)
        }
    }

    fn get_advances_for_glyphs(&self,
                               orientation: CTFontOrientation,
                               glyphs: *CGGlyph,
                               advances: *CGSize,
                               count: CFIndex)
                            -> float {
        unsafe {
            CTFontGetAdvancesForGlyphs(self.obj, orientation, glyphs, advances, count) as float
        }
    }

    fn get_font_table(&self, tag: u32) -> Option<CFData> {
        unsafe {
            let result = CTFontCopyTable(self.obj,
                                         tag as CTFontTableTag,
                                         kCTFontTableOptionsExcludeSynthetic);
            return match result.is_null() {
                true => None,
                false => Some(CFWrapper::wrap_owned(result)),
            }
        }
    }
}

// Helper methods
priv fn get_string_by_name_key(font: &CTFont, name_key: CFStringRef) -> Option<~str> {
    unsafe {
        let result = CTFontCopyName(*font.borrow_ref(), name_key);
        if result.is_null() { return None; }

        return Some(CFWrapper::wrap_owned(result).to_str());
    }
}

pub fn debug_font_names(font: &CTFont) {
    fn get_key(font: &CTFont, key: CFStringRef) -> ~str {
        option::unwrap(get_string_by_name_key(font, key))
    }

    io::println(fmt!("kCTFontFamilyNameKey: %s", get_key(font, kCTFontFamilyNameKey)));
    io::println(fmt!("kCTFontSubFamilyNameKey: %s", get_key(font, kCTFontSubFamilyNameKey)));
    io::println(fmt!("kCTFontStyleNameKey: %s", get_key(font, kCTFontStyleNameKey)));
    io::println(fmt!("kCTFontUniqueNameKey: %s", get_key(font, kCTFontUniqueNameKey)));
    io::println(fmt!("kCTFontFullNameKey: %s", get_key(font, kCTFontFullNameKey)));
    io::println(fmt!("kCTFontPostScriptNameKey: %s", get_key(font, kCTFontPostScriptNameKey)));
}

pub fn debug_font_traits(font: &CTFont) {
    let sym = font.symbolic_traits();
    io::println(fmt!("kCTFontItalicTrait: %b", sym.is_italic()));
    io::println(fmt!("kCTFontBoldTrait: %b", sym.is_bold()));
    io::println(fmt!("kCTFontExpandedTrait: %b", sym.is_expanded()));
    io::println(fmt!("kCTFontCondensedTrait: %b", sym.is_condensed()));
    io::println(fmt!("kCTFontMonoSpaceTrait: %b", sym.is_monospace()));

    let traits = font.all_traits();
    io::println(fmt!("kCTFontWeightTrait: %f", traits.normalized_weight()));
//    io::println(fmt!("kCTFontWidthTrait: %f", traits.normalized_width()));
//    io::println(fmt!("kCTFontSlantTrait: %f", traits.normalized_slant()));
}

#[nolink]
#[link_args = "-framework ApplicationServices"]
extern {
    /*
     * CTFont.h
     */

    /* Name Specifier Constants */
    const kCTFontCopyrightNameKey: CFStringRef;
    const kCTFontFamilyNameKey: CFStringRef;
    const kCTFontSubFamilyNameKey: CFStringRef;
    const kCTFontStyleNameKey: CFStringRef;
    const kCTFontUniqueNameKey: CFStringRef;
    const kCTFontFullNameKey: CFStringRef;
    const kCTFontVersionNameKey: CFStringRef;
    const kCTFontPostScriptNameKey: CFStringRef;
    const kCTFontTrademarkNameKey: CFStringRef;
    const kCTFontManufacturerNameKey: CFStringRef;
    const kCTFontDesignerNameKey: CFStringRef;
    const kCTFontDescriptionNameKey: CFStringRef;
    const kCTFontVendorURLNameKey: CFStringRef;
    const kCTFontDesignerURLNameKey: CFStringRef;
    const kCTFontLicenseNameKey: CFStringRef;
    const kCTFontLicenseURLNameKey: CFStringRef;
    const kCTFontSampleTextNameKey: CFStringRef;
    const kCTFontPostScriptCIDNameKey: CFStringRef;

    const kCTFontVariationAxisIdentifierKey: CFStringRef;
    const kCTFontVariationAxisMinimumValueKey: CFStringRef;
    const kCTFontVariationAxisMaximumValueKey: CFStringRef;
    const kCTFontVariationAxisDefaultValueKey: CFStringRef;
    const kCTFontVariationAxisNameKey: CFStringRef;

    const kCTFontFeatureTypeIdentifierKey: CFStringRef;
    const kCTFontFeatureTypeNameKey: CFStringRef;
    const kCTFontFeatureTypeExclusiveKey: CFStringRef;
    const kCTFontFeatureTypeSelectorsKey: CFStringRef;
    const kCTFontFeatureSelectorIdentifierKey: CFStringRef;
    const kCTFontFeatureSelectorNameKey: CFStringRef;
    const kCTFontFeatureSelectorDefaultKey: CFStringRef;
    const kCTFontFeatureSelectorSettingKey: CFStringRef;

    // N.B. Unlike most Cocoa bindings, this extern block is organized according
    // to the documentation's Functions By Task listing, because there so many functions.

    /* Creating Fonts */
    fn CTFontCreateWithName(name: CFStringRef, size: CGFloat, matrix: *CGAffineTransform) -> CTFontRef;
    //fn CTFontCreateWithNameAndOptions
    fn CTFontCreateWithFontDescriptor(descriptor: CTFontDescriptorRef, size: CGFloat,
                                      matrix: *CGAffineTransform) -> CTFontRef;
    //fn CTFontCreateWithFontDescriptorAndOptions
    //fn CTFontCreateUIFontForLanguage
    fn CTFontCreateCopyWithAttributes(font: CTFontRef, size: CGFloat, matrix: *CGAffineTransform, 
                                      attributes: CTFontDescriptorRef) -> CTFontRef;
    //fn CTFontCreateCopyWithSymbolicTraits
    //fn CTFontCreateCopyWithFamily
    //fn CTFontCreateForString

    /* Getting Font Data */
    fn CTFontCopyFontDescriptor(font: CTFontRef) -> CTFontDescriptorRef;
    fn CTFontCopyAttribute(font: CTFontRef) -> CFTypeRef;
    fn CTFontGetSize(font: CTFontRef) -> CGFloat;
    //fn CTFontGetMatrix
    fn CTFontGetSymbolicTraits(font: CTFontRef) -> CTFontSymbolicTraits;
    fn CTFontCopyTraits(font: CTFontRef) -> CFDictionaryRef;

    /* Getting Font Names */
    fn CTFontCopyPostScriptName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyFamilyName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyFullName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyDisplayName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyName(font: CTFontRef, nameKey: CFStringRef) -> CFStringRef;
    fn CTFontCopyLocalizedName(font: CTFontRef, nameKey: CFStringRef, 
                               language: *CFStringRef) -> CFStringRef;

    /* Working With Encoding */
    //fn CTFontCopyCharacterSet
    //fn CTFontGetStringEncoding
    //fn CTFontCopySupportedLanguages

    /* Getting Font Metrics */
    fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    fn CTFontGetLeading(font: CTFontRef) -> CGFloat;
    fn CTFontGetUnitsPerEm(font: CTFontRef) -> libc::c_uint;
    //fn CTFontGetGlyphCount
    fn CTFontGetBoundingBox(font: CTFontRef) -> CGRect;
    fn CTFontGetUnderlinePosition(font: CTFontRef) -> CGFloat;
    fn CTFontGetUnderlineThickness(font: CTFontRef) -> CGFloat;
    //fn CTFontGetSlantAngle
    //fn CTFontGetCapHeight
    fn CTFontGetXHeight(font: CTFontRef) -> CGFloat;

    /* Getting Glyph Data */
    //fn CTFontCreatePathForGlyph
    //fn CTFontGetGlyphWithName
    //fn CTFontGetBoundingRectsForGlyphs
    fn CTFontGetAdvancesForGlyphs(font: CTFontRef, orientation: CTFontOrientation, glyphs: *CGGlyph, advances: *CGSize, count: CFIndex) -> libc::c_double;
    //fn CTFontGetVerticalTranslationsForGlyphs

    /* Working With Font Variations */
    //fn CTFontCopyVariationAxes
    //fn CTFontCopyVariation

    /* Getting Font Features */
    //fn CTFontCopyFeatures
    //fn CTFontCopyFeatureSettings

    /* Working with Glyphs */
    fn CTFontGetGlyphsForCharacters(font: CTFontRef, characters: *UniChar, glyphs: *CGGlyph, count: CFIndex) -> bool;
    //fn CTFontDrawGlyphs
    //fn CTFontGetLigatureCaretPositions

    /* Converting Fonts */
    fn CTFontCopyGraphicsFont(font: CTFontRef, attributes: *CTFontDescriptorRef) -> CGFontRef;
    fn CTFontCreateWithGraphicsFont(graphicsFont: CGFontRef, size: CGFloat, 
                                    matrix: *CGAffineTransform, 
                                    attributes: CTFontDescriptorRef) -> CTFontRef;
    //fn CTFontGetPlatformFont
    //fn CTFontCreateWithPlatformFont
    //fn CTFontCreateWithQuickdrawInstance

    /* Getting Font Table Data */
    fn CTFontCopyAvailableTables(font: CTFontRef, options: CTFontTableOptions) -> CFArrayRef;
    fn CTFontCopyTable(font: CTFontRef, table: CTFontTableTag, options: CTFontTableOptions) -> CFDataRef;

    fn CTFontGetTypeID() -> CFTypeID;
    
}
