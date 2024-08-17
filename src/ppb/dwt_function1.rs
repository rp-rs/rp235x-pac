#[doc = "Register `DWT_FUNCTION1` reader"]
pub type R = crate::R<DWT_FUNCTION1_SPEC>;
#[doc = "Register `DWT_FUNCTION1` writer"]
pub type W = crate::W<DWT_FUNCTION1_SPEC>;
#[doc = "Field `MATCH` reader - Controls the type of match generated by this comparator"]
pub type MATCH_R = crate::FieldReader;
#[doc = "Field `MATCH` writer - Controls the type of match generated by this comparator"]
pub type MATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACTION` reader - Defines the action on a match. This field is ignored and the comparator generates no actions if it is disabled by MATCH"]
pub type ACTION_R = crate::FieldReader;
#[doc = "Field `ACTION` writer - Defines the action on a match. This field is ignored and the comparator generates no actions if it is disabled by MATCH"]
pub type ACTION_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATAVSIZE` reader - Defines the size of the object being watched for by Data Value and Data Address comparators"]
pub type DATAVSIZE_R = crate::FieldReader;
#[doc = "Field `DATAVSIZE` writer - Defines the size of the object being watched for by Data Value and Data Address comparators"]
pub type DATAVSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MATCHED` reader - Set to 1 when the comparator matches"]
pub type MATCHED_R = crate::BitReader;
#[doc = "Field `ID` reader - Identifies the capabilities for MATCH for comparator *n"]
pub type ID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Controls the type of match generated by this comparator"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Defines the action on a match. This field is ignored and the comparator generates no actions if it is disabled by MATCH"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Defines the size of the object being watched for by Data Value and Data Address comparators"]
    #[inline(always)]
    pub fn datavsize(&self) -> DATAVSIZE_R {
        DATAVSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 24 - Set to 1 when the comparator matches"]
    #[inline(always)]
    pub fn matched(&self) -> MATCHED_R {
        MATCHED_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 27:31 - Identifies the capabilities for MATCH for comparator *n"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Controls the type of match generated by this comparator"]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<DWT_FUNCTION1_SPEC> {
        MATCH_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Defines the action on a match. This field is ignored and the comparator generates no actions if it is disabled by MATCH"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ACTION_W<DWT_FUNCTION1_SPEC> {
        ACTION_W::new(self, 4)
    }
    #[doc = "Bits 10:11 - Defines the size of the object being watched for by Data Value and Data Address comparators"]
    #[inline(always)]
    #[must_use]
    pub fn datavsize(&mut self) -> DATAVSIZE_W<DWT_FUNCTION1_SPEC> {
        DATAVSIZE_W::new(self, 10)
    }
}
#[doc = "Controls the operation of watchpoint comparator 1  

You can [`read`](crate::Reg::read) this register and get [`dwt_function1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_function1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_FUNCTION1_SPEC;
impl crate::RegisterSpec for DWT_FUNCTION1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_function1::R`](R) reader structure"]
impl crate::Readable for DWT_FUNCTION1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dwt_function1::W`](W) writer structure"]
impl crate::Writable for DWT_FUNCTION1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DWT_FUNCTION1 to value 0x8900_0828"]
impl crate::Resettable for DWT_FUNCTION1_SPEC {
    const RESET_VALUE: u32 = 0x8900_0828;
}
