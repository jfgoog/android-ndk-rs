//! Bindings for `AConfiguration`.
//!
//! See also the [NDK docs](https://developer.android.com/ndk/reference/group/configuration) for
//! `AConfiguration`, as well as the [docs for providing
//! resources](https://developer.android.com/guide/topics/resources/providing-resources.html),
//! which explain many of the configuration values.

use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryInto;
use std::ptr::NonNull;

/// A native `AConfiguration *`.
///
/// This stores information about configuration.
pub struct Configuration {
    ptr: NonNull<ffi::AConfiguration>,
}

unsafe impl Send for Configuration {}
unsafe impl Sync for Configuration {}

impl Drop for Configuration {
    fn drop(&mut self) {
        unsafe { ffi::AConfiguration_delete(self.ptr.as_ptr()) }
    }
}

impl Clone for Configuration {
    fn clone(&self) -> Self {
        let mut new = Self::new();
        new.copy(self);
        new
    }
}

impl PartialEq for Configuration {
    fn eq(&self, other: &Self) -> bool {
        self.diff(other).0 == 0
    }
}
impl Eq for Configuration {}

impl Configuration {
    /// Construct a `Configuration` from a pointer.
    ///
    /// By calling this function, you assert that it is a valid pointer to a native
    /// `AConfiguration`, and give ownership of it to the `Configuration` instance.
    pub unsafe fn from_ptr(ptr: NonNull<ffi::AConfiguration>) -> Self {
        Self { ptr }
    }

    /// Create a new `Configuration`, with the same contents as the `AConfiguration` referenced by
    /// the pointer.
    ///
    /// This is useful if you have a pointer, but not ownership of it.
    pub unsafe fn clone_from_ptr(ptr: NonNull<ffi::AConfiguration>) -> Self {
        let conf = Self::new();
        ffi::AConfiguration_copy(conf.ptr.as_ptr(), ptr.as_ptr());
        conf
    }

    /// The pointer to the native `AConfiguration`.  Keep in mind that the `Configuration` object
    /// still has ownership, and will free it when dropped.
    pub fn ptr(&self) -> NonNull<ffi::AConfiguration> {
        self.ptr
    }

    /// Create a new `Configuration`, with none of the values set.
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: NonNull::new(ffi::AConfiguration_new()).unwrap(),
            }
        }
    }

    /// `dest.copy(&src)` copies the contents of `src` to `dest`
    pub fn copy(&mut self, other: &Self) {
        unsafe { ffi::AConfiguration_copy(self.ptr.as_ptr(), other.ptr.as_ptr()) }
    }

    /// Information about what fields differ between the two configurations
    pub fn diff(&self, other: &Self) -> DiffResult {
        unsafe {
            DiffResult(ffi::AConfiguration_diff(self.ptr.as_ptr(), other.ptr.as_ptr()) as u32)
        }
    }

    /// Returns false if anything in `self` conflicts with `requested`
    pub fn matches(&self, requested: &Self) -> bool {
        unsafe { ffi::AConfiguration_match(self.ptr.as_ptr(), requested.ptr.as_ptr()) != 0 }
    }

    /// Returns the country code. It will always be two letters.
    pub fn country(&self) -> String {
        let mut result = "  ".to_owned();
        unsafe {
            ffi::AConfiguration_getCountry(self.ptr.as_ptr(), result.as_mut_ptr() as *mut _);
        }
        result
    }

    /// Returns the screen density class.
    pub fn density(&self) -> Density {
        unsafe {
            (ffi::AConfiguration_getDensity(self.ptr.as_ptr()) as u32)
                .try_into()
                .unwrap()
        }
    }

    /// Returns the keyboard type.
    pub fn keyboard(&self) -> Keyboard {
        unsafe {
            (ffi::AConfiguration_getKeyboard(self.ptr.as_ptr()) as u32)
                .try_into()
                .unwrap()
        }
    }

    /// Returns the language, as a `String` of two characters, if a language is set
    pub fn language(&self) -> Option<String> {
        let mut chars = [0u8; 2];
        unsafe {
            ffi::AConfiguration_getLanguage(self.ptr.as_ptr(), chars[..].as_mut_ptr() as *mut _);
        }
        if chars[0] == 0 {
            None
        } else {
            Some(std::str::from_utf8(&chars[..]).unwrap().to_owned())
        }
    }

    /// Returns the layout direction
    pub fn layout_direction(&self) -> LayoutDir {
        unsafe {
            (ffi::AConfiguration_getLayoutDirection(self.ptr.as_ptr()) as u32)
                .try_into()
                .unwrap()
        }
    }

    /// Returns the mobile country code.
    pub fn mcc(&self) -> i32 {
        unsafe { ffi::AConfiguration_getMcc(self.ptr.as_ptr()) }
    }

    /// Returns the mobile network code, if one is defined
    pub fn mnc(&self) -> Option<i32> {
        unsafe {
            match ffi::AConfiguration_getMnc(self.ptr.as_ptr()) {
                0 => None,
                x if x == ffi::ACONFIGURATION_MNC_ZERO as i32 => Some(0),
                x => Some(x),
            }
        }
    }

    pub fn nav_hidden(&self) -> NavHidden {
        unsafe {
            (ffi::AConfiguration_getNavHidden(self.ptr.as_ptr()) as u32)
                .try_into()
                .unwrap()
        }
    }

    pub fn navigation(&self) -> Navigation {
        unsafe {
            (ffi::AConfiguration_getNavigation(self.ptr.as_ptr()) as u32)
                .try_into()
                .unwrap()
        }
    }

    pub fn orientation(&self) -> Orientation {
        unsafe {
            (ffi::AConfiguration_getOrientation(self.ptr.as_ptr()) as u32)
                .try_into()
                .unwrap()
        }
    }

    pub fn screen_height_dp(&self) -> Option<i32> {
        unsafe {
            let height = ffi::AConfiguration_getScreenHeightDp(self.ptr.as_ptr());
            if height == ffi::ACONFIGURATION_SCREEN_HEIGHT_DP_ANY as i32 {
                None
            } else {
                Some(height)
            }
        }
    }

    pub fn screen_width_dp(&self) -> Option<i32> {
        unsafe {
            let width = ffi::AConfiguration_getScreenWidthDp(self.ptr.as_ptr());
            if width == ffi::ACONFIGURATION_SCREEN_WIDTH_DP_ANY as i32 {
                None
            } else {
                Some(width)
            }
        }
    }

    pub fn screen_long(&self) -> ScreenLong {
        unsafe {
            (ffi::AConfiguration_getScreenLong(self.ptr.as_ptr()) as u32)
                .try_into()
                .unwrap()
        }
    }

    pub fn screen_round(&self) -> ScreenRound {
        unsafe {
            (ffi::AConfiguration_getScreenRound(self.ptr.as_ptr()) as u32)
                .try_into()
                .unwrap()
        }
    }

    pub fn screen_size(&self) -> ScreenSize {
        unsafe {
            (ffi::AConfiguration_getScreenSize(self.ptr.as_ptr()) as u32)
                .try_into()
                .unwrap()
        }
    }

    pub fn sdk_version(&self) -> i32 {
        unsafe { ffi::AConfiguration_getSdkVersion(self.ptr.as_ptr()) }
    }

    pub fn smallest_screen_width_dp(&self) -> Option<i32> {
        unsafe {
            let width = ffi::AConfiguration_getSmallestScreenWidthDp(self.ptr.as_ptr());
            if width == ffi::ACONFIGURATION_SMALLEST_SCREEN_WIDTH_DP_ANY as i32 {
                None
            } else {
                Some(width)
            }
        }
    }

    pub fn touchscreen(&self) -> Touchscreen {
        unsafe {
            (ffi::AConfiguration_getTouchscreen(self.ptr.as_ptr()) as u32)
                .try_into()
                .unwrap()
        }
    }

    pub fn ui_mode_night(&self) -> UiModeNight {
        unsafe {
            (ffi::AConfiguration_getUiModeNight(self.ptr.as_ptr()) as u32)
                .try_into()
                .unwrap()
        }
    }

    pub fn ui_mode_type(&self) -> UiModeType {
        unsafe {
            (ffi::AConfiguration_getUiModeType(self.ptr.as_ptr()) as u32)
                .try_into()
                .unwrap()
        }
    }
}

/// A bitfield representing the differences between two `Configuration`s
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DiffResult(pub u32);

impl DiffResult {
    pub fn mcc(self) -> bool {
        self.0 & ffi::ACONFIGURATION_MCC != 0
    }
    pub fn mnc(self) -> bool {
        self.0 & ffi::ACONFIGURATION_MNC != 0
    }
    pub fn locale(self) -> bool {
        self.0 & ffi::ACONFIGURATION_LOCALE != 0
    }
    pub fn touchscreen(self) -> bool {
        self.0 & ffi::ACONFIGURATION_TOUCHSCREEN != 0
    }
    pub fn keyboard(self) -> bool {
        self.0 & ffi::ACONFIGURATION_KEYBOARD != 0
    }
    pub fn keyboard_hidden(self) -> bool {
        self.0 & ffi::ACONFIGURATION_KEYBOARD_HIDDEN != 0
    }
    pub fn navigation(self) -> bool {
        self.0 & ffi::ACONFIGURATION_NAVIGATION != 0
    }
    pub fn orientation(self) -> bool {
        self.0 & ffi::ACONFIGURATION_ORIENTATION != 0
    }
    pub fn density(self) -> bool {
        self.0 & ffi::ACONFIGURATION_DENSITY != 0
    }
    pub fn screen_size(self) -> bool {
        self.0 & ffi::ACONFIGURATION_SCREEN_SIZE != 0
    }
    pub fn version(self) -> bool {
        self.0 & ffi::ACONFIGURATION_VERSION != 0
    }
    pub fn screen_layout(self) -> bool {
        self.0 & ffi::ACONFIGURATION_SCREEN_LAYOUT != 0
    }
    pub fn ui_mode(self) -> bool {
        self.0 & ffi::ACONFIGURATION_UI_MODE != 0
    }
    pub fn smallest_screen_size(self) -> bool {
        self.0 & ffi::ACONFIGURATION_SMALLEST_SCREEN_SIZE != 0
    }
    pub fn layout_dir(self) -> bool {
        self.0 & ffi::ACONFIGURATION_LAYOUTDIR != 0
    }
    pub fn screen_round(self) -> bool {
        self.0 & ffi::ACONFIGURATION_SCREEN_ROUND != 0
    }
    pub fn color_mode(self) -> bool {
        self.0 & ffi::ACONFIGURATION_COLOR_MODE != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Orientation {
    Any = ffi::ACONFIGURATION_ORIENTATION_ANY,
    Port = ffi::ACONFIGURATION_ORIENTATION_PORT,
    Land = ffi::ACONFIGURATION_ORIENTATION_LAND,
    Square = ffi::ACONFIGURATION_ORIENTATION_SQUARE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Touchscreen {
    Any = ffi::ACONFIGURATION_TOUCHSCREEN_ANY,
    NoTouch = ffi::ACONFIGURATION_TOUCHSCREEN_NOTOUCH,
    Stylus = ffi::ACONFIGURATION_TOUCHSCREEN_STYLUS,
    Finger = ffi::ACONFIGURATION_TOUCHSCREEN_FINGER,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Density {
    Default = ffi::ACONFIGURATION_DENSITY_DEFAULT,
    Low = ffi::ACONFIGURATION_DENSITY_LOW,
    Medium = ffi::ACONFIGURATION_DENSITY_MEDIUM,
    TV = ffi::ACONFIGURATION_DENSITY_TV,
    High = ffi::ACONFIGURATION_DENSITY_HIGH,
    XHigh = ffi::ACONFIGURATION_DENSITY_XHIGH,
    XXHigh = ffi::ACONFIGURATION_DENSITY_XXHIGH,
    XXXHigh = ffi::ACONFIGURATION_DENSITY_XXXHIGH,
    Any = ffi::ACONFIGURATION_DENSITY_ANY,
    None = ffi::ACONFIGURATION_DENSITY_NONE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Keyboard {
    Any = ffi::ACONFIGURATION_KEYBOARD_ANY,
    NoKeys = ffi::ACONFIGURATION_KEYBOARD_NOKEYS,
    Qwerty = ffi::ACONFIGURATION_KEYBOARD_QWERTY,
    TwelveKey = ffi::ACONFIGURATION_KEYBOARD_12KEY,
}

// FIXME is it a bitmask?
// FIXME are they all bitmasks?
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Navigation {
    Any = ffi::ACONFIGURATION_NAVIGATION_ANY,
    NoNav = ffi::ACONFIGURATION_NAVIGATION_NONAV,
    DPad = ffi::ACONFIGURATION_NAVIGATION_DPAD,
    Trackball = ffi::ACONFIGURATION_NAVIGATION_TRACKBALL,
    Wheel = ffi::ACONFIGURATION_NAVIGATION_WHEEL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum KeysHidden {
    Any = ffi::ACONFIGURATION_KEYSHIDDEN_ANY,
    No = ffi::ACONFIGURATION_KEYSHIDDEN_NO,
    Yes = ffi::ACONFIGURATION_KEYSHIDDEN_YES,
    Soft = ffi::ACONFIGURATION_KEYSHIDDEN_SOFT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum NavHidden {
    Any = ffi::ACONFIGURATION_NAVHIDDEN_ANY,
    No = ffi::ACONFIGURATION_NAVHIDDEN_NO,
    Yes = ffi::ACONFIGURATION_NAVHIDDEN_YES,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum ScreenSize {
    Any = ffi::ACONFIGURATION_SCREENSIZE_ANY,
    Small = ffi::ACONFIGURATION_SCREENSIZE_SMALL,
    Normal = ffi::ACONFIGURATION_SCREENSIZE_NORMAL,
    Large = ffi::ACONFIGURATION_SCREENSIZE_LARGE,
    XLarge = ffi::ACONFIGURATION_SCREENSIZE_XLARGE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum ScreenLong {
    Any = ffi::ACONFIGURATION_SCREENLONG_ANY,
    No = ffi::ACONFIGURATION_SCREENLONG_NO,
    Yes = ffi::ACONFIGURATION_SCREENLONG_YES,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum ScreenRound {
    Any = ffi::ACONFIGURATION_SCREENROUND_ANY,
    No = ffi::ACONFIGURATION_SCREENROUND_NO,
    Yes = ffi::ACONFIGURATION_SCREENROUND_YES,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum WideColorGamut {
    Any = ffi::ACONFIGURATION_WIDE_COLOR_GAMUT_ANY,
    No = ffi::ACONFIGURATION_WIDE_COLOR_GAMUT_NO,
    Yes = ffi::ACONFIGURATION_WIDE_COLOR_GAMUT_YES,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum HDR {
    Any = ffi::ACONFIGURATION_HDR_ANY,
    No = ffi::ACONFIGURATION_HDR_NO,
    Yes = ffi::ACONFIGURATION_HDR_YES,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum LayoutDir {
    Any = ffi::ACONFIGURATION_LAYOUTDIR_ANY,
    Ltr = ffi::ACONFIGURATION_LAYOUTDIR_LTR,
    Rtl = ffi::ACONFIGURATION_LAYOUTDIR_RTL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UiModeType {
    Any = ffi::ACONFIGURATION_UI_MODE_TYPE_ANY,
    Normal = ffi::ACONFIGURATION_UI_MODE_TYPE_NORMAL,
    Desk = ffi::ACONFIGURATION_UI_MODE_TYPE_DESK,
    Car = ffi::ACONFIGURATION_UI_MODE_TYPE_CAR,
    Television = ffi::ACONFIGURATION_UI_MODE_TYPE_TELEVISION,
    Applicance = ffi::ACONFIGURATION_UI_MODE_TYPE_APPLIANCE,
    Watch = ffi::ACONFIGURATION_UI_MODE_TYPE_WATCH,
    VrHeadset = ffi::ACONFIGURATION_UI_MODE_TYPE_VR_HEADSET,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum UiModeNight {
    Any = ffi::ACONFIGURATION_UI_MODE_NIGHT_ANY,
    No = ffi::ACONFIGURATION_UI_MODE_NIGHT_NO,
    Yes = ffi::ACONFIGURATION_UI_MODE_NIGHT_YES,
}
