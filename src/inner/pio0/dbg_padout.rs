#[doc = "Register `DBG_PADOUT` reader"]
pub type R = crate::R<DBG_PADOUT_SPEC>;
#[doc = "Register `DBG_PADOUT` writer"]
pub type W = crate::W<DBG_PADOUT_SPEC>;
#[doc = "Field `DBG_PADOUT` reader - "]
pub type DBG_PADOUT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dbg_padout(&self) -> DBG_PADOUT_R {
        DBG_PADOUT_R::new(self.bits)
    }
}
impl W {}
#[doc = "Read to sample the pad output values PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`dbg_padout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_padout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_PADOUT_SPEC;
impl crate::RegisterSpec for DBG_PADOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_padout::R`](R) reader structure"]
impl crate::Readable for DBG_PADOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_padout::W`](W) writer structure"]
impl crate::Writable for DBG_PADOUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_PADOUT to value 0"]
impl crate::Resettable for DBG_PADOUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
