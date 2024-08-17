#[doc = "Register `BOOTKEY2_1` reader"]
pub type R = crate::R<BOOTKEY2_1_SPEC>;
#[doc = "Register `BOOTKEY2_1` writer"]
pub type W = crate::W<BOOTKEY2_1_SPEC>;
#[doc = "Field `BOOTKEY2_1` reader - "]
pub type BOOTKEY2_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn bootkey2_1(&self) -> BOOTKEY2_1_R {
        BOOTKEY2_1_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 31:16 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY2_1_SPEC;
impl crate::RegisterSpec for BOOTKEY2_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootkey2_1::R`](R) reader structure"]
impl crate::Readable for BOOTKEY2_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootkey2_1::W`](W) writer structure"]
impl crate::Writable for BOOTKEY2_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTKEY2_1 to value 0"]
impl crate::Resettable for BOOTKEY2_1_SPEC {
    const RESET_VALUE: u32 = 0;
}