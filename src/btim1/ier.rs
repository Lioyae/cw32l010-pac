#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `UIE` reader - desc UIE"]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - desc UIE"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - desc TIE"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - desc TIE"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - desc TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UIE"]
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<'_, IerSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 6 - desc TIE"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, IerSpec> {
        TieW::new(self, 6)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
