#[doc = "Register `IRQ_INTE` reader"]
pub type R = crate::R<IRQ_INTE_SPEC>;
#[doc = "Register `IRQ_INTE` writer"]
pub type W = crate::W<IRQ_INTE_SPEC>;
#[doc = "Field `SM0_RXNEMPTY` reader - "]
pub type SM0_RXNEMPTY_R = crate::BitReader;
#[doc = "Field `SM0_RXNEMPTY` writer - "]
pub type SM0_RXNEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM1_RXNEMPTY` reader - "]
pub type SM1_RXNEMPTY_R = crate::BitReader;
#[doc = "Field `SM1_RXNEMPTY` writer - "]
pub type SM1_RXNEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM2_RXNEMPTY` reader - "]
pub type SM2_RXNEMPTY_R = crate::BitReader;
#[doc = "Field `SM2_RXNEMPTY` writer - "]
pub type SM2_RXNEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM3_RXNEMPTY` reader - "]
pub type SM3_RXNEMPTY_R = crate::BitReader;
#[doc = "Field `SM3_RXNEMPTY` writer - "]
pub type SM3_RXNEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM0_TXNFULL` reader - "]
pub type SM0_TXNFULL_R = crate::BitReader;
#[doc = "Field `SM0_TXNFULL` writer - "]
pub type SM0_TXNFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM1_TXNFULL` reader - "]
pub type SM1_TXNFULL_R = crate::BitReader;
#[doc = "Field `SM1_TXNFULL` writer - "]
pub type SM1_TXNFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM2_TXNFULL` reader - "]
pub type SM2_TXNFULL_R = crate::BitReader;
#[doc = "Field `SM2_TXNFULL` writer - "]
pub type SM2_TXNFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM3_TXNFULL` reader - "]
pub type SM3_TXNFULL_R = crate::BitReader;
#[doc = "Field `SM3_TXNFULL` writer - "]
pub type SM3_TXNFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM0` reader - "]
pub type SM0_R = crate::BitReader;
#[doc = "Field `SM0` writer - "]
pub type SM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM1` reader - "]
pub type SM1_R = crate::BitReader;
#[doc = "Field `SM1` writer - "]
pub type SM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM2` reader - "]
pub type SM2_R = crate::BitReader;
#[doc = "Field `SM2` writer - "]
pub type SM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM3` reader - "]
pub type SM3_R = crate::BitReader;
#[doc = "Field `SM3` writer - "]
pub type SM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM4` reader - "]
pub type SM4_R = crate::BitReader;
#[doc = "Field `SM4` writer - "]
pub type SM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM5` reader - "]
pub type SM5_R = crate::BitReader;
#[doc = "Field `SM5` writer - "]
pub type SM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM6` reader - "]
pub type SM6_R = crate::BitReader;
#[doc = "Field `SM6` writer - "]
pub type SM6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM7` reader - "]
pub type SM7_R = crate::BitReader;
#[doc = "Field `SM7` writer - "]
pub type SM7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sm0_rxnempty(&self) -> SM0_RXNEMPTY_R {
        SM0_RXNEMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sm1_rxnempty(&self) -> SM1_RXNEMPTY_R {
        SM1_RXNEMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sm2_rxnempty(&self) -> SM2_RXNEMPTY_R {
        SM2_RXNEMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sm3_rxnempty(&self) -> SM3_RXNEMPTY_R {
        SM3_RXNEMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sm0_txnfull(&self) -> SM0_TXNFULL_R {
        SM0_TXNFULL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sm1_txnfull(&self) -> SM1_TXNFULL_R {
        SM1_TXNFULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sm2_txnfull(&self) -> SM2_TXNFULL_R {
        SM2_TXNFULL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sm3_txnfull(&self) -> SM3_TXNFULL_R {
        SM3_TXNFULL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sm0(&self) -> SM0_R {
        SM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sm1(&self) -> SM1_R {
        SM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sm2(&self) -> SM2_R {
        SM2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sm3(&self) -> SM3_R {
        SM3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sm4(&self) -> SM4_R {
        SM4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sm5(&self) -> SM5_R {
        SM5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sm6(&self) -> SM6_R {
        SM6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sm7(&self) -> SM7_R {
        SM7_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sm0_rxnempty(&mut self) -> SM0_RXNEMPTY_W<IRQ_INTE_SPEC> {
        SM0_RXNEMPTY_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sm1_rxnempty(&mut self) -> SM1_RXNEMPTY_W<IRQ_INTE_SPEC> {
        SM1_RXNEMPTY_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sm2_rxnempty(&mut self) -> SM2_RXNEMPTY_W<IRQ_INTE_SPEC> {
        SM2_RXNEMPTY_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sm3_rxnempty(&mut self) -> SM3_RXNEMPTY_W<IRQ_INTE_SPEC> {
        SM3_RXNEMPTY_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sm0_txnfull(&mut self) -> SM0_TXNFULL_W<IRQ_INTE_SPEC> {
        SM0_TXNFULL_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn sm1_txnfull(&mut self) -> SM1_TXNFULL_W<IRQ_INTE_SPEC> {
        SM1_TXNFULL_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn sm2_txnfull(&mut self) -> SM2_TXNFULL_W<IRQ_INTE_SPEC> {
        SM2_TXNFULL_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sm3_txnfull(&mut self) -> SM3_TXNFULL_W<IRQ_INTE_SPEC> {
        SM3_TXNFULL_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sm0(&mut self) -> SM0_W<IRQ_INTE_SPEC> {
        SM0_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sm1(&mut self) -> SM1_W<IRQ_INTE_SPEC> {
        SM1_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn sm2(&mut self) -> SM2_W<IRQ_INTE_SPEC> {
        SM2_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn sm3(&mut self) -> SM3_W<IRQ_INTE_SPEC> {
        SM3_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn sm4(&mut self) -> SM4_W<IRQ_INTE_SPEC> {
        SM4_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn sm5(&mut self) -> SM5_W<IRQ_INTE_SPEC> {
        SM5_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn sm6(&mut self) -> SM6_W<IRQ_INTE_SPEC> {
        SM6_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn sm7(&mut self) -> SM7_W<IRQ_INTE_SPEC> {
        SM7_W::new(self, 15)
    }
}
#[doc = "Interrupt Enable for irq0  

You can [`read`](crate::Reg::read) this register and get [`irq_inte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_inte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_INTE_SPEC;
impl crate::RegisterSpec for IRQ_INTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_inte::R`](R) reader structure"]
impl crate::Readable for IRQ_INTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq_inte::W`](W) writer structure"]
impl crate::Writable for IRQ_INTE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_INTE to value 0"]
impl crate::Resettable for IRQ_INTE_SPEC {
    const RESET_VALUE: u32 = 0;
}
