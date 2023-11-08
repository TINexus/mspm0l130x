# [doc = "Register `RIS` reader"] pub type R = crate :: R < RIS_SPEC > ; # [doc = "Field `RIS_Z` reader - Zero event generated an interrupt."] pub type RIS_Z_R = crate :: BitReader < RIS_Z_A > ; # [doc = "Zero event generated an interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum RIS_Z_A { # [doc = "0: CLR"] RIS_Z_CLR = 0 , # [doc = "1: SET"] RIS_Z_SET = 1 , } impl From < RIS_Z_A > for bool { # [inline (always)] fn from (variant : RIS_Z_A) -> Self { variant as u8 != 0 } } impl RIS_Z_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> RIS_Z_A { match self . bits { false => RIS_Z_A :: RIS_Z_CLR , true => RIS_Z_A :: RIS_Z_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_ris_z_clr (& self) -> bool { * self == RIS_Z_A :: RIS_Z_CLR } # [doc = "SET"] # [inline (always)] pub fn is_ris_z_set (& self) -> bool { * self == RIS_Z_A :: RIS_Z_SET } } # [doc = "Field `RIS_L` reader - Load event generated an interrupt."] pub type RIS_L_R = crate :: BitReader < RIS_L_A > ; # [doc = "Load event generated an interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum RIS_L_A { # [doc = "0: CLR"] RIS_L_CLR = 0 , # [doc = "1: SET"] RIS_L_SET = 1 , } impl From < RIS_L_A > for bool { # [inline (always)] fn from (variant : RIS_L_A) -> Self { variant as u8 != 0 } } impl RIS_L_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> RIS_L_A { match self . bits { false => RIS_L_A :: RIS_L_CLR , true => RIS_L_A :: RIS_L_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_ris_l_clr (& self) -> bool { * self == RIS_L_A :: RIS_L_CLR } # [doc = "SET"] # [inline (always)] pub fn is_ris_l_set (& self) -> bool { * self == RIS_L_A :: RIS_L_SET } } # [doc = "Field `RIS_CCD0` reader - Capture or compare down event generated an interrupt CCP0"] pub type RIS_CCD0_R = crate :: BitReader < RIS_CCD0_A > ; # [doc = "Capture or compare down event generated an interrupt CCP0\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum RIS_CCD0_A { # [doc = "0: CLR"] RIS_CCD0_CLR = 0 , # [doc = "1: SET"] RIS_CCD0_SET = 1 , } impl From < RIS_CCD0_A > for bool { # [inline (always)] fn from (variant : RIS_CCD0_A) -> Self { variant as u8 != 0 } } impl RIS_CCD0_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> RIS_CCD0_A { match self . bits { false => RIS_CCD0_A :: RIS_CCD0_CLR , true => RIS_CCD0_A :: RIS_CCD0_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_ris_ccd0_clr (& self) -> bool { * self == RIS_CCD0_A :: RIS_CCD0_CLR } # [doc = "SET"] # [inline (always)] pub fn is_ris_ccd0_set (& self) -> bool { * self == RIS_CCD0_A :: RIS_CCD0_SET } } # [doc = "Field `RIS_CCD1` reader - Capture or compare down event generated an interrupt CCP1"] pub type RIS_CCD1_R = crate :: BitReader < RIS_CCD1_A > ; # [doc = "Capture or compare down event generated an interrupt CCP1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum RIS_CCD1_A { # [doc = "0: CLR"] RIS_CCD1_CLR = 0 , # [doc = "1: SET"] RIS_CCD1_SET = 1 , } impl From < RIS_CCD1_A > for bool { # [inline (always)] fn from (variant : RIS_CCD1_A) -> Self { variant as u8 != 0 } } impl RIS_CCD1_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> RIS_CCD1_A { match self . bits { false => RIS_CCD1_A :: RIS_CCD1_CLR , true => RIS_CCD1_A :: RIS_CCD1_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_ris_ccd1_clr (& self) -> bool { * self == RIS_CCD1_A :: RIS_CCD1_CLR } # [doc = "SET"] # [inline (always)] pub fn is_ris_ccd1_set (& self) -> bool { * self == RIS_CCD1_A :: RIS_CCD1_SET } } # [doc = "Field `RIS_CCU0` reader - Capture or compare up event generated an interrupt CCP0"] pub type RIS_CCU0_R = crate :: BitReader < RIS_CCU0_A > ; # [doc = "Capture or compare up event generated an interrupt CCP0\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum RIS_CCU0_A { # [doc = "0: CLR"] RIS_CCU0_CLR = 0 , # [doc = "1: SET"] RIS_CCU0_SET = 1 , } impl From < RIS_CCU0_A > for bool { # [inline (always)] fn from (variant : RIS_CCU0_A) -> Self { variant as u8 != 0 } } impl RIS_CCU0_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> RIS_CCU0_A { match self . bits { false => RIS_CCU0_A :: RIS_CCU0_CLR , true => RIS_CCU0_A :: RIS_CCU0_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_ris_ccu0_clr (& self) -> bool { * self == RIS_CCU0_A :: RIS_CCU0_CLR } # [doc = "SET"] # [inline (always)] pub fn is_ris_ccu0_set (& self) -> bool { * self == RIS_CCU0_A :: RIS_CCU0_SET } } # [doc = "Field `RIS_CCU1` reader - Capture or compare up event generated an interrupt CCP1"] pub type RIS_CCU1_R = crate :: BitReader < RIS_CCU1_A > ; # [doc = "Capture or compare up event generated an interrupt CCP1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum RIS_CCU1_A { # [doc = "0: CLR"] RIS_CCU1_CLR = 0 , # [doc = "1: SET"] RIS_CCU1_SET = 1 , } impl From < RIS_CCU1_A > for bool { # [inline (always)] fn from (variant : RIS_CCU1_A) -> Self { variant as u8 != 0 } } impl RIS_CCU1_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> RIS_CCU1_A { match self . bits { false => RIS_CCU1_A :: RIS_CCU1_CLR , true => RIS_CCU1_A :: RIS_CCU1_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_ris_ccu1_clr (& self) -> bool { * self == RIS_CCU1_A :: RIS_CCU1_CLR } # [doc = "SET"] # [inline (always)] pub fn is_ris_ccu1_set (& self) -> bool { * self == RIS_CCU1_A :: RIS_CCU1_SET } } # [doc = "Field `RIS_TOV` reader - Trigger overflow"] pub type RIS_TOV_R = crate :: BitReader < RIS_TOV_A > ; # [doc = "Trigger overflow\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum RIS_TOV_A { # [doc = "0: CLR"] RIS_TOV_CLR = 0 , # [doc = "1: SET"] RIS_TOV_SET = 1 , } impl From < RIS_TOV_A > for bool { # [inline (always)] fn from (variant : RIS_TOV_A) -> Self { variant as u8 != 0 } } impl RIS_TOV_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> RIS_TOV_A { match self . bits { false => RIS_TOV_A :: RIS_TOV_CLR , true => RIS_TOV_A :: RIS_TOV_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_ris_tov_clr (& self) -> bool { * self == RIS_TOV_A :: RIS_TOV_CLR } # [doc = "SET"] # [inline (always)] pub fn is_ris_tov_set (& self) -> bool { * self == RIS_TOV_A :: RIS_TOV_SET } } impl R { # [doc = "Bit 0 - Zero event generated an interrupt."] # [inline (always)] pub fn ris_z (& self) -> RIS_Z_R { RIS_Z_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Load event generated an interrupt."] # [inline (always)] pub fn ris_l (& self) -> RIS_L_R { RIS_L_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 4 - Capture or compare down event generated an interrupt CCP0"] # [inline (always)] pub fn ris_ccd0 (& self) -> RIS_CCD0_R { RIS_CCD0_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - Capture or compare down event generated an interrupt CCP1"] # [inline (always)] pub fn ris_ccd1 (& self) -> RIS_CCD1_R { RIS_CCD1_R :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 8 - Capture or compare up event generated an interrupt CCP0"] # [inline (always)] pub fn ris_ccu0 (& self) -> RIS_CCU0_R { RIS_CCU0_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - Capture or compare up event generated an interrupt CCP1"] # [inline (always)] pub fn ris_ccu1 (& self) -> RIS_CCU1_R { RIS_CCU1_R :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 25 - Trigger overflow"] # [inline (always)] pub fn ris_tov (& self) -> RIS_TOV_R { RIS_TOV_R :: new (((self . bits >> 25) & 1) != 0) } } # [doc = "Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct RIS_SPEC ; impl crate :: RegisterSpec for RIS_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`ris::R`](R) reader structure"] impl crate :: Readable for RIS_SPEC { } # [doc = "`reset()` method sets RIS to value 0"] impl crate :: Resettable for RIS_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }