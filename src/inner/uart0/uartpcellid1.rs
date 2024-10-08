#[doc = "Register `UARTPCELLID1` reader"]
pub type R = crate::R<UARTPCELLID1_SPEC>;
#[doc = "Register `UARTPCELLID1` writer"]
pub type W = crate::W<UARTPCELLID1_SPEC>;
#[doc = "Field `UARTPCELLID1` reader - These bits read back as 0xF0"]
pub type UARTPCELLID1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xF0"]
    #[inline(always)]
    pub fn uartpcellid1(&self) -> UARTPCELLID1_R {
        UARTPCELLID1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "UARTPCellID1 Register  

You can [`read`](crate::Reg::read) this register and get [`uartpcellid1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartpcellid1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTPCELLID1_SPEC;
impl crate::RegisterSpec for UARTPCELLID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartpcellid1::R`](R) reader structure"]
impl crate::Readable for UARTPCELLID1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartpcellid1::W`](W) writer structure"]
impl crate::Writable for UARTPCELLID1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTPCELLID1 to value 0xf0"]
impl crate::Resettable for UARTPCELLID1_SPEC {
    const RESET_VALUE: u32 = 0xf0;
}
