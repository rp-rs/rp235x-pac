#[doc = "Register `DPIDR7` reader"]
pub type R = crate::R<DPIDR7_SPEC>;
#[doc = "Register `DPIDR7` writer"]
pub type W = crate::W<DPIDR7_SPEC>;
#[doc = "Field `DPIDR7` reader - "]
pub type DPIDR7_R = crate::FieldReader<u32>;
#[doc = "Field `DPIDR7` writer - "]
pub type DPIDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dpidr7(&self) -> DPIDR7_R {
        DPIDR7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dpidr7(&mut self) -> DPIDR7_W<DPIDR7_SPEC> {
        DPIDR7_W::new(self, 0)
    }
}
#[doc = "Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dpidr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpidr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPIDR7_SPEC;
impl crate::RegisterSpec for DPIDR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpidr7::R`](R) reader structure"]
impl crate::Readable for DPIDR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpidr7::W`](W) writer structure"]
impl crate::Writable for DPIDR7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPIDR7 to value 0"]
impl crate::Resettable for DPIDR7_SPEC {
    const RESET_VALUE: u32 = 0;
}