# [doc = "Register `INT_GROUP0_RIS` reader"] pub type R = crate :: R < INT_GROUP0_RIS_SPEC > ; # [doc = "Field `INT_GROUP0_RIS_INT` reader - Raw interrupt status for INT"] pub type INT_GROUP0_RIS_INT_R = crate :: FieldReader < INT_GROUP0_RIS_INT_A > ; # [doc = "Raw interrupt status for INT\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum INT_GROUP0_RIS_INT_A { # [doc = "0: CLR"] INT_GROUP0_RIS_INT_CLR = 0 , # [doc = "1: SET"] INT_GROUP0_RIS_INT_SET = 1 , } impl From < INT_GROUP0_RIS_INT_A > for u8 { # [inline (always)] fn from (variant : INT_GROUP0_RIS_INT_A) -> Self { variant as _ } } impl crate :: FieldSpec for INT_GROUP0_RIS_INT_A { type Ux = u8 ; } impl INT_GROUP0_RIS_INT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < INT_GROUP0_RIS_INT_A > { match self . bits { 0 => Some (INT_GROUP0_RIS_INT_A :: INT_GROUP0_RIS_INT_CLR) , 1 => Some (INT_GROUP0_RIS_INT_A :: INT_GROUP0_RIS_INT_SET) , _ => None , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_group0_ris_int_clr (& self) -> bool { * self == INT_GROUP0_RIS_INT_A :: INT_GROUP0_RIS_INT_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_group0_ris_int_set (& self) -> bool { * self == INT_GROUP0_RIS_INT_A :: INT_GROUP0_RIS_INT_SET } } impl R { # [doc = "Bits 0:7 - Raw interrupt status for INT"] # [inline (always)] pub fn int_group0_ris_int (& self) -> INT_GROUP0_RIS_INT_R { INT_GROUP0_RIS_INT_R :: new ((self . bits & 0xff) as u8) } } # [doc = "Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_group0_ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_GROUP0_RIS_SPEC ; impl crate :: RegisterSpec for INT_GROUP0_RIS_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_group0_ris::R`](R) reader structure"] impl crate :: Readable for INT_GROUP0_RIS_SPEC { } # [doc = "`reset()` method sets INT_GROUP0_RIS to value 0"] impl crate :: Resettable for INT_GROUP0_RIS_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }