#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AfrhSpec>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AfrhSpec>;
#[doc = "Field `AFR8` reader - desc AFR8"]
pub type Afr8R = crate::FieldReader;
#[doc = "Field `AFR8` writer - desc AFR8"]
pub type Afr8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc AFR8"]
    #[inline(always)]
    pub fn afr8(&self) -> Afr8R {
        Afr8R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc AFR8"]
    #[inline(always)]
    pub fn afr8(&mut self) -> Afr8W<'_, AfrhSpec> {
        Afr8W::new(self, 0)
    }
}
#[doc = "desc AFRH\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrhSpec;
impl crate::RegisterSpec for AfrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AfrhSpec {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AfrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AfrhSpec {}
