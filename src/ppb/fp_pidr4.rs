#[doc = "Register `FP_PIDR4` reader"]
pub type R = crate::R<FP_PIDR4_SPEC>;
#[doc = "Register `FP_PIDR4` writer"]
pub type W = crate::W<FP_PIDR4_SPEC>;
#[doc = "Field `DES_2` reader - See CoreSight Architecture Specification"]
pub type DES_2_R = crate::FieldReader;
#[doc = "Field `SIZE` reader - See CoreSight Architecture Specification"]
pub type SIZE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn des_2(&self) -> DES_2_R {
        DES_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_PIDR4_SPEC;
impl crate::RegisterSpec for FP_PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_pidr4::R`](R) reader structure"]
impl crate::Readable for FP_PIDR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fp_pidr4::W`](W) writer structure"]
impl crate::Writable for FP_PIDR4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FP_PIDR4 to value 0x04"]
impl crate::Resettable for FP_PIDR4_SPEC {
    const RESET_VALUE: u32 = 0x04;
}