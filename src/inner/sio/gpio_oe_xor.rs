#[doc = "Register `GPIO_OE_XOR` reader"]
pub type R = crate::R<GPIO_OE_XOR_SPEC>;
#[doc = "Register `GPIO_OE_XOR` writer"]
pub type W = crate::W<GPIO_OE_XOR_SPEC>;
#[doc = "Field `GPIO_OE_XOR` writer - Perform an atomic bitwise XOR on GPIO_OE, i.e. `GPIO_OE ^= wdata`"]
pub type GPIO_OE_XOR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Perform an atomic bitwise XOR on GPIO_OE, i.e. `GPIO_OE ^= wdata`"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_oe_xor(&mut self) -> GPIO_OE_XOR_W<GPIO_OE_XOR_SPEC> {
        GPIO_OE_XOR_W::new(self, 0)
    }
}
#[doc = "GPIO0...31 output enable XOR  

You can [`read`](crate::Reg::read) this register and get [`gpio_oe_xor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_oe_xor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OE_XOR_SPEC;
impl crate::RegisterSpec for GPIO_OE_XOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_oe_xor::R`](R) reader structure"]
impl crate::Readable for GPIO_OE_XOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_oe_xor::W`](W) writer structure"]
impl crate::Writable for GPIO_OE_XOR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_OE_XOR to value 0"]
impl crate::Resettable for GPIO_OE_XOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
