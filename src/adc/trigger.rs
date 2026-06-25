#[doc = "Register `TRIGGER` reader"]
pub type R = crate::R<TriggerSpec>;
#[doc = "Register `TRIGGER` writer"]
pub type W = crate::W<TriggerSpec>;
#[doc = "Field `ATIMTRGO` reader - desc ATIMTRGO"]
pub type AtimtrgoR = crate::BitReader;
#[doc = "Field `ATIMTRGO` writer - desc ATIMTRGO"]
pub type AtimtrgoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIMTRGO2` reader - desc ATIMTRGO2"]
pub type Atimtrgo2R = crate::BitReader;
#[doc = "Field `ATIMTRGO2` writer - desc ATIMTRGO2"]
pub type Atimtrgo2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIMCC1` reader - desc ATIMCC1"]
pub type Atimcc1R = crate::BitReader;
#[doc = "Field `ATIMCC1` writer - desc ATIMCC1"]
pub type Atimcc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIMCC2` reader - desc ATIMCC2"]
pub type Atimcc2R = crate::BitReader;
#[doc = "Field `ATIMCC2` writer - desc ATIMCC2"]
pub type Atimcc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIMCC3` reader - desc ATIMCC3"]
pub type Atimcc3R = crate::BitReader;
#[doc = "Field `ATIMCC3` writer - desc ATIMCC3"]
pub type Atimcc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIMCC4` reader - desc ATIMCC4"]
pub type Atimcc4R = crate::BitReader;
#[doc = "Field `ATIMCC4` writer - desc ATIMCC4"]
pub type Atimcc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIMCC5` reader - desc ATIMCC5"]
pub type Atimcc5R = crate::BitReader;
#[doc = "Field `ATIMCC5` writer - desc ATIMCC5"]
pub type Atimcc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIMCC6` reader - desc ATIMCC6"]
pub type Atimcc6R = crate::BitReader;
#[doc = "Field `ATIMCC6` writer - desc ATIMCC6"]
pub type Atimcc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM1TRGO` reader - desc GTIM1TRGO"]
pub type Gtim1trgoR = crate::BitReader;
#[doc = "Field `GTIM1TRGO` writer - desc GTIM1TRGO"]
pub type Gtim1trgoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM1CC1` reader - desc GTIM1CC1"]
pub type Gtim1cc1R = crate::BitReader;
#[doc = "Field `GTIM1CC1` writer - desc GTIM1CC1"]
pub type Gtim1cc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM1CC2` reader - desc GTIM1CC2"]
pub type Gtim1cc2R = crate::BitReader;
#[doc = "Field `GTIM1CC2` writer - desc GTIM1CC2"]
pub type Gtim1cc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM1CC3` reader - desc GTIM1CC3"]
pub type Gtim1cc3R = crate::BitReader;
#[doc = "Field `GTIM1CC3` writer - desc GTIM1CC3"]
pub type Gtim1cc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM1CC4` reader - desc GTIM1CC4"]
pub type Gtim1cc4R = crate::BitReader;
#[doc = "Field `GTIM1CC4` writer - desc GTIM1CC4"]
pub type Gtim1cc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTIM1TRGO` reader - desc BTIM1TRGO"]
pub type Btim1trgoR = crate::BitReader;
#[doc = "Field `BTIM1TRGO` writer - desc BTIM1TRGO"]
pub type Btim1trgoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTIM2TRGO` reader - desc BTIM2TRGO"]
pub type Btim2trgoR = crate::BitReader;
#[doc = "Field `BTIM2TRGO` writer - desc BTIM2TRGO"]
pub type Btim2trgoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTIM3TRGO` reader - desc BTIM3TRGO"]
pub type Btim3trgoR = crate::BitReader;
#[doc = "Field `BTIM3TRGO` writer - desc BTIM3TRGO"]
pub type Btim3trgoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - desc SPI1"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - desc SPI1"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - desc UART1"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - desc UART1"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2` reader - desc UART2"]
pub type Uart2R = crate::BitReader;
#[doc = "Field `UART2` writer - desc UART2"]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc ATIMTRGO"]
    #[inline(always)]
    pub fn atimtrgo(&self) -> AtimtrgoR {
        AtimtrgoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ATIMTRGO2"]
    #[inline(always)]
    pub fn atimtrgo2(&self) -> Atimtrgo2R {
        Atimtrgo2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ATIMCC1"]
    #[inline(always)]
    pub fn atimcc1(&self) -> Atimcc1R {
        Atimcc1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ATIMCC2"]
    #[inline(always)]
    pub fn atimcc2(&self) -> Atimcc2R {
        Atimcc2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ATIMCC3"]
    #[inline(always)]
    pub fn atimcc3(&self) -> Atimcc3R {
        Atimcc3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ATIMCC4"]
    #[inline(always)]
    pub fn atimcc4(&self) -> Atimcc4R {
        Atimcc4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ATIMCC5"]
    #[inline(always)]
    pub fn atimcc5(&self) -> Atimcc5R {
        Atimcc5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ATIMCC6"]
    #[inline(always)]
    pub fn atimcc6(&self) -> Atimcc6R {
        Atimcc6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc GTIM1TRGO"]
    #[inline(always)]
    pub fn gtim1trgo(&self) -> Gtim1trgoR {
        Gtim1trgoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc GTIM1CC1"]
    #[inline(always)]
    pub fn gtim1cc1(&self) -> Gtim1cc1R {
        Gtim1cc1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc GTIM1CC2"]
    #[inline(always)]
    pub fn gtim1cc2(&self) -> Gtim1cc2R {
        Gtim1cc2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc GTIM1CC3"]
    #[inline(always)]
    pub fn gtim1cc3(&self) -> Gtim1cc3R {
        Gtim1cc3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc GTIM1CC4"]
    #[inline(always)]
    pub fn gtim1cc4(&self) -> Gtim1cc4R {
        Gtim1cc4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc BTIM1TRGO"]
    #[inline(always)]
    pub fn btim1trgo(&self) -> Btim1trgoR {
        Btim1trgoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc BTIM2TRGO"]
    #[inline(always)]
    pub fn btim2trgo(&self) -> Btim2trgoR {
        Btim2trgoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc BTIM3TRGO"]
    #[inline(always)]
    pub fn btim3trgo(&self) -> Btim3trgoR {
        Btim3trgoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ATIMTRGO"]
    #[inline(always)]
    pub fn atimtrgo(&mut self) -> AtimtrgoW<'_, TriggerSpec> {
        AtimtrgoW::new(self, 0)
    }
    #[doc = "Bit 1 - desc ATIMTRGO2"]
    #[inline(always)]
    pub fn atimtrgo2(&mut self) -> Atimtrgo2W<'_, TriggerSpec> {
        Atimtrgo2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc ATIMCC1"]
    #[inline(always)]
    pub fn atimcc1(&mut self) -> Atimcc1W<'_, TriggerSpec> {
        Atimcc1W::new(self, 2)
    }
    #[doc = "Bit 3 - desc ATIMCC2"]
    #[inline(always)]
    pub fn atimcc2(&mut self) -> Atimcc2W<'_, TriggerSpec> {
        Atimcc2W::new(self, 3)
    }
    #[doc = "Bit 4 - desc ATIMCC3"]
    #[inline(always)]
    pub fn atimcc3(&mut self) -> Atimcc3W<'_, TriggerSpec> {
        Atimcc3W::new(self, 4)
    }
    #[doc = "Bit 5 - desc ATIMCC4"]
    #[inline(always)]
    pub fn atimcc4(&mut self) -> Atimcc4W<'_, TriggerSpec> {
        Atimcc4W::new(self, 5)
    }
    #[doc = "Bit 6 - desc ATIMCC5"]
    #[inline(always)]
    pub fn atimcc5(&mut self) -> Atimcc5W<'_, TriggerSpec> {
        Atimcc5W::new(self, 6)
    }
    #[doc = "Bit 7 - desc ATIMCC6"]
    #[inline(always)]
    pub fn atimcc6(&mut self) -> Atimcc6W<'_, TriggerSpec> {
        Atimcc6W::new(self, 7)
    }
    #[doc = "Bit 8 - desc GTIM1TRGO"]
    #[inline(always)]
    pub fn gtim1trgo(&mut self) -> Gtim1trgoW<'_, TriggerSpec> {
        Gtim1trgoW::new(self, 8)
    }
    #[doc = "Bit 9 - desc GTIM1CC1"]
    #[inline(always)]
    pub fn gtim1cc1(&mut self) -> Gtim1cc1W<'_, TriggerSpec> {
        Gtim1cc1W::new(self, 9)
    }
    #[doc = "Bit 10 - desc GTIM1CC2"]
    #[inline(always)]
    pub fn gtim1cc2(&mut self) -> Gtim1cc2W<'_, TriggerSpec> {
        Gtim1cc2W::new(self, 10)
    }
    #[doc = "Bit 11 - desc GTIM1CC3"]
    #[inline(always)]
    pub fn gtim1cc3(&mut self) -> Gtim1cc3W<'_, TriggerSpec> {
        Gtim1cc3W::new(self, 11)
    }
    #[doc = "Bit 12 - desc GTIM1CC4"]
    #[inline(always)]
    pub fn gtim1cc4(&mut self) -> Gtim1cc4W<'_, TriggerSpec> {
        Gtim1cc4W::new(self, 12)
    }
    #[doc = "Bit 13 - desc BTIM1TRGO"]
    #[inline(always)]
    pub fn btim1trgo(&mut self) -> Btim1trgoW<'_, TriggerSpec> {
        Btim1trgoW::new(self, 13)
    }
    #[doc = "Bit 14 - desc BTIM2TRGO"]
    #[inline(always)]
    pub fn btim2trgo(&mut self) -> Btim2trgoW<'_, TriggerSpec> {
        Btim2trgoW::new(self, 14)
    }
    #[doc = "Bit 15 - desc BTIM3TRGO"]
    #[inline(always)]
    pub fn btim3trgo(&mut self) -> Btim3trgoW<'_, TriggerSpec> {
        Btim3trgoW::new(self, 15)
    }
    #[doc = "Bit 16 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<'_, TriggerSpec> {
        Spi1W::new(self, 16)
    }
    #[doc = "Bit 17 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<'_, TriggerSpec> {
        Uart1W::new(self, 17)
    }
    #[doc = "Bit 18 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<'_, TriggerSpec> {
        Uart2W::new(self, 18)
    }
}
#[doc = "desc TRIGGER\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriggerSpec;
impl crate::RegisterSpec for TriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger::R`](R) reader structure"]
impl crate::Readable for TriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`trigger::W`](W) writer structure"]
impl crate::Writable for TriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TriggerSpec {}
