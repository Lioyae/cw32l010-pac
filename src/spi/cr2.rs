#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRX` reader - desc ADCRX"]
pub type AdcrxR = crate::BitReader;
#[doc = "Field `ADCRX` writer - desc ADCRX"]
pub type AdcrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCTX` reader - desc ADCTX"]
pub type AdctxR = crate::BitReader;
#[doc = "Field `ADCTX` writer - desc ADCTX"]
pub type AdctxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - desc ADCRX"]
    #[inline(always)]
    pub fn adcrx(&self) -> AdcrxR {
        AdcrxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ADCTX"]
    #[inline(always)]
    pub fn adctx(&self) -> AdctxR {
        AdctxR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Cr2Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 3 - desc ADCRX"]
    #[inline(always)]
    pub fn adcrx(&mut self) -> AdcrxW<'_, Cr2Spec> {
        AdcrxW::new(self, 3)
    }
    #[doc = "Bit 4 - desc ADCTX"]
    #[inline(always)]
    pub fn adctx(&mut self) -> AdctxW<'_, Cr2Spec> {
        AdctxW::new(self, 4)
    }
}
#[doc = "Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
