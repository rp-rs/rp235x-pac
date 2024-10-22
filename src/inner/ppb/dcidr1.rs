#[doc = "Register `DCIDR1` reader"]
pub type R = crate::R<DCIDR1_SPEC>;
#[doc = "Register `DCIDR1` writer"]
pub type W = crate::W<DCIDR1_SPEC>;
#[doc = "Field `PRMBL_1` reader - See CoreSight Architecture Specification"]
pub type PRMBL_1_R = crate::FieldReader;
#[doc = "Field `CLASS` reader - See CoreSight Architecture Specification"]
pub type CLASS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn prmbl_1(&self) -> PRMBL_1_R {
        PRMBL_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dcidr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcidr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCIDR1_SPEC;
impl crate::RegisterSpec for DCIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcidr1::R`](R) reader structure"]
impl crate::Readable for DCIDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcidr1::W`](W) writer structure"]
impl crate::Writable for DCIDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCIDR1 to value 0x90"]
impl crate::Resettable for DCIDR1_SPEC {
    const RESET_VALUE: u32 = 0x90;
}
