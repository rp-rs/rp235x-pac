#[doc = "Register `CRT_KEY_W0` reader"]
pub type R = crate::R<CRT_KEY_W0_SPEC>;
#[doc = "Register `CRT_KEY_W0` writer"]
pub type W = crate::W<CRT_KEY_W0_SPEC>;
#[doc = "Field `CRT_KEY_W0` writer - "]
pub type CRT_KEY_W0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn crt_key_w0(&mut self) -> CRT_KEY_W0_W<CRT_KEY_W0_SPEC> {
        CRT_KEY_W0_W::new(self, 0)
    }
}
#[doc = "Word 0 (bits 31..0) of the key. Write only, read returns 0x0  

You can [`read`](crate::Reg::read) this register and get [`crt_key_w0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crt_key_w0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRT_KEY_W0_SPEC;
impl crate::RegisterSpec for CRT_KEY_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crt_key_w0::R`](R) reader structure"]
impl crate::Readable for CRT_KEY_W0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crt_key_w0::W`](W) writer structure"]
impl crate::Writable for CRT_KEY_W0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRT_KEY_W0 to value 0"]
impl crate::Resettable for CRT_KEY_W0_SPEC {
    const RESET_VALUE: u32 = 0;
}