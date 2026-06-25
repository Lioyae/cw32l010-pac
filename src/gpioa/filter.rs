#[doc = "Register `FILTER` reader"]
pub type R = crate::R<FilterSpec>;
#[doc = "Register `FILTER` writer"]
pub type W = crate::W<FilterSpec>;
#[doc = "Field `PIN0` reader - desc PIN0"]
pub type Pin0R = crate::BitReader;
#[doc = "Field `PIN0` writer - desc PIN0"]
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN1` reader - desc PIN1"]
pub type Pin1R = crate::BitReader;
#[doc = "Field `PIN1` writer - desc PIN1"]
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN2` reader - desc PIN2"]
pub type Pin2R = crate::BitReader;
#[doc = "Field `PIN2` writer - desc PIN2"]
pub type Pin2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN3` reader - desc PIN3"]
pub type Pin3R = crate::BitReader;
#[doc = "Field `PIN3` writer - desc PIN3"]
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN4` reader - desc PIN4"]
pub type Pin4R = crate::BitReader;
#[doc = "Field `PIN4` writer - desc PIN4"]
pub type Pin4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN5` reader - desc PIN5"]
pub type Pin5R = crate::BitReader;
#[doc = "Field `PIN5` writer - desc PIN5"]
pub type Pin5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN6` reader - desc PIN6"]
pub type Pin6R = crate::BitReader;
#[doc = "Field `PIN6` writer - desc PIN6"]
pub type Pin6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN7` reader - desc PIN7"]
pub type Pin7R = crate::BitReader;
#[doc = "Field `PIN7` writer - desc PIN7"]
pub type Pin7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN8` reader - desc PIN8"]
pub type Pin8R = crate::BitReader;
#[doc = "Field `PIN8` writer - desc PIN8"]
pub type Pin8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTCLK` reader - desc FLTCLK"]
pub type FltclkR = crate::FieldReader;
#[doc = "Field `FLTCLK` writer - desc FLTCLK"]
pub type FltclkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - desc PIN0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PIN1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PIN2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PIN3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PIN4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PIN5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PIN6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PIN7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PIN8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:18 - desc FLTCLK"]
    #[inline(always)]
    pub fn fltclk(&self) -> FltclkR {
        FltclkR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc PIN0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<'_, FilterSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc PIN1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<'_, FilterSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc PIN2"]
    #[inline(always)]
    pub fn pin2(&mut self) -> Pin2W<'_, FilterSpec> {
        Pin2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc PIN3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<'_, FilterSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc PIN4"]
    #[inline(always)]
    pub fn pin4(&mut self) -> Pin4W<'_, FilterSpec> {
        Pin4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc PIN5"]
    #[inline(always)]
    pub fn pin5(&mut self) -> Pin5W<'_, FilterSpec> {
        Pin5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc PIN6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> Pin6W<'_, FilterSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc PIN7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> Pin7W<'_, FilterSpec> {
        Pin7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc PIN8"]
    #[inline(always)]
    pub fn pin8(&mut self) -> Pin8W<'_, FilterSpec> {
        Pin8W::new(self, 8)
    }
    #[doc = "Bits 16:18 - desc FLTCLK"]
    #[inline(always)]
    pub fn fltclk(&mut self) -> FltclkW<'_, FilterSpec> {
        FltclkW::new(self, 16)
    }
}
#[doc = "desc FILTER\n\nYou can [`read`](crate::Reg::read) this register and get [`filter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FilterSpec;
impl crate::RegisterSpec for FilterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter::R`](R) reader structure"]
impl crate::Readable for FilterSpec {}
#[doc = "`write(|w| ..)` method takes [`filter::W`](W) writer structure"]
impl crate::Writable for FilterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER to value 0"]
impl crate::Resettable for FilterSpec {}
