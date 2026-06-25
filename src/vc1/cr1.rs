#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `FLTTIME` reader - desc FLTTIME"]
pub type FlttimeR = crate::FieldReader;
#[doc = "Field `FLTTIME` writer - desc FLTTIME"]
pub type FlttimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLTCLK` reader - desc FLTCLK"]
pub type FltclkR = crate::BitReader;
#[doc = "Field `FLTCLK` writer - desc FLTCLK"]
pub type FltclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALLIE` reader - desc FALLIE"]
pub type FallieR = crate::BitReader;
#[doc = "Field `FALLIE` writer - desc FALLIE"]
pub type FallieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RISEIE` reader - desc RISEIE"]
pub type RiseieR = crate::BitReader;
#[doc = "Field `RISEIE` writer - desc RISEIE"]
pub type RiseieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHIE` reader - desc HIGHIE"]
pub type HighieR = crate::BitReader;
#[doc = "Field `HIGHIE` writer - desc HIGHIE"]
pub type HighieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKATCH1` reader - desc BLANKATCH1"]
pub type Blankatch1R = crate::BitReader;
#[doc = "Field `BLANKATCH1` writer - desc BLANKATCH1"]
pub type Blankatch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKATCH2` reader - desc BLANKATCH2"]
pub type Blankatch2R = crate::BitReader;
#[doc = "Field `BLANKATCH2` writer - desc BLANKATCH2"]
pub type Blankatch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKATCH3` reader - desc BLANKATCH3"]
pub type Blankatch3R = crate::BitReader;
#[doc = "Field `BLANKATCH3` writer - desc BLANKATCH3"]
pub type Blankatch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKATCH4` reader - desc BLANKATCH4"]
pub type Blankatch4R = crate::BitReader;
#[doc = "Field `BLANKATCH4` writer - desc BLANKATCH4"]
pub type Blankatch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKATCH5` reader - desc BLANKATCH5"]
pub type Blankatch5R = crate::BitReader;
#[doc = "Field `BLANKATCH5` writer - desc BLANKATCH5"]
pub type Blankatch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKATCH6` reader - desc BLANKATCH6"]
pub type Blankatch6R = crate::BitReader;
#[doc = "Field `BLANKATCH6` writer - desc BLANKATCH6"]
pub type Blankatch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKTIME` reader - desc BLANKTIME"]
pub type BlanktimeR = crate::FieldReader;
#[doc = "Field `BLANKTIME` writer - desc BLANKTIME"]
pub type BlanktimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BLANKLVL` reader - desc BLANKLVL"]
pub type BlanklvlR = crate::BitReader;
#[doc = "Field `BLANKLVL` writer - desc BLANKLVL"]
pub type BlanklvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - desc FLTTIME"]
    #[inline(always)]
    pub fn flttime(&self) -> FlttimeR {
        FlttimeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - desc FLTCLK"]
    #[inline(always)]
    pub fn fltclk(&self) -> FltclkR {
        FltclkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc FALLIE"]
    #[inline(always)]
    pub fn fallie(&self) -> FallieR {
        FallieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RISEIE"]
    #[inline(always)]
    pub fn riseie(&self) -> RiseieR {
        RiseieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HIGHIE"]
    #[inline(always)]
    pub fn highie(&self) -> HighieR {
        HighieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BLANKATCH1"]
    #[inline(always)]
    pub fn blankatch1(&self) -> Blankatch1R {
        Blankatch1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc BLANKATCH2"]
    #[inline(always)]
    pub fn blankatch2(&self) -> Blankatch2R {
        Blankatch2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BLANKATCH3"]
    #[inline(always)]
    pub fn blankatch3(&self) -> Blankatch3R {
        Blankatch3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc BLANKATCH4"]
    #[inline(always)]
    pub fn blankatch4(&self) -> Blankatch4R {
        Blankatch4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc BLANKATCH5"]
    #[inline(always)]
    pub fn blankatch5(&self) -> Blankatch5R {
        Blankatch5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc BLANKATCH6"]
    #[inline(always)]
    pub fn blankatch6(&self) -> Blankatch6R {
        Blankatch6R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - desc BLANKTIME"]
    #[inline(always)]
    pub fn blanktime(&self) -> BlanktimeR {
        BlanktimeR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - desc BLANKLVL"]
    #[inline(always)]
    pub fn blanklvl(&self) -> BlanklvlR {
        BlanklvlR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc FLTTIME"]
    #[inline(always)]
    pub fn flttime(&mut self) -> FlttimeW<'_, Cr1Spec> {
        FlttimeW::new(self, 0)
    }
    #[doc = "Bit 4 - desc FLTCLK"]
    #[inline(always)]
    pub fn fltclk(&mut self) -> FltclkW<'_, Cr1Spec> {
        FltclkW::new(self, 4)
    }
    #[doc = "Bit 5 - desc FALLIE"]
    #[inline(always)]
    pub fn fallie(&mut self) -> FallieW<'_, Cr1Spec> {
        FallieW::new(self, 5)
    }
    #[doc = "Bit 6 - desc RISEIE"]
    #[inline(always)]
    pub fn riseie(&mut self) -> RiseieW<'_, Cr1Spec> {
        RiseieW::new(self, 6)
    }
    #[doc = "Bit 7 - desc HIGHIE"]
    #[inline(always)]
    pub fn highie(&mut self) -> HighieW<'_, Cr1Spec> {
        HighieW::new(self, 7)
    }
    #[doc = "Bit 8 - desc BLANKATCH1"]
    #[inline(always)]
    pub fn blankatch1(&mut self) -> Blankatch1W<'_, Cr1Spec> {
        Blankatch1W::new(self, 8)
    }
    #[doc = "Bit 9 - desc BLANKATCH2"]
    #[inline(always)]
    pub fn blankatch2(&mut self) -> Blankatch2W<'_, Cr1Spec> {
        Blankatch2W::new(self, 9)
    }
    #[doc = "Bit 10 - desc BLANKATCH3"]
    #[inline(always)]
    pub fn blankatch3(&mut self) -> Blankatch3W<'_, Cr1Spec> {
        Blankatch3W::new(self, 10)
    }
    #[doc = "Bit 11 - desc BLANKATCH4"]
    #[inline(always)]
    pub fn blankatch4(&mut self) -> Blankatch4W<'_, Cr1Spec> {
        Blankatch4W::new(self, 11)
    }
    #[doc = "Bit 12 - desc BLANKATCH5"]
    #[inline(always)]
    pub fn blankatch5(&mut self) -> Blankatch5W<'_, Cr1Spec> {
        Blankatch5W::new(self, 12)
    }
    #[doc = "Bit 13 - desc BLANKATCH6"]
    #[inline(always)]
    pub fn blankatch6(&mut self) -> Blankatch6W<'_, Cr1Spec> {
        Blankatch6W::new(self, 13)
    }
    #[doc = "Bits 14:16 - desc BLANKTIME"]
    #[inline(always)]
    pub fn blanktime(&mut self) -> BlanktimeW<'_, Cr1Spec> {
        BlanktimeW::new(self, 14)
    }
    #[doc = "Bit 17 - desc BLANKLVL"]
    #[inline(always)]
    pub fn blanklvl(&mut self) -> BlanklvlW<'_, Cr1Spec> {
        BlanklvlW::new(self, 17)
    }
}
#[doc = "VC Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
