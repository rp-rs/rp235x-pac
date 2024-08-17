#[doc = "Register `CTIINEN0` reader"]
pub type R = crate::R<CTIINEN0_SPEC>;
#[doc = "Register `CTIINEN0` writer"]
pub type W = crate::W<CTIINEN0_SPEC>;
#[doc = "Field `TRIGINEN` reader - Enables a cross trigger event to the corresponding channel when a ctitrigin input is activated. There is one bit of the field for each of the four channels"]
pub type TRIGINEN_R = crate::FieldReader;
#[doc = "Field `TRIGINEN` writer - Enables a cross trigger event to the corresponding channel when a ctitrigin input is activated. There is one bit of the field for each of the four channels"]
pub type TRIGINEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Enables a cross trigger event to the corresponding channel when a ctitrigin input is activated. There is one bit of the field for each of the four channels"]
    #[inline(always)]
    pub fn triginen(&self) -> TRIGINEN_R {
        TRIGINEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Enables a cross trigger event to the corresponding channel when a ctitrigin input is activated. There is one bit of the field for each of the four channels"]
    #[inline(always)]
    #[must_use]
    pub fn triginen(&mut self) -> TRIGINEN_W<CTIINEN0_SPEC> {
        TRIGINEN_W::new(self, 0)
    }
}
#[doc = "CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctiinen0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiinen0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTIINEN0_SPEC;
impl crate::RegisterSpec for CTIINEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctiinen0::R`](R) reader structure"]
impl crate::Readable for CTIINEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctiinen0::W`](W) writer structure"]
impl crate::Writable for CTIINEN0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTIINEN0 to value 0"]
impl crate::Resettable for CTIINEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}