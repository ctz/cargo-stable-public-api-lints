#![cfg_attr(spal, feature(register_tool))]
#![cfg_attr(spal, register_tool(spal))]

#[cfg(feature = "enum_allow")]
pub mod enum_allow;
#[cfg(feature = "enum_fixed")]
pub mod enum_fixed;
#[cfg(feature = "enum_warn")]
pub mod enum_warn;
#[cfg(feature = "struct_allow")]
pub mod struct_allow;
#[cfg(feature = "struct_fixed")]
pub mod struct_fixed;
#[cfg(feature = "struct_warn")]
pub mod struct_warn;
