#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    proc_config: PROC_CONFIG,
    proc_in_sync_bypass: PROC_IN_SYNC_BYPASS,
    proc_in_sync_bypass_hi: PROC_IN_SYNC_BYPASS_HI,
    dbgforce: DBGFORCE,
    mempowerdown: MEMPOWERDOWN,
    auxctrl: AUXCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Configuration for processors"]
    #[inline(always)]
    pub const fn proc_config(&self) -> &PROC_CONFIG {
        &self.proc_config
    }
    #[doc = "0x04 - For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 0...31."]
    #[inline(always)]
    pub const fn proc_in_sync_bypass(&self) -> &PROC_IN_SYNC_BYPASS {
        &self.proc_in_sync_bypass
    }
    #[doc = "0x08 - For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 32...47. USB GPIO 56..57 QSPI GPIO 58..63"]
    #[inline(always)]
    pub const fn proc_in_sync_bypass_hi(&self) -> &PROC_IN_SYNC_BYPASS_HI {
        &self.proc_in_sync_bypass_hi
    }
    #[doc = "0x0c - Directly control the chip SWD debug port"]
    #[inline(always)]
    pub const fn dbgforce(&self) -> &DBGFORCE {
        &self.dbgforce
    }
    #[doc = "0x10 - Control PD pins to memories. Set high to put memories to a low power state. In this state the memories will retain contents but not be accessible Use with caution"]
    #[inline(always)]
    pub const fn mempowerdown(&self) -> &MEMPOWERDOWN {
        &self.mempowerdown
    }
    #[doc = "0x14 - Auxiliary system control register"]
    #[inline(always)]
    pub const fn auxctrl(&self) -> &AUXCTRL {
        &self.auxctrl
    }
}
#[doc = "PROC_CONFIG (rw) register accessor: Configuration for processors  

You can [`read`](crate::Reg::read) this register and get [`proc_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc_config`]
module"]
pub type PROC_CONFIG = crate::Reg<proc_config::PROC_CONFIG_SPEC>;
#[doc = "Configuration for processors"]
pub mod proc_config;
#[doc = "PROC_IN_SYNC_BYPASS (rw) register accessor: For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 0...31.  

You can [`read`](crate::Reg::read) this register and get [`proc_in_sync_bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc_in_sync_bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc_in_sync_bypass`]
module"]
pub type PROC_IN_SYNC_BYPASS = crate::Reg<proc_in_sync_bypass::PROC_IN_SYNC_BYPASS_SPEC>;
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 0...31."]
pub mod proc_in_sync_bypass;
#[doc = "PROC_IN_SYNC_BYPASS_HI (rw) register accessor: For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 32...47. USB GPIO 56..57 QSPI GPIO 58..63  

You can [`read`](crate::Reg::read) this register and get [`proc_in_sync_bypass_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc_in_sync_bypass_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@proc_in_sync_bypass_hi`]
module"]
pub type PROC_IN_SYNC_BYPASS_HI = crate::Reg<proc_in_sync_bypass_hi::PROC_IN_SYNC_BYPASS_HI_SPEC>;
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 32...47. USB GPIO 56..57 QSPI GPIO 58..63"]
pub mod proc_in_sync_bypass_hi;
#[doc = "DBGFORCE (rw) register accessor: Directly control the chip SWD debug port  

You can [`read`](crate::Reg::read) this register and get [`dbgforce::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgforce::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbgforce`]
module"]
pub type DBGFORCE = crate::Reg<dbgforce::DBGFORCE_SPEC>;
#[doc = "Directly control the chip SWD debug port"]
pub mod dbgforce;
#[doc = "MEMPOWERDOWN (rw) register accessor: Control PD pins to memories. Set high to put memories to a low power state. In this state the memories will retain contents but not be accessible Use with caution  

You can [`read`](crate::Reg::read) this register and get [`mempowerdown::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mempowerdown::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mempowerdown`]
module"]
pub type MEMPOWERDOWN = crate::Reg<mempowerdown::MEMPOWERDOWN_SPEC>;
#[doc = "Control PD pins to memories. Set high to put memories to a low power state. In this state the memories will retain contents but not be accessible Use with caution"]
pub mod mempowerdown;
#[doc = "AUXCTRL (rw) register accessor: Auxiliary system control register  

You can [`read`](crate::Reg::read) this register and get [`auxctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`auxctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@auxctrl`]
module"]
pub type AUXCTRL = crate::Reg<auxctrl::AUXCTRL_SPEC>;
#[doc = "Auxiliary system control register"]
pub mod auxctrl;
