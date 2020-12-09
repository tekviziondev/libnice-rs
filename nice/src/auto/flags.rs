// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
bitflags! {
    pub struct AgentOption: u32 {
        const REGULAR_NOMINATION = 1;
        const RELIABLE = 2;
        const LITE_MODE = 4;
        const ICE_TRICKLE = 8;
        const SUPPORT_RENOMINATION = 16;
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
impl fmt::Display for AgentOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
#[doc(hidden)]
impl ToGlib for AgentOption {
    type GlibType = ffi::NiceAgentOption;

    fn to_glib(&self) -> ffi::NiceAgentOption {
        self.bits()
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
#[doc(hidden)]
impl FromGlib<ffi::NiceAgentOption> for AgentOption {
    unsafe fn from_glib(value: ffi::NiceAgentOption) -> AgentOption {
        skip_assert_initialized!();
        AgentOption::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
impl StaticType for AgentOption {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nice_agent_option_get_type()) }
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
impl<'a> FromValueOptional<'a> for AgentOption {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
impl<'a> FromValue<'a> for AgentOption {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
impl SetValue for AgentOption {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

