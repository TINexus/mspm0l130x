# [doc = "Register `MIS` reader"] pub type R = crate :: R < MIS_SPEC > ; # [doc = "Field `MIS_COMPIFG` reader - Masked interrupt status for COMPIFG"] pub type MIS_COMPIFG_R = crate :: BitReader < MIS_COMPIFG_A > ; # [doc = "Masked interrupt status for COMPIFG\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum MIS_COMPIFG_A { # [doc = "0: CLR"] MIS_COMPIFG_CLR = 0 , # [doc = "1: SET"] MIS_COMPIFG_SET = 1 , } impl From < MIS_COMPIFG_A > for bool { # [inline (always)] fn from (variant : MIS_COMPIFG_A) -> Self { variant as u8 != 0 } } impl MIS_COMPIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> MIS_COMPIFG_A { match self . bits { false => MIS_COMPIFG_A :: MIS_COMPIFG_CLR , true => MIS_COMPIFG_A :: MIS_COMPIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_mis_compifg_clr (& self) -> bool { * self == MIS_COMPIFG_A :: MIS_COMPIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_mis_compifg_set (& self) -> bool { * self == MIS_COMPIFG_A :: MIS_COMPIFG_SET } } # [doc = "Field `MIS_COMPINVIFG` reader - Masked interrupt status for COMPINVIFG"] pub type MIS_COMPINVIFG_R = crate :: BitReader < MIS_COMPINVIFG_A > ; # [doc = "Masked interrupt status for COMPINVIFG\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum MIS_COMPINVIFG_A { # [doc = "0: CLR"] MIS_COMPINVIFG_CLR = 0 , # [doc = "1: SET"] MIS_COMPINVIFG_SET = 1 , } impl From < MIS_COMPINVIFG_A > for bool { # [inline (always)] fn from (variant : MIS_COMPINVIFG_A) -> Self { variant as u8 != 0 } } impl MIS_COMPINVIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> MIS_COMPINVIFG_A { match self . bits { false => MIS_COMPINVIFG_A :: MIS_COMPINVIFG_CLR , true => MIS_COMPINVIFG_A :: MIS_COMPINVIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_mis_compinvifg_clr (& self) -> bool { * self == MIS_COMPINVIFG_A :: MIS_COMPINVIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_mis_compinvifg_set (& self) -> bool { * self == MIS_COMPINVIFG_A :: MIS_COMPINVIFG_SET } } # [doc = "Field `MIS_OUTRDYIFG` reader - Masked interrupt status for OUTRDYIFG"] pub type MIS_OUTRDYIFG_R = crate :: BitReader < MIS_OUTRDYIFG_A > ; # [doc = "Masked interrupt status for OUTRDYIFG\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum MIS_OUTRDYIFG_A { # [doc = "0: CLR"] MIS_OUTRDYIFG_CLR = 0 , # [doc = "1: SET"] MIS_OUTRDYIFG_SET = 1 , } impl From < MIS_OUTRDYIFG_A > for bool { # [inline (always)] fn from (variant : MIS_OUTRDYIFG_A) -> Self { variant as u8 != 0 } } impl MIS_OUTRDYIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> MIS_OUTRDYIFG_A { match self . bits { false => MIS_OUTRDYIFG_A :: MIS_OUTRDYIFG_CLR , true => MIS_OUTRDYIFG_A :: MIS_OUTRDYIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_mis_outrdyifg_clr (& self) -> bool { * self == MIS_OUTRDYIFG_A :: MIS_OUTRDYIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_mis_outrdyifg_set (& self) -> bool { * self == MIS_OUTRDYIFG_A :: MIS_OUTRDYIFG_SET } } impl R { # [doc = "Bit 1 - Masked interrupt status for COMPIFG"] # [inline (always)] pub fn mis_compifg (& self) -> MIS_COMPIFG_R { MIS_COMPIFG_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Masked interrupt status for COMPINVIFG"] # [inline (always)] pub fn mis_compinvifg (& self) -> MIS_COMPINVIFG_R { MIS_COMPINVIFG_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - Masked interrupt status for OUTRDYIFG"] # [inline (always)] pub fn mis_outrdyifg (& self) -> MIS_OUTRDYIFG_R { MIS_OUTRDYIFG_R :: new (((self . bits >> 3) & 1) != 0) } } # [doc = "Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct MIS_SPEC ; impl crate :: RegisterSpec for MIS_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`mis::R`](R) reader structure"] impl crate :: Readable for MIS_SPEC { } # [doc = "`reset()` method sets MIS to value 0"] impl crate :: Resettable for MIS_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }