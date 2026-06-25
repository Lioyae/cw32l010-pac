#[doc = "Register `DEBUG` reader"]
pub type R = crate::R<DebugSpec>;
#[doc = "Register `DEBUG` writer"]
pub type W = crate::W<DebugSpec>;
#[doc = "Field `ATIM` reader - desc ATIM"]
pub type AtimR = crate::BitReader;
#[doc = "Field `ATIM` writer - desc ATIM"]
pub type AtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM1` reader - desc GTIM1"]
pub type Gtim1R = crate::BitReader;
#[doc = "Field `GTIM1` writer - desc GTIM1"]
pub type Gtim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTIM123` reader - desc BTIM123"]
pub type Btim123R = crate::BitReader;
#[doc = "Field `BTIM123` writer - desc BTIM123"]
pub type Btim123W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM` reader - desc LPTIM"]
pub type LptimR = crate::BitReader;
#[doc = "Field `LPTIM` writer - desc LPTIM"]
pub type LptimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - desc RTC"]
pub type RtcR = crate::BitReader;
#[doc = "Field `RTC` writer - desc RTC"]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDT` reader - desc IWDT"]
pub type IwdtR = crate::BitReader;
#[doc = "Field `IWDT` writer - desc IWDT"]
pub type IwdtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&self) -> AtimR {
        AtimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc GTIM1"]
    #[inline(always)]
    pub fn gtim1(&self) -> Gtim1R {
        Gtim1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BTIM123"]
    #[inline(always)]
    pub fn btim123(&self) -> Btim123R {
        Btim123R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc LPTIM"]
    #[inline(always)]
    pub fn lptim(&self) -> LptimR {
        LptimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&self) -> IwdtR {
        IwdtR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&mut self) -> AtimW<'_, DebugSpec> {
        AtimW::new(self, 0)
    }
    #[doc = "Bit 1 - desc GTIM1"]
    #[inline(always)]
    pub fn gtim1(&mut self) -> Gtim1W<'_, DebugSpec> {
        Gtim1W::new(self, 1)
    }
    #[doc = "Bit 5 - desc BTIM123"]
    #[inline(always)]
    pub fn btim123(&mut self) -> Btim123W<'_, DebugSpec> {
        Btim123W::new(self, 5)
    }
    #[doc = "Bit 6 - desc LPTIM"]
    #[inline(always)]
    pub fn lptim(&mut self) -> LptimW<'_, DebugSpec> {
        LptimW::new(self, 6)
    }
    #[doc = "Bit 8 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, DebugSpec> {
        RtcW::new(self, 8)
    }
    #[doc = "Bit 9 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&mut self) -> IwdtW<'_, DebugSpec> {
        IwdtW::new(self, 9)
    }
}
#[doc = "Debug Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugSpec;
impl crate::RegisterSpec for DebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug::R`](R) reader structure"]
impl crate::Readable for DebugSpec {}
#[doc = "`write(|w| ..)` method takes [`debug::W`](W) writer structure"]
impl crate::Writable for DebugSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG to value 0"]
impl crate::Resettable for DebugSpec {}
