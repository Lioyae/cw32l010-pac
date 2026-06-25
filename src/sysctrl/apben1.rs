#[doc = "Register `APBEN1` reader"]
pub type R = crate::R<Apben1Spec>;
#[doc = "Register `APBEN1` writer"]
pub type W = crate::W<Apben1Spec>;
#[doc = "Field `ADC` reader - desc ADC"]
pub type AdcR = crate::BitReader;
#[doc = "Field `ADC` writer - desc ADC"]
pub type AdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC` reader - desc VC"]
pub type VcR = crate::BitReader;
#[doc = "Field `VC` writer - desc VC"]
pub type VcW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `ATIM` reader - desc ATIM"]
pub type AtimR = crate::BitReader;
#[doc = "Field `ATIM` writer - desc ATIM"]
pub type AtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM1` reader - desc GTIM1"]
pub type Gtim1R = crate::BitReader;
#[doc = "Field `GTIM1` writer - desc GTIM1"]
pub type Gtim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - desc ADC"]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc VC"]
    #[inline(always)]
    pub fn vc(&self) -> VcR {
        VcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&self) -> AtimR {
        AtimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc GTIM1"]
    #[inline(always)]
    pub fn gtim1(&self) -> Gtim1R {
        Gtim1R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ADC"]
    #[inline(always)]
    pub fn adc(&mut self) -> AdcW<'_, Apben1Spec> {
        AdcW::new(self, 0)
    }
    #[doc = "Bit 1 - desc VC"]
    #[inline(always)]
    pub fn vc(&mut self) -> VcW<'_, Apben1Spec> {
        VcW::new(self, 1)
    }
    #[doc = "Bit 2 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<'_, Apben1Spec> {
        Spi1W::new(self, 2)
    }
    #[doc = "Bit 3 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<'_, Apben1Spec> {
        Uart1W::new(self, 3)
    }
    #[doc = "Bit 4 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<'_, Apben1Spec> {
        Uart2W::new(self, 4)
    }
    #[doc = "Bit 5 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&mut self) -> AtimW<'_, Apben1Spec> {
        AtimW::new(self, 5)
    }
    #[doc = "Bit 6 - desc GTIM1"]
    #[inline(always)]
    pub fn gtim1(&mut self) -> Gtim1W<'_, Apben1Spec> {
        Gtim1W::new(self, 6)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Apben1Spec> {
        KeyW::new(self, 16)
    }
}
#[doc = "APB Clock Control Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`apben1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apben1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apben1Spec;
impl crate::RegisterSpec for Apben1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apben1::R`](R) reader structure"]
impl crate::Readable for Apben1Spec {}
#[doc = "`write(|w| ..)` method takes [`apben1::W`](W) writer structure"]
impl crate::Writable for Apben1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBEN1 to value 0"]
impl crate::Resettable for Apben1Spec {}
