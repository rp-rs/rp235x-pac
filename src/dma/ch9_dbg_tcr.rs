#[doc = "Register `CH9_DBG_TCR` reader"]
pub type R = crate::R<CH9_DBG_TCR_SPEC>;
#[doc = "Register `CH9_DBG_TCR` writer"]
pub type W = crate::W<CH9_DBG_TCR_SPEC>;
#[doc = "Field `CH9_DBG_TCR` reader - "]
pub type CH9_DBG_TCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch9_dbg_tcr(&self) -> CH9_DBG_TCR_R {
        CH9_DBG_TCR_R::new(self.bits)
    }
}
impl W {}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch9_dbg_tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_dbg_tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH9_DBG_TCR_SPEC;
impl crate::RegisterSpec for CH9_DBG_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch9_dbg_tcr::R`](R) reader structure"]
impl crate::Readable for CH9_DBG_TCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch9_dbg_tcr::W`](W) writer structure"]
impl crate::Writable for CH9_DBG_TCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH9_DBG_TCR to value 0"]
impl crate::Resettable for CH9_DBG_TCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
