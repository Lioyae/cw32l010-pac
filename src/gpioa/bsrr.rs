#[doc = "Register `BSRR` reader"]
pub type R = crate::R<BsrrSpec>;
#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BsrrSpec>;
#[doc = "Field `BSS0` reader - desc BSS0"]
pub type Bss0R = crate::BitReader;
#[doc = "Field `BSS0` writer - desc BSS0"]
pub type Bss0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS1` reader - desc BSS1"]
pub type Bss1R = crate::BitReader;
#[doc = "Field `BSS1` writer - desc BSS1"]
pub type Bss1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS2` reader - desc BSS2"]
pub type Bss2R = crate::BitReader;
#[doc = "Field `BSS2` writer - desc BSS2"]
pub type Bss2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS3` reader - desc BSS3"]
pub type Bss3R = crate::BitReader;
#[doc = "Field `BSS3` writer - desc BSS3"]
pub type Bss3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS4` reader - desc BSS4"]
pub type Bss4R = crate::BitReader;
#[doc = "Field `BSS4` writer - desc BSS4"]
pub type Bss4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS5` reader - desc BSS5"]
pub type Bss5R = crate::BitReader;
#[doc = "Field `BSS5` writer - desc BSS5"]
pub type Bss5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS6` reader - desc BSS6"]
pub type Bss6R = crate::BitReader;
#[doc = "Field `BSS6` writer - desc BSS6"]
pub type Bss6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS7` reader - desc BSS7"]
pub type Bss7R = crate::BitReader;
#[doc = "Field `BSS7` writer - desc BSS7"]
pub type Bss7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS8` reader - desc BSS8"]
pub type Bss8R = crate::BitReader;
#[doc = "Field `BSS8` writer - desc BSS8"]
pub type Bss8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR0` reader - desc BRR0"]
pub type Brr0R = crate::BitReader;
#[doc = "Field `BRR0` writer - desc BRR0"]
pub type Brr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR1` reader - desc BRR1"]
pub type Brr1R = crate::BitReader;
#[doc = "Field `BRR1` writer - desc BRR1"]
pub type Brr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR2` reader - desc BRR2"]
pub type Brr2R = crate::BitReader;
#[doc = "Field `BRR2` writer - desc BRR2"]
pub type Brr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR3` reader - desc BRR3"]
pub type Brr3R = crate::BitReader;
#[doc = "Field `BRR3` writer - desc BRR3"]
pub type Brr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR4` reader - desc BRR4"]
pub type Brr4R = crate::BitReader;
#[doc = "Field `BRR4` writer - desc BRR4"]
pub type Brr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR5` reader - desc BRR5"]
pub type Brr5R = crate::BitReader;
#[doc = "Field `BRR5` writer - desc BRR5"]
pub type Brr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR6` reader - desc BRR6"]
pub type Brr6R = crate::BitReader;
#[doc = "Field `BRR6` writer - desc BRR6"]
pub type Brr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR7` reader - desc BRR7"]
pub type Brr7R = crate::BitReader;
#[doc = "Field `BRR7` writer - desc BRR7"]
pub type Brr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR8` reader - desc BRR8"]
pub type Brr8R = crate::BitReader;
#[doc = "Field `BRR8` writer - desc BRR8"]
pub type Brr8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc BSS0"]
    #[inline(always)]
    pub fn bss0(&self) -> Bss0R {
        Bss0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BSS1"]
    #[inline(always)]
    pub fn bss1(&self) -> Bss1R {
        Bss1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BSS2"]
    #[inline(always)]
    pub fn bss2(&self) -> Bss2R {
        Bss2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc BSS3"]
    #[inline(always)]
    pub fn bss3(&self) -> Bss3R {
        Bss3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc BSS4"]
    #[inline(always)]
    pub fn bss4(&self) -> Bss4R {
        Bss4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BSS5"]
    #[inline(always)]
    pub fn bss5(&self) -> Bss5R {
        Bss5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc BSS6"]
    #[inline(always)]
    pub fn bss6(&self) -> Bss6R {
        Bss6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BSS7"]
    #[inline(always)]
    pub fn bss7(&self) -> Bss7R {
        Bss7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BSS8"]
    #[inline(always)]
    pub fn bss8(&self) -> Bss8R {
        Bss8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - desc BRR0"]
    #[inline(always)]
    pub fn brr0(&self) -> Brr0R {
        Brr0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc BRR1"]
    #[inline(always)]
    pub fn brr1(&self) -> Brr1R {
        Brr1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc BRR2"]
    #[inline(always)]
    pub fn brr2(&self) -> Brr2R {
        Brr2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc BRR3"]
    #[inline(always)]
    pub fn brr3(&self) -> Brr3R {
        Brr3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc BRR4"]
    #[inline(always)]
    pub fn brr4(&self) -> Brr4R {
        Brr4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc BRR5"]
    #[inline(always)]
    pub fn brr5(&self) -> Brr5R {
        Brr5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc BRR6"]
    #[inline(always)]
    pub fn brr6(&self) -> Brr6R {
        Brr6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc BRR7"]
    #[inline(always)]
    pub fn brr7(&self) -> Brr7R {
        Brr7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc BRR8"]
    #[inline(always)]
    pub fn brr8(&self) -> Brr8R {
        Brr8R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BSS0"]
    #[inline(always)]
    pub fn bss0(&mut self) -> Bss0W<'_, BsrrSpec> {
        Bss0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc BSS1"]
    #[inline(always)]
    pub fn bss1(&mut self) -> Bss1W<'_, BsrrSpec> {
        Bss1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc BSS2"]
    #[inline(always)]
    pub fn bss2(&mut self) -> Bss2W<'_, BsrrSpec> {
        Bss2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc BSS3"]
    #[inline(always)]
    pub fn bss3(&mut self) -> Bss3W<'_, BsrrSpec> {
        Bss3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc BSS4"]
    #[inline(always)]
    pub fn bss4(&mut self) -> Bss4W<'_, BsrrSpec> {
        Bss4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc BSS5"]
    #[inline(always)]
    pub fn bss5(&mut self) -> Bss5W<'_, BsrrSpec> {
        Bss5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc BSS6"]
    #[inline(always)]
    pub fn bss6(&mut self) -> Bss6W<'_, BsrrSpec> {
        Bss6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc BSS7"]
    #[inline(always)]
    pub fn bss7(&mut self) -> Bss7W<'_, BsrrSpec> {
        Bss7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc BSS8"]
    #[inline(always)]
    pub fn bss8(&mut self) -> Bss8W<'_, BsrrSpec> {
        Bss8W::new(self, 8)
    }
    #[doc = "Bit 16 - desc BRR0"]
    #[inline(always)]
    pub fn brr0(&mut self) -> Brr0W<'_, BsrrSpec> {
        Brr0W::new(self, 16)
    }
    #[doc = "Bit 17 - desc BRR1"]
    #[inline(always)]
    pub fn brr1(&mut self) -> Brr1W<'_, BsrrSpec> {
        Brr1W::new(self, 17)
    }
    #[doc = "Bit 18 - desc BRR2"]
    #[inline(always)]
    pub fn brr2(&mut self) -> Brr2W<'_, BsrrSpec> {
        Brr2W::new(self, 18)
    }
    #[doc = "Bit 19 - desc BRR3"]
    #[inline(always)]
    pub fn brr3(&mut self) -> Brr3W<'_, BsrrSpec> {
        Brr3W::new(self, 19)
    }
    #[doc = "Bit 20 - desc BRR4"]
    #[inline(always)]
    pub fn brr4(&mut self) -> Brr4W<'_, BsrrSpec> {
        Brr4W::new(self, 20)
    }
    #[doc = "Bit 21 - desc BRR5"]
    #[inline(always)]
    pub fn brr5(&mut self) -> Brr5W<'_, BsrrSpec> {
        Brr5W::new(self, 21)
    }
    #[doc = "Bit 22 - desc BRR6"]
    #[inline(always)]
    pub fn brr6(&mut self) -> Brr6W<'_, BsrrSpec> {
        Brr6W::new(self, 22)
    }
    #[doc = "Bit 23 - desc BRR7"]
    #[inline(always)]
    pub fn brr7(&mut self) -> Brr7W<'_, BsrrSpec> {
        Brr7W::new(self, 23)
    }
    #[doc = "Bit 24 - desc BRR8"]
    #[inline(always)]
    pub fn brr8(&mut self) -> Brr8W<'_, BsrrSpec> {
        Brr8W::new(self, 24)
    }
}
#[doc = "desc BSRR\n\nYou can [`read`](crate::Reg::read) this register and get [`bsrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsrrSpec;
impl crate::RegisterSpec for BsrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsrr::R`](R) reader structure"]
impl crate::Readable for BsrrSpec {}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BsrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BsrrSpec {}
