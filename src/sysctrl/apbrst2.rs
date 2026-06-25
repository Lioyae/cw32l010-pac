#[doc = "Register `APBRST2` reader"]
pub type R = crate::R<Apbrst2Spec>;
#[doc = "Register `APBRST2` writer"]
pub type W = crate::W<Apbrst2Spec>;
#[doc = "Field `RTC` reader - desc RTC"]
pub type RtcR = crate::BitReader;
#[doc = "Field `RTC` writer - desc RTC"]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTIM123` reader - desc BTIM123"]
pub type Btim123R = crate::BitReader;
#[doc = "Field `BTIM123` writer - desc BTIM123"]
pub type Btim123W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDT` reader - desc IWDT"]
pub type IwdtR = crate::BitReader;
#[doc = "Field `IWDT` writer - desc IWDT"]
pub type IwdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - desc I2C1"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - desc I2C1"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM` reader - desc LPTIM"]
pub type LptimR = crate::BitReader;
#[doc = "Field `LPTIM` writer - desc LPTIM"]
pub type LptimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BTIM123"]
    #[inline(always)]
    pub fn btim123(&self) -> Btim123R {
        Btim123R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&self) -> IwdtR {
        IwdtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc LPTIM"]
    #[inline(always)]
    pub fn lptim(&self) -> LptimR {
        LptimR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, Apbrst2Spec> {
        RtcW::new(self, 1)
    }
    #[doc = "Bit 2 - desc BTIM123"]
    #[inline(always)]
    pub fn btim123(&mut self) -> Btim123W<'_, Apbrst2Spec> {
        Btim123W::new(self, 2)
    }
    #[doc = "Bit 4 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&mut self) -> IwdtW<'_, Apbrst2Spec> {
        IwdtW::new(self, 4)
    }
    #[doc = "Bit 6 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<'_, Apbrst2Spec> {
        I2c1W::new(self, 6)
    }
    #[doc = "Bit 7 - desc LPTIM"]
    #[inline(always)]
    pub fn lptim(&mut self) -> LptimW<'_, Apbrst2Spec> {
        LptimW::new(self, 7)
    }
}
#[doc = "APB Reset Control Reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrst2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrst2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbrst2Spec;
impl crate::RegisterSpec for Apbrst2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrst2::R`](R) reader structure"]
impl crate::Readable for Apbrst2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbrst2::W`](W) writer structure"]
impl crate::Writable for Apbrst2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBRST2 to value 0"]
impl crate::Resettable for Apbrst2Spec {}
