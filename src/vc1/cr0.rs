#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP` reader - desc RESP"]
pub type RespR = crate::BitReader;
#[doc = "Field `RESP` writer - desc RESP"]
pub type RespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYS` reader - desc HYS"]
pub type HysR = crate::BitReader;
#[doc = "Field `HYS` writer - desc HYS"]
pub type HysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - desc IE"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - desc IE"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - desc POL"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - desc POL"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINDOW` reader - desc WINDOW"]
pub type WindowR = crate::BitReader;
#[doc = "Field `WINDOW` writer - desc WINDOW"]
pub type WindowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INP` reader - desc PCHANNEL"]
pub type InpR = crate::FieldReader;
#[doc = "Field `INP` writer - desc PCHANNEL"]
pub type InpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INN` reader - desc NCHANNEL"]
pub type InnR = crate::FieldReader;
#[doc = "Field `INN` writer - desc NCHANNEL"]
pub type InnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RESP"]
    #[inline(always)]
    pub fn resp(&self) -> RespR {
        RespR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HYS"]
    #[inline(always)]
    pub fn hys(&self) -> HysR {
        HysR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc POL"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc WINDOW"]
    #[inline(always)]
    pub fn window(&self) -> WindowR {
        WindowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc PCHANNEL"]
    #[inline(always)]
    pub fn inp(&self) -> InpR {
        InpR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - desc NCHANNEL"]
    #[inline(always)]
    pub fn inn(&self) -> InnR {
        InnR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Cr0Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RESP"]
    #[inline(always)]
    pub fn resp(&mut self) -> RespW<'_, Cr0Spec> {
        RespW::new(self, 1)
    }
    #[doc = "Bit 2 - desc HYS"]
    #[inline(always)]
    pub fn hys(&mut self) -> HysW<'_, Cr0Spec> {
        HysW::new(self, 2)
    }
    #[doc = "Bit 3 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, Cr0Spec> {
        IeW::new(self, 3)
    }
    #[doc = "Bit 4 - desc POL"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, Cr0Spec> {
        PolW::new(self, 4)
    }
    #[doc = "Bit 5 - desc WINDOW"]
    #[inline(always)]
    pub fn window(&mut self) -> WindowW<'_, Cr0Spec> {
        WindowW::new(self, 5)
    }
    #[doc = "Bits 6:7 - desc PCHANNEL"]
    #[inline(always)]
    pub fn inp(&mut self) -> InpW<'_, Cr0Spec> {
        InpW::new(self, 6)
    }
    #[doc = "Bits 8:9 - desc NCHANNEL"]
    #[inline(always)]
    pub fn inn(&mut self) -> InnW<'_, Cr0Spec> {
        InnW::new(self, 8)
    }
}
#[doc = "VC Control register0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {}
