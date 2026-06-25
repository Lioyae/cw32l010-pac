#[doc = "Register `AHBRST` reader"]
pub type R = crate::R<AhbrstSpec>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::W<AhbrstSpec>;
#[doc = "Field `FLASH` reader - desc FLASH"]
pub type FlashR = crate::BitReader;
#[doc = "Field `FLASH` writer - desc FLASH"]
pub type FlashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - desc CRC"]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - desc CRC"]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOA` reader - desc GPIOA"]
pub type GpioaR = crate::BitReader;
#[doc = "Field `GPIOA` writer - desc GPIOA"]
pub type GpioaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - desc GPIOB"]
pub type GpiobR = crate::BitReader;
#[doc = "Field `GPIOB` writer - desc GPIOB"]
pub type GpiobW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc FLASH"]
    #[inline(always)]
    pub fn flash(&self) -> FlashR {
        FlashR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CRC"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc GPIOA"]
    #[inline(always)]
    pub fn gpioa(&self) -> GpioaR {
        GpioaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc GPIOB"]
    #[inline(always)]
    pub fn gpiob(&self) -> GpiobR {
        GpiobR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc FLASH"]
    #[inline(always)]
    pub fn flash(&mut self) -> FlashW<'_, AhbrstSpec> {
        FlashW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CRC"]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, AhbrstSpec> {
        CrcW::new(self, 2)
    }
    #[doc = "Bit 4 - desc GPIOA"]
    #[inline(always)]
    pub fn gpioa(&mut self) -> GpioaW<'_, AhbrstSpec> {
        GpioaW::new(self, 4)
    }
    #[doc = "Bit 5 - desc GPIOB"]
    #[inline(always)]
    pub fn gpiob(&mut self) -> GpiobW<'_, AhbrstSpec> {
        GpiobW::new(self, 5)
    }
}
#[doc = "AHB Reset Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstSpec;
impl crate::RegisterSpec for AhbrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst::R`](R) reader structure"]
impl crate::Readable for AhbrstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrst::W`](W) writer structure"]
impl crate::Writable for AhbrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::Resettable for AhbrstSpec {}
