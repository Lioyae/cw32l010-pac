#[doc = "Register `AF1` reader"]
pub type R = crate::R<Af1Spec>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<Af1Spec>;
#[doc = "Field `BKINE` reader - desc BKINE"]
pub type BkineR = crate::BitReader;
#[doc = "Field `BKINE` writer - desc BKINE"]
pub type BkineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKVC1E` reader - desc BKVC1E"]
pub type Bkvc1eR = crate::BitReader;
#[doc = "Field `BKVC1E` writer - desc BKVC1E"]
pub type Bkvc1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKVC2E` reader - desc BKVC2E"]
pub type Bkvc2eR = crate::BitReader;
#[doc = "Field `BKVC2E` writer - desc BKVC2E"]
pub type Bkvc2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKINP` reader - desc BKINP"]
pub type BkinpR = crate::BitReader;
#[doc = "Field `BKINP` writer - desc BKINP"]
pub type BkinpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKVC1P` reader - desc BKVC1P"]
pub type Bkvc1pR = crate::BitReader;
#[doc = "Field `BKVC1P` writer - desc BKVC1P"]
pub type Bkvc1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKVC2P` reader - desc BKVC2P"]
pub type Bkvc2pR = crate::BitReader;
#[doc = "Field `BKVC2P` writer - desc BKVC2P"]
pub type Bkvc2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETRSEL` reader - desc ETRSEL"]
pub type EtrselR = crate::FieldReader;
#[doc = "Field `ETRSEL` writer - desc ETRSEL"]
pub type EtrselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - desc BKINE"]
    #[inline(always)]
    pub fn bkine(&self) -> BkineR {
        BkineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BKVC1E"]
    #[inline(always)]
    pub fn bkvc1e(&self) -> Bkvc1eR {
        Bkvc1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BKVC2E"]
    #[inline(always)]
    pub fn bkvc2e(&self) -> Bkvc2eR {
        Bkvc2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - desc BKINP"]
    #[inline(always)]
    pub fn bkinp(&self) -> BkinpR {
        BkinpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BKVC1P"]
    #[inline(always)]
    pub fn bkvc1p(&self) -> Bkvc1pR {
        Bkvc1pR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc BKVC2P"]
    #[inline(always)]
    pub fn bkvc2p(&self) -> Bkvc2pR {
        Bkvc2pR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:17 - desc ETRSEL"]
    #[inline(always)]
    pub fn etrsel(&self) -> EtrselR {
        EtrselR::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc BKINE"]
    #[inline(always)]
    pub fn bkine(&mut self) -> BkineW<'_, Af1Spec> {
        BkineW::new(self, 0)
    }
    #[doc = "Bit 1 - desc BKVC1E"]
    #[inline(always)]
    pub fn bkvc1e(&mut self) -> Bkvc1eW<'_, Af1Spec> {
        Bkvc1eW::new(self, 1)
    }
    #[doc = "Bit 2 - desc BKVC2E"]
    #[inline(always)]
    pub fn bkvc2e(&mut self) -> Bkvc2eW<'_, Af1Spec> {
        Bkvc2eW::new(self, 2)
    }
    #[doc = "Bit 9 - desc BKINP"]
    #[inline(always)]
    pub fn bkinp(&mut self) -> BkinpW<'_, Af1Spec> {
        BkinpW::new(self, 9)
    }
    #[doc = "Bit 10 - desc BKVC1P"]
    #[inline(always)]
    pub fn bkvc1p(&mut self) -> Bkvc1pW<'_, Af1Spec> {
        Bkvc1pW::new(self, 10)
    }
    #[doc = "Bit 11 - desc BKVC2P"]
    #[inline(always)]
    pub fn bkvc2p(&mut self) -> Bkvc2pW<'_, Af1Spec> {
        Bkvc2pW::new(self, 11)
    }
    #[doc = "Bits 14:17 - desc ETRSEL"]
    #[inline(always)]
    pub fn etrsel(&mut self) -> EtrselW<'_, Af1Spec> {
        EtrselW::new(self, 14)
    }
}
#[doc = "Alternate function Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Af1Spec;
impl crate::RegisterSpec for Af1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for Af1Spec {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for Af1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF1 to value 0"]
impl crate::Resettable for Af1Spec {}
