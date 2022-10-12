#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_A {
    #[doc = "0x00 - Select function for this pad"]
    pub function: FUNCTION,
    #[doc = "0x04 - Configurate pad settings"]
    pub config: CONFIG,
}
#[doc = "function (rw) register accessor: an alias for `Reg<FUNCTION_SPEC>`"]
pub type FUNCTION = crate::Reg<function::FUNCTION_SPEC>;
#[doc = "Select function for this pad"]
pub mod function;
#[doc = "config (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configurate pad settings"]
pub mod config;
