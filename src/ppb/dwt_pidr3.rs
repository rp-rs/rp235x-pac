#[doc = "Register `DWT_PIDR3` reader"]
pub type R = crate::R<DWT_PIDR3_SPEC>;
#[doc = "Register `DWT_PIDR3` writer"]
pub type W = crate::W<DWT_PIDR3_SPEC>;
#[doc = "Field `CMOD` reader - See CoreSight Architecture Specification"]
pub type CMOD_R = crate::FieldReader;
#[doc = "Field `REVAND` reader - See CoreSight Architecture Specification"]
pub type REVAND_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the DWT  

You can [`read`](crate::Reg::read) this register and get [`dwt_pidr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_pidr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_PIDR3_SPEC;
impl crate::RegisterSpec for DWT_PIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_pidr3::R`](R) reader structure"]
impl crate::Readable for DWT_PIDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dwt_pidr3::W`](W) writer structure"]
impl crate::Writable for DWT_PIDR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DWT_PIDR3 to value 0"]
impl crate::Resettable for DWT_PIDR3_SPEC {
    const RESET_VALUE: u32 = 0;
}