#[doc = "Register `DWT_COMP0` reader"]
pub type R = crate::R<DWT_COMP0_SPEC>;
#[doc = "Register `DWT_COMP0` writer"]
pub type W = crate::W<DWT_COMP0_SPEC>;
#[doc = "Field `DWT_COMP0` reader - "]
pub type DWT_COMP0_R = crate::FieldReader<u32>;
#[doc = "Field `DWT_COMP0` writer - "]
pub type DWT_COMP0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dwt_comp0(&self) -> DWT_COMP0_R {
        DWT_COMP0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dwt_comp0(&mut self) -> DWT_COMP0_W<DWT_COMP0_SPEC> {
        DWT_COMP0_W::new(self, 0)
    }
}
#[doc = "Provides a reference value for use by watchpoint comparator 0  

You can [`read`](crate::Reg::read) this register and get [`dwt_comp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_comp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_COMP0_SPEC;
impl crate::RegisterSpec for DWT_COMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_comp0::R`](R) reader structure"]
impl crate::Readable for DWT_COMP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dwt_comp0::W`](W) writer structure"]
impl crate::Writable for DWT_COMP0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DWT_COMP0 to value 0"]
impl crate::Resettable for DWT_COMP0_SPEC {
    const RESET_VALUE: u32 = 0;
}