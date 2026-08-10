#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    rng_imr: RNG_IMR,
    rng_isr: RNG_ISR,
    rng_icr: RNG_ICR,
    trng_config: TRNG_CONFIG,
    trng_valid: TRNG_VALID,
    ehr_data: [EHR_DATA; 6],
    rnd_source_enable: RND_SOURCE_ENABLE,
    sample_cnt1: SAMPLE_CNT1,
    autocorr_statistic: AUTOCORR_STATISTIC,
    trng_debug_control: TRNG_DEBUG_CONTROL,
    _reserved10: [u8; 0x04],
    trng_sw_reset: TRNG_SW_RESET,
    _reserved11: [u8; 0x70],
    rng_debug_en_input: RNG_DEBUG_EN_INPUT,
    trng_busy: TRNG_BUSY,
    rst_bits_counter: RST_BITS_COUNTER,
    rng_version: RNG_VERSION,
    _reserved15: [u8; 0x1c],
    rng_bist_cntr_0: RNG_BIST_CNTR_0,
    rng_bist_cntr_1: RNG_BIST_CNTR_1,
    rng_bist_cntr_2: RNG_BIST_CNTR_2,
}
impl RegisterBlock {
    #[doc = "0x100 - Interrupt masking."]
    #[inline(always)]
    pub const fn rng_imr(&self) -> &RNG_IMR {
        &self.rng_imr
    }
    #[doc = "0x104 - RNG status register. If corresponding RNG_IMR bit is unmasked, an interrupt will be generated."]
    #[inline(always)]
    pub const fn rng_isr(&self) -> &RNG_ISR {
        &self.rng_isr
    }
    #[doc = "0x108 - Interrupt/status bit clear Register."]
    #[inline(always)]
    pub const fn rng_icr(&self) -> &RNG_ICR {
        &self.rng_icr
    }
    #[doc = "0x10c - Selecting the inverter-chain length."]
    #[inline(always)]
    pub const fn trng_config(&self) -> &TRNG_CONFIG {
        &self.trng_config
    }
    #[doc = "0x110 - 192 bit collection indication."]
    #[inline(always)]
    pub const fn trng_valid(&self) -> &TRNG_VALID {
        &self.trng_valid
    }
    #[doc = "0x114..0x12c - RNG collected bits. Reading EHR_DATA\\[5\\]
clears the contents of all 6 registers."]
    #[inline(always)]
    pub const fn ehr_data(&self, n: usize) -> &EHR_DATA {
        &self.ehr_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x114..0x12c - RNG collected bits. Reading EHR_DATA\\[5\\]
clears the contents of all 6 registers."]
    #[inline(always)]
    pub fn ehr_data_iter(&self) -> impl Iterator<Item = &EHR_DATA> {
        self.ehr_data.iter()
    }
    #[doc = "0x12c - Enable signal for the random source."]
    #[inline(always)]
    pub const fn rnd_source_enable(&self) -> &RND_SOURCE_ENABLE {
        &self.rnd_source_enable
    }
    #[doc = "0x130 - Counts clocks between sampling of random bit."]
    #[inline(always)]
    pub const fn sample_cnt1(&self) -> &SAMPLE_CNT1 {
        &self.sample_cnt1
    }
    #[doc = "0x134 - Statistic about Autocorrelation test activations."]
    #[inline(always)]
    pub const fn autocorr_statistic(&self) -> &AUTOCORR_STATISTIC {
        &self.autocorr_statistic
    }
    #[doc = "0x138 - Debug register."]
    #[inline(always)]
    pub const fn trng_debug_control(&self) -> &TRNG_DEBUG_CONTROL {
        &self.trng_debug_control
    }
    #[doc = "0x140 - Generate internal SW reset within the RNG block."]
    #[inline(always)]
    pub const fn trng_sw_reset(&self) -> &TRNG_SW_RESET {
        &self.trng_sw_reset
    }
    #[doc = "0x1b4 - Enable the RNG debug mode"]
    #[inline(always)]
    pub const fn rng_debug_en_input(&self) -> &RNG_DEBUG_EN_INPUT {
        &self.rng_debug_en_input
    }
    #[doc = "0x1b8 - RNG Busy indication."]
    #[inline(always)]
    pub const fn trng_busy(&self) -> &TRNG_BUSY {
        &self.trng_busy
    }
    #[doc = "0x1bc - Reset the counter of collected bits in the RNG."]
    #[inline(always)]
    pub const fn rst_bits_counter(&self) -> &RST_BITS_COUNTER {
        &self.rst_bits_counter
    }
    #[doc = "0x1c0 - Displays the version settings of the TRNG."]
    #[inline(always)]
    pub const fn rng_version(&self) -> &RNG_VERSION {
        &self.rng_version
    }
    #[doc = "0x1e0 - Collected BIST results."]
    #[inline(always)]
    pub const fn rng_bist_cntr_0(&self) -> &RNG_BIST_CNTR_0 {
        &self.rng_bist_cntr_0
    }
    #[doc = "0x1e4 - Collected BIST results."]
    #[inline(always)]
    pub const fn rng_bist_cntr_1(&self) -> &RNG_BIST_CNTR_1 {
        &self.rng_bist_cntr_1
    }
    #[doc = "0x1e8 - Collected BIST results."]
    #[inline(always)]
    pub const fn rng_bist_cntr_2(&self) -> &RNG_BIST_CNTR_2 {
        &self.rng_bist_cntr_2
    }
}
#[doc = "RNG_IMR (rw) register accessor: Interrupt masking.  

You can [`read`](crate::Reg::read) this register and get [`rng_imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rng_imr`]
module"]
pub type RNG_IMR = crate::Reg<rng_imr::RNG_IMR_SPEC>;
#[doc = "Interrupt masking."]
pub mod rng_imr;
#[doc = "RNG_ISR (rw) register accessor: RNG status register. If corresponding RNG_IMR bit is unmasked, an interrupt will be generated.  

You can [`read`](crate::Reg::read) this register and get [`rng_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rng_isr`]
module"]
pub type RNG_ISR = crate::Reg<rng_isr::RNG_ISR_SPEC>;
#[doc = "RNG status register. If corresponding RNG_IMR bit is unmasked, an interrupt will be generated."]
pub mod rng_isr;
#[doc = "RNG_ICR (rw) register accessor: Interrupt/status bit clear Register.  

You can [`read`](crate::Reg::read) this register and get [`rng_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rng_icr`]
module"]
pub type RNG_ICR = crate::Reg<rng_icr::RNG_ICR_SPEC>;
#[doc = "Interrupt/status bit clear Register."]
pub mod rng_icr;
#[doc = "TRNG_CONFIG (rw) register accessor: Selecting the inverter-chain length.  

You can [`read`](crate::Reg::read) this register and get [`trng_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trng_config`]
module"]
pub type TRNG_CONFIG = crate::Reg<trng_config::TRNG_CONFIG_SPEC>;
#[doc = "Selecting the inverter-chain length."]
pub mod trng_config;
#[doc = "TRNG_VALID (rw) register accessor: 192 bit collection indication.  

You can [`read`](crate::Reg::read) this register and get [`trng_valid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_valid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trng_valid`]
module"]
pub type TRNG_VALID = crate::Reg<trng_valid::TRNG_VALID_SPEC>;
#[doc = "192 bit collection indication."]
pub mod trng_valid;
#[doc = "EHR_DATA (rw) register accessor: RNG collected bits. Reading EHR_DATA\\[5\\]
clears the contents of all 6 registers.  

You can [`read`](crate::Reg::read) this register and get [`ehr_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehr_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ehr_data`]
module"]
pub type EHR_DATA = crate::Reg<ehr_data::EHR_DATA_SPEC>;
#[doc = "RNG collected bits. Reading EHR_DATA\\[5\\]
clears the contents of all 6 registers."]
pub mod ehr_data;
#[doc = "RND_SOURCE_ENABLE (rw) register accessor: Enable signal for the random source.  

You can [`read`](crate::Reg::read) this register and get [`rnd_source_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_source_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rnd_source_enable`]
module"]
pub type RND_SOURCE_ENABLE = crate::Reg<rnd_source_enable::RND_SOURCE_ENABLE_SPEC>;
#[doc = "Enable signal for the random source."]
pub mod rnd_source_enable;
#[doc = "SAMPLE_CNT1 (rw) register accessor: Counts clocks between sampling of random bit.  

You can [`read`](crate::Reg::read) this register and get [`sample_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sample_cnt1`]
module"]
pub type SAMPLE_CNT1 = crate::Reg<sample_cnt1::SAMPLE_CNT1_SPEC>;
#[doc = "Counts clocks between sampling of random bit."]
pub mod sample_cnt1;
#[doc = "AUTOCORR_STATISTIC (rw) register accessor: Statistic about Autocorrelation test activations.  

You can [`read`](crate::Reg::read) this register and get [`autocorr_statistic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocorr_statistic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@autocorr_statistic`]
module"]
pub type AUTOCORR_STATISTIC = crate::Reg<autocorr_statistic::AUTOCORR_STATISTIC_SPEC>;
#[doc = "Statistic about Autocorrelation test activations."]
pub mod autocorr_statistic;
#[doc = "TRNG_DEBUG_CONTROL (rw) register accessor: Debug register.  

You can [`read`](crate::Reg::read) this register and get [`trng_debug_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_debug_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trng_debug_control`]
module"]
pub type TRNG_DEBUG_CONTROL = crate::Reg<trng_debug_control::TRNG_DEBUG_CONTROL_SPEC>;
#[doc = "Debug register."]
pub mod trng_debug_control;
#[doc = "TRNG_SW_RESET (rw) register accessor: Generate internal SW reset within the RNG block.  

You can [`read`](crate::Reg::read) this register and get [`trng_sw_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_sw_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trng_sw_reset`]
module"]
pub type TRNG_SW_RESET = crate::Reg<trng_sw_reset::TRNG_SW_RESET_SPEC>;
#[doc = "Generate internal SW reset within the RNG block."]
pub mod trng_sw_reset;
#[doc = "RNG_DEBUG_EN_INPUT (rw) register accessor: Enable the RNG debug mode  

You can [`read`](crate::Reg::read) this register and get [`rng_debug_en_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_debug_en_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rng_debug_en_input`]
module"]
pub type RNG_DEBUG_EN_INPUT = crate::Reg<rng_debug_en_input::RNG_DEBUG_EN_INPUT_SPEC>;
#[doc = "Enable the RNG debug mode"]
pub mod rng_debug_en_input;
#[doc = "TRNG_BUSY (rw) register accessor: RNG Busy indication.  

You can [`read`](crate::Reg::read) this register and get [`trng_busy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_busy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trng_busy`]
module"]
pub type TRNG_BUSY = crate::Reg<trng_busy::TRNG_BUSY_SPEC>;
#[doc = "RNG Busy indication."]
pub mod trng_busy;
#[doc = "RST_BITS_COUNTER (rw) register accessor: Reset the counter of collected bits in the RNG.  

You can [`read`](crate::Reg::read) this register and get [`rst_bits_counter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_bits_counter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rst_bits_counter`]
module"]
pub type RST_BITS_COUNTER = crate::Reg<rst_bits_counter::RST_BITS_COUNTER_SPEC>;
#[doc = "Reset the counter of collected bits in the RNG."]
pub mod rst_bits_counter;
#[doc = "RNG_VERSION (rw) register accessor: Displays the version settings of the TRNG.  

You can [`read`](crate::Reg::read) this register and get [`rng_version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rng_version`]
module"]
pub type RNG_VERSION = crate::Reg<rng_version::RNG_VERSION_SPEC>;
#[doc = "Displays the version settings of the TRNG."]
pub mod rng_version;
#[doc = "RNG_BIST_CNTR_0 (rw) register accessor: Collected BIST results.  

You can [`read`](crate::Reg::read) this register and get [`rng_bist_cntr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_bist_cntr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rng_bist_cntr_0`]
module"]
pub type RNG_BIST_CNTR_0 = crate::Reg<rng_bist_cntr_0::RNG_BIST_CNTR_0_SPEC>;
#[doc = "Collected BIST results."]
pub mod rng_bist_cntr_0;
#[doc = "RNG_BIST_CNTR_1 (rw) register accessor: Collected BIST results.  

You can [`read`](crate::Reg::read) this register and get [`rng_bist_cntr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_bist_cntr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rng_bist_cntr_1`]
module"]
pub type RNG_BIST_CNTR_1 = crate::Reg<rng_bist_cntr_1::RNG_BIST_CNTR_1_SPEC>;
#[doc = "Collected BIST results."]
pub mod rng_bist_cntr_1;
#[doc = "RNG_BIST_CNTR_2 (rw) register accessor: Collected BIST results.  

You can [`read`](crate::Reg::read) this register and get [`rng_bist_cntr_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_bist_cntr_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rng_bist_cntr_2`]
module"]
pub type RNG_BIST_CNTR_2 = crate::Reg<rng_bist_cntr_2::RNG_BIST_CNTR_2_SPEC>;
#[doc = "Collected BIST results."]
pub mod rng_bist_cntr_2;
