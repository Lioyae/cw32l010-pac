#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGREN` reader - desc BGREN"]
pub type BgrenR = crate::BitReader;
#[doc = "Field `BGREN` writer - desc BGREN"]
pub type BgrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - desc TSEN"]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - desc TSEN"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - desc CONT"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - desc CONT"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK` reader - desc CLKSRC"]
pub type ClkR = crate::FieldReader;
#[doc = "Field `CLK` writer - desc CLKSRC"]
pub type ClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENS` reader - desc ENS"]
pub type EnsR = crate::FieldReader;
#[doc = "Field `ENS` writer - desc ENS"]
pub type EnsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BGREN"]
    #[inline(always)]
    pub fn bgren(&self) -> BgrenR {
        BgrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TSEN"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CONT"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc CLKSRC"]
    #[inline(always)]
    pub fn clk(&self) -> ClkR {
        ClkR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:8 - desc ENS"]
    #[inline(always)]
    pub fn ens(&self) -> EnsR {
        EnsR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - desc BGREN"]
    #[inline(always)]
    pub fn bgren(&mut self) -> BgrenW<'_, CrSpec> {
        BgrenW::new(self, 1)
    }
    #[doc = "Bit 2 - desc TSEN"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TsenW<'_, CrSpec> {
        TsenW::new(self, 2)
    }
    #[doc = "Bit 3 - desc CONT"]
    #[inline(always)]
    pub fn cont(&mut self) -> ContW<'_, CrSpec> {
        ContW::new(self, 3)
    }
    #[doc = "Bits 4:5 - desc CLKSRC"]
    #[inline(always)]
    pub fn clk(&mut self) -> ClkW<'_, CrSpec> {
        ClkW::new(self, 4)
    }
    #[doc = "Bits 6:8 - desc ENS"]
    #[inline(always)]
    pub fn ens(&mut self) -> EnsW<'_, CrSpec> {
        EnsW::new(self, 6)
    }
}
#[doc = "desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
