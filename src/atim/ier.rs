#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `UIE` reader - desc UIE"]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - desc UIE"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IE` reader - desc CC1IE"]
pub type Cc1ieR = crate::BitReader;
#[doc = "Field `CC1IE` writer - desc CC1IE"]
pub type Cc1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IE` reader - desc CC2IE"]
pub type Cc2ieR = crate::BitReader;
#[doc = "Field `CC2IE` writer - desc CC2IE"]
pub type Cc2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IE` reader - desc CC3IE"]
pub type Cc3ieR = crate::BitReader;
#[doc = "Field `CC3IE` writer - desc CC3IE"]
pub type Cc3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IE` reader - desc CC4IE"]
pub type Cc4ieR = crate::BitReader;
#[doc = "Field `CC4IE` writer - desc CC4IE"]
pub type Cc4ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIE` reader - desc COMIE"]
pub type ComieR = crate::BitReader;
#[doc = "Field `COMIE` writer - desc COMIE"]
pub type ComieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - desc TIE"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - desc TIE"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIE` reader - desc BIE"]
pub type BieR = crate::BitReader;
#[doc = "Field `BIE` writer - desc BIE"]
pub type BieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5IE` reader - desc CC5IE"]
pub type Cc5ieR = crate::BitReader;
#[doc = "Field `CC5IE` writer - desc CC5IE"]
pub type Cc5ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6IE` reader - desc CC6IE"]
pub type Cc6ieR = crate::BitReader;
#[doc = "Field `CC6IE` writer - desc CC6IE"]
pub type Cc6ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDXIE` reader - desc IDXIE"]
pub type IdxieR = crate::BitReader;
#[doc = "Field `IDXIE` writer - desc IDXIE"]
pub type IdxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRIE` reader - desc DIRIE"]
pub type DirieR = crate::BitReader;
#[doc = "Field `DIRIE` writer - desc DIRIE"]
pub type DirieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IERRIE` reader - desc IERRIE"]
pub type IerrieR = crate::BitReader;
#[doc = "Field `IERRIE` writer - desc IERRIE"]
pub type IerrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRIE` reader - desc TERRIE"]
pub type TerrieR = crate::BitReader;
#[doc = "Field `TERRIE` writer - desc TERRIE"]
pub type TerrieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CC1IE"]
    #[inline(always)]
    pub fn cc1ie(&self) -> Cc1ieR {
        Cc1ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CC2IE"]
    #[inline(always)]
    pub fn cc2ie(&self) -> Cc2ieR {
        Cc2ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CC3IE"]
    #[inline(always)]
    pub fn cc3ie(&self) -> Cc3ieR {
        Cc3ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CC4IE"]
    #[inline(always)]
    pub fn cc4ie(&self) -> Cc4ieR {
        Cc4ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc COMIE"]
    #[inline(always)]
    pub fn comie(&self) -> ComieR {
        ComieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BIE"]
    #[inline(always)]
    pub fn bie(&self) -> BieR {
        BieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - desc CC5IE"]
    #[inline(always)]
    pub fn cc5ie(&self) -> Cc5ieR {
        Cc5ieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc CC6IE"]
    #[inline(always)]
    pub fn cc6ie(&self) -> Cc6ieR {
        Cc6ieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - desc IDXIE"]
    #[inline(always)]
    pub fn idxie(&self) -> IdxieR {
        IdxieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc DIRIE"]
    #[inline(always)]
    pub fn dirie(&self) -> DirieR {
        DirieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc IERRIE"]
    #[inline(always)]
    pub fn ierrie(&self) -> IerrieR {
        IerrieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc TERRIE"]
    #[inline(always)]
    pub fn terrie(&self) -> TerrieR {
        TerrieR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UIE"]
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<'_, IerSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CC1IE"]
    #[inline(always)]
    pub fn cc1ie(&mut self) -> Cc1ieW<'_, IerSpec> {
        Cc1ieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CC2IE"]
    #[inline(always)]
    pub fn cc2ie(&mut self) -> Cc2ieW<'_, IerSpec> {
        Cc2ieW::new(self, 2)
    }
    #[doc = "Bit 3 - desc CC3IE"]
    #[inline(always)]
    pub fn cc3ie(&mut self) -> Cc3ieW<'_, IerSpec> {
        Cc3ieW::new(self, 3)
    }
    #[doc = "Bit 4 - desc CC4IE"]
    #[inline(always)]
    pub fn cc4ie(&mut self) -> Cc4ieW<'_, IerSpec> {
        Cc4ieW::new(self, 4)
    }
    #[doc = "Bit 5 - desc COMIE"]
    #[inline(always)]
    pub fn comie(&mut self) -> ComieW<'_, IerSpec> {
        ComieW::new(self, 5)
    }
    #[doc = "Bit 6 - desc TIE"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, IerSpec> {
        TieW::new(self, 6)
    }
    #[doc = "Bit 7 - desc BIE"]
    #[inline(always)]
    pub fn bie(&mut self) -> BieW<'_, IerSpec> {
        BieW::new(self, 7)
    }
    #[doc = "Bit 16 - desc CC5IE"]
    #[inline(always)]
    pub fn cc5ie(&mut self) -> Cc5ieW<'_, IerSpec> {
        Cc5ieW::new(self, 16)
    }
    #[doc = "Bit 17 - desc CC6IE"]
    #[inline(always)]
    pub fn cc6ie(&mut self) -> Cc6ieW<'_, IerSpec> {
        Cc6ieW::new(self, 17)
    }
    #[doc = "Bit 20 - desc IDXIE"]
    #[inline(always)]
    pub fn idxie(&mut self) -> IdxieW<'_, IerSpec> {
        IdxieW::new(self, 20)
    }
    #[doc = "Bit 21 - desc DIRIE"]
    #[inline(always)]
    pub fn dirie(&mut self) -> DirieW<'_, IerSpec> {
        DirieW::new(self, 21)
    }
    #[doc = "Bit 22 - desc IERRIE"]
    #[inline(always)]
    pub fn ierrie(&mut self) -> IerrieW<'_, IerSpec> {
        IerrieW::new(self, 22)
    }
    #[doc = "Bit 23 - desc TERRIE"]
    #[inline(always)]
    pub fn terrie(&mut self) -> TerrieW<'_, IerSpec> {
        TerrieW::new(self, 23)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
