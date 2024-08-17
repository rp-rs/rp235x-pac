#[doc = "Register `ITM_CIDR2` reader"]
pub type R = crate::R<ITM_CIDR2_SPEC>;
#[doc = "Register `ITM_CIDR2` writer"]
pub type W = crate::W<ITM_CIDR2_SPEC>;
#[doc = "Field `PRMBL_2` reader - See CoreSight Architecture Specification"]
pub type PRMBL_2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_cidr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_cidr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITM_CIDR2_SPEC;
impl crate::RegisterSpec for ITM_CIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itm_cidr2::R`](R) reader structure"]
impl crate::Readable for ITM_CIDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itm_cidr2::W`](W) writer structure"]
impl crate::Writable for ITM_CIDR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITM_CIDR2 to value 0x05"]
impl crate::Resettable for ITM_CIDR2_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
