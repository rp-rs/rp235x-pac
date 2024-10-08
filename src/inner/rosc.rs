#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    freqa: FREQA,
    freqb: FREQB,
    random: RANDOM,
    dormant: DORMANT,
    div: DIV,
    phase: PHASE,
    status: STATUS,
    randombit: RANDOMBIT,
    count: COUNT,
}
impl RegisterBlock {
    #[doc = "0x00 - Ring Oscillator control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - The FREQA &amp; FREQB registers control the frequency by controlling the drive strength of each stage The drive strength has 4 levels determined by the number of bits set Increasing the number of bits set increases the drive strength and increases the oscillation frequency 0 bits set is the default drive strength 1 bit set doubles the drive strength 2 bits set triples drive strength 3 bits set quadruples drive strength For frequency randomisation set both DS0_RANDOM=1 &amp; DS1_RANDOM=1"]
    #[inline(always)]
    pub const fn freqa(&self) -> &FREQA {
        &self.freqa
    }
    #[doc = "0x08 - For a detailed description see freqa register"]
    #[inline(always)]
    pub const fn freqb(&self) -> &FREQB {
        &self.freqb
    }
    #[doc = "0x0c - Loads a value to the LFSR randomiser"]
    #[inline(always)]
    pub const fn random(&self) -> &RANDOM {
        &self.random
    }
    #[doc = "0x10 - Ring Oscillator pause control"]
    #[inline(always)]
    pub const fn dormant(&self) -> &DORMANT {
        &self.dormant
    }
    #[doc = "0x14 - Controls the output divider"]
    #[inline(always)]
    pub const fn div(&self) -> &DIV {
        &self.div
    }
    #[doc = "0x18 - Controls the phase shifted output"]
    #[inline(always)]
    pub const fn phase(&self) -> &PHASE {
        &self.phase
    }
    #[doc = "0x1c - Ring Oscillator Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x20 - This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency"]
    #[inline(always)]
    pub const fn randombit(&self) -> &RANDOMBIT {
        &self.randombit
    }
    #[doc = "0x24 - A down counter running at the ROSC frequency which counts to zero and stops. To start the counter write a non-zero value. Can be used for short software pauses when setting up time sensitive hardware."]
    #[inline(always)]
    pub const fn count(&self) -> &COUNT {
        &self.count
    }
}
#[doc = "CTRL (rw) register accessor: Ring Oscillator control  

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Ring Oscillator control"]
pub mod ctrl;
#[doc = "FREQA (rw) register accessor: The FREQA &amp; FREQB registers control the frequency by controlling the drive strength of each stage The drive strength has 4 levels determined by the number of bits set Increasing the number of bits set increases the drive strength and increases the oscillation frequency 0 bits set is the default drive strength 1 bit set doubles the drive strength 2 bits set triples drive strength 3 bits set quadruples drive strength For frequency randomisation set both DS0_RANDOM=1 &amp; DS1_RANDOM=1  

You can [`read`](crate::Reg::read) this register and get [`freqa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@freqa`]
module"]
pub type FREQA = crate::Reg<freqa::FREQA_SPEC>;
#[doc = "The FREQA &amp; FREQB registers control the frequency by controlling the drive strength of each stage The drive strength has 4 levels determined by the number of bits set Increasing the number of bits set increases the drive strength and increases the oscillation frequency 0 bits set is the default drive strength 1 bit set doubles the drive strength 2 bits set triples drive strength 3 bits set quadruples drive strength For frequency randomisation set both DS0_RANDOM=1 &amp; DS1_RANDOM=1"]
pub mod freqa;
#[doc = "FREQB (rw) register accessor: For a detailed description see freqa register  

You can [`read`](crate::Reg::read) this register and get [`freqb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@freqb`]
module"]
pub type FREQB = crate::Reg<freqb::FREQB_SPEC>;
#[doc = "For a detailed description see freqa register"]
pub mod freqb;
#[doc = "RANDOM (rw) register accessor: Loads a value to the LFSR randomiser  

You can [`read`](crate::Reg::read) this register and get [`random::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`random::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@random`]
module"]
pub type RANDOM = crate::Reg<random::RANDOM_SPEC>;
#[doc = "Loads a value to the LFSR randomiser"]
pub mod random;
#[doc = "DORMANT (rw) register accessor: Ring Oscillator pause control  

You can [`read`](crate::Reg::read) this register and get [`dormant::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dormant::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dormant`]
module"]
pub type DORMANT = crate::Reg<dormant::DORMANT_SPEC>;
#[doc = "Ring Oscillator pause control"]
pub mod dormant;
#[doc = "DIV (rw) register accessor: Controls the output divider  

You can [`read`](crate::Reg::read) this register and get [`div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@div`]
module"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Controls the output divider"]
pub mod div;
#[doc = "PHASE (rw) register accessor: Controls the phase shifted output  

You can [`read`](crate::Reg::read) this register and get [`phase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@phase`]
module"]
pub type PHASE = crate::Reg<phase::PHASE_SPEC>;
#[doc = "Controls the phase shifted output"]
pub mod phase;
#[doc = "STATUS (rw) register accessor: Ring Oscillator Status  

You can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Ring Oscillator Status"]
pub mod status;
#[doc = "RANDOMBIT (rw) register accessor: This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency  

You can [`read`](crate::Reg::read) this register and get [`randombit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randombit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randombit`]
module"]
pub type RANDOMBIT = crate::Reg<randombit::RANDOMBIT_SPEC>;
#[doc = "This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency"]
pub mod randombit;
#[doc = "COUNT (rw) register accessor: A down counter running at the ROSC frequency which counts to zero and stops. To start the counter write a non-zero value. Can be used for short software pauses when setting up time sensitive hardware.  

You can [`read`](crate::Reg::read) this register and get [`count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@count`]
module"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "A down counter running at the ROSC frequency which counts to zero and stops. To start the counter write a non-zero value. Can be used for short software pauses when setting up time sensitive hardware."]
pub mod count;
