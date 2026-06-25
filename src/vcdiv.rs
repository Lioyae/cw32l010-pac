#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    div: Div,
}
impl RegisterBlock {
    #[doc = "0x00 - VCREF Control register"]
    #[inline(always)]
    pub const fn div(&self) -> &Div {
        &self.div
    }
}
#[doc = "DIV (rw) register accessor: VCREF Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`] module"]
#[doc(alias = "DIV")]
pub type Div = crate::Reg<div::DivSpec>;
#[doc = "VCREF Control register"]
pub mod div;
