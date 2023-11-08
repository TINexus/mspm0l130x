# [doc = "Register `INT_EVENT0_IIDX` reader"] pub type R = crate :: R < INT_EVENT0_IIDX_SPEC > ; # [doc = "Field `INT_EVENT0_IIDX_STAT` reader - Interrupt index status"] pub type INT_EVENT0_IIDX_STAT_R = crate :: FieldReader < INT_EVENT0_IIDX_STAT_A > ; # [doc = "Interrupt index status\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u16)] pub enum INT_EVENT0_IIDX_STAT_A { # [doc = "0: NO_INTR"] INT_EVENT0_IIDX_STAT_NO_INTR = 0 , # [doc = "1: OVIFG"] INT_EVENT0_IIDX_STAT_OVIFG = 1 , # [doc = "2: TOVIFG"] INT_EVENT0_IIDX_STAT_TOVIFG = 2 , # [doc = "3: HIGHIFG"] INT_EVENT0_IIDX_STAT_HIGHIFG = 3 , # [doc = "4: LOWIFG"] INT_EVENT0_IIDX_STAT_LOWIFG = 4 , # [doc = "5: INIFG"] INT_EVENT0_IIDX_STAT_INIFG = 5 , # [doc = "6: DMADONE"] INT_EVENT0_IIDX_STAT_DMADONE = 6 , # [doc = "7: UVIFG"] INT_EVENT0_IIDX_STAT_UVIFG = 7 , # [doc = "9: MEMRESIFG0"] INT_EVENT0_IIDX_STAT_MEMRESIFG0 = 9 , # [doc = "10: MEMRESIFG1"] INT_EVENT0_IIDX_STAT_MEMRESIFG1 = 10 , # [doc = "11: MEMRESIFG2"] INT_EVENT0_IIDX_STAT_MEMRESIFG2 = 11 , # [doc = "12: MEMRESIFG3"] INT_EVENT0_IIDX_STAT_MEMRESIFG3 = 12 , # [doc = "13: MEMRESIFG4"] INT_EVENT0_IIDX_STAT_MEMRESIFG4 = 13 , # [doc = "14: MEMRESIFG5"] INT_EVENT0_IIDX_STAT_MEMRESIFG5 = 14 , # [doc = "15: MEMRESIFG6"] INT_EVENT0_IIDX_STAT_MEMRESIFG6 = 15 , # [doc = "16: MEMRESIFG7"] INT_EVENT0_IIDX_STAT_MEMRESIFG7 = 16 , # [doc = "17: MEMRESIFG8"] INT_EVENT0_IIDX_STAT_MEMRESIFG8 = 17 , # [doc = "18: MEMRESIFG9"] INT_EVENT0_IIDX_STAT_MEMRESIFG9 = 18 , # [doc = "19: MEMRESIFG10"] INT_EVENT0_IIDX_STAT_MEMRESIFG10 = 19 , # [doc = "20: MEMRESIFG11"] INT_EVENT0_IIDX_STAT_MEMRESIFG11 = 20 , # [doc = "21: MEMRESIFG12"] INT_EVENT0_IIDX_STAT_MEMRESIFG12 = 21 , # [doc = "22: MEMRESIFG13"] INT_EVENT0_IIDX_STAT_MEMRESIFG13 = 22 , # [doc = "23: MEMRESIFG14"] INT_EVENT0_IIDX_STAT_MEMRESIFG14 = 23 , # [doc = "24: MEMRESIFG15"] INT_EVENT0_IIDX_STAT_MEMRESIFG15 = 24 , # [doc = "25: MEMRESIFG16"] INT_EVENT0_IIDX_STAT_MEMRESIFG16 = 25 , # [doc = "26: MEMRESIFG17"] INT_EVENT0_IIDX_STAT_MEMRESIFG17 = 26 , # [doc = "27: MEMRESIFG18"] INT_EVENT0_IIDX_STAT_MEMRESIFG18 = 27 , # [doc = "28: MEMRESIFG19"] INT_EVENT0_IIDX_STAT_MEMRESIFG19 = 28 , # [doc = "29: MEMRESIFG20"] INT_EVENT0_IIDX_STAT_MEMRESIFG20 = 29 , # [doc = "30: MEMRESIFG21"] INT_EVENT0_IIDX_STAT_MEMRESIFG21 = 30 , # [doc = "31: MEMRESIFG22"] INT_EVENT0_IIDX_STAT_MEMRESIFG22 = 31 , # [doc = "32: MEMRESIFG23"] INT_EVENT0_IIDX_STAT_MEMRESIFG23 = 32 , } impl From < INT_EVENT0_IIDX_STAT_A > for u16 { # [inline (always)] fn from (variant : INT_EVENT0_IIDX_STAT_A) -> Self { variant as _ } } impl crate :: FieldSpec for INT_EVENT0_IIDX_STAT_A { type Ux = u16 ; } impl INT_EVENT0_IIDX_STAT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < INT_EVENT0_IIDX_STAT_A > { match self . bits { 0 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_NO_INTR) , 1 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_OVIFG) , 2 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_TOVIFG) , 3 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_HIGHIFG) , 4 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_LOWIFG) , 5 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_INIFG) , 6 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_DMADONE) , 7 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_UVIFG) , 9 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG0) , 10 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG1) , 11 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG2) , 12 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG3) , 13 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG4) , 14 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG5) , 15 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG6) , 16 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG7) , 17 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG8) , 18 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG9) , 19 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG10) , 20 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG11) , 21 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG12) , 22 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG13) , 23 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG14) , 24 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG15) , 25 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG16) , 26 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG17) , 27 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG18) , 28 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG19) , 29 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG20) , 30 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG21) , 31 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG22) , 32 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG23) , _ => None , } } # [doc = "NO_INTR"] # [inline (always)] pub fn is_int_event0_iidx_stat_no_intr (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_NO_INTR } # [doc = "OVIFG"] # [inline (always)] pub fn is_int_event0_iidx_stat_ovifg (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_OVIFG } # [doc = "TOVIFG"] # [inline (always)] pub fn is_int_event0_iidx_stat_tovifg (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_TOVIFG } # [doc = "HIGHIFG"] # [inline (always)] pub fn is_int_event0_iidx_stat_highifg (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_HIGHIFG } # [doc = "LOWIFG"] # [inline (always)] pub fn is_int_event0_iidx_stat_lowifg (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_LOWIFG } # [doc = "INIFG"] # [inline (always)] pub fn is_int_event0_iidx_stat_inifg (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_INIFG } # [doc = "DMADONE"] # [inline (always)] pub fn is_int_event0_iidx_stat_dmadone (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_DMADONE } # [doc = "UVIFG"] # [inline (always)] pub fn is_int_event0_iidx_stat_uvifg (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_UVIFG } # [doc = "MEMRESIFG0"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg0 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG0 } # [doc = "MEMRESIFG1"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg1 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG1 } # [doc = "MEMRESIFG2"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg2 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG2 } # [doc = "MEMRESIFG3"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg3 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG3 } # [doc = "MEMRESIFG4"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg4 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG4 } # [doc = "MEMRESIFG5"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg5 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG5 } # [doc = "MEMRESIFG6"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg6 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG6 } # [doc = "MEMRESIFG7"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg7 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG7 } # [doc = "MEMRESIFG8"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg8 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG8 } # [doc = "MEMRESIFG9"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg9 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG9 } # [doc = "MEMRESIFG10"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg10 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG10 } # [doc = "MEMRESIFG11"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg11 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG11 } # [doc = "MEMRESIFG12"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg12 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG12 } # [doc = "MEMRESIFG13"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg13 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG13 } # [doc = "MEMRESIFG14"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg14 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG14 } # [doc = "MEMRESIFG15"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg15 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG15 } # [doc = "MEMRESIFG16"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg16 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG16 } # [doc = "MEMRESIFG17"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg17 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG17 } # [doc = "MEMRESIFG18"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg18 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG18 } # [doc = "MEMRESIFG19"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg19 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG19 } # [doc = "MEMRESIFG20"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg20 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG20 } # [doc = "MEMRESIFG21"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg21 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG21 } # [doc = "MEMRESIFG22"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg22 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG22 } # [doc = "MEMRESIFG23"] # [inline (always)] pub fn is_int_event0_iidx_stat_memresifg23 (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_MEMRESIFG23 } } impl R { # [doc = "Bits 0:9 - Interrupt index status"] # [inline (always)] pub fn int_event0_iidx_stat (& self) -> INT_EVENT0_IIDX_STAT_R { INT_EVENT0_IIDX_STAT_R :: new ((self . bits & 0x03ff) as u16) } } # [doc = "Interrupt index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event0_iidx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT0_IIDX_SPEC ; impl crate :: RegisterSpec for INT_EVENT0_IIDX_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event0_iidx::R`](R) reader structure"] impl crate :: Readable for INT_EVENT0_IIDX_SPEC { } # [doc = "`reset()` method sets INT_EVENT0_IIDX to value 0"] impl crate :: Resettable for INT_EVENT0_IIDX_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }