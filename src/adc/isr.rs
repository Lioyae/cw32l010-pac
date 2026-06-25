#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `EOC` reader - desc EOC"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOS` reader - desc EOS"]
pub type EosR = crate::BitReader;
#[doc = "Field `AWDL` reader - desc AWDL"]
pub type AwdlR = crate::BitReader;
#[doc = "Field `AWDH` reader - desc AWDH"]
pub type AwdhR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EOS"]
    #[inline(always)]
    pub fn eos(&self) -> EosR {
        EosR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc AWDL"]
    #[inline(always)]
    pub fn awdl(&self) -> AwdlR {
        AwdlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc AWDH"]
    #[inline(always)]
    pub fn awdh(&self) -> AwdhR {
        AwdhR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "desc ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
