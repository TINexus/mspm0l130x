# [doc = "Register `FCC` reader"] pub type R = crate :: R < FCC_SPEC > ; # [doc = "Field `FCC_DATA` reader - Frequency clock counter (FCC) count value."] pub type FCC_DATA_R = crate :: FieldReader < u32 > ; impl R { # [doc = "Bits 0:21 - Frequency clock counter (FCC) count value."] # [inline (always)] pub fn fcc_data (& self) -> FCC_DATA_R { FCC_DATA_R :: new (self . bits & 0x003f_ffff) } } # [doc = "Frequency clock counter (FCC) count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct FCC_SPEC ; impl crate :: RegisterSpec for FCC_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`fcc::R`](R) reader structure"] impl crate :: Readable for FCC_SPEC { } # [doc = "`reset()` method sets FCC to value 0"] impl crate :: Resettable for FCC_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }