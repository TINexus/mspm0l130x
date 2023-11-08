# [doc = "Register `EXRSTPIN` writer"] pub type W = crate :: W < EXRSTPIN_SPEC > ; # [doc = "Set DISABLE to disable the reset function of the NRST pin. Once set, this configuration is locked until the next POR.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum EXRSTPIN_DISABLE_AW { # [doc = "0: FALSE"] EXRSTPIN_DISABLE_FALSE = 0 , # [doc = "1: TRUE"] EXRSTPIN_DISABLE_TRUE = 1 , } impl From < EXRSTPIN_DISABLE_AW > for bool { # [inline (always)] fn from (variant : EXRSTPIN_DISABLE_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `EXRSTPIN_DISABLE` writer - Set DISABLE to disable the reset function of the NRST pin. Once set, this configuration is locked until the next POR."] pub type EXRSTPIN_DISABLE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , EXRSTPIN_DISABLE_AW > ; impl < 'a , REG , const O : u8 > EXRSTPIN_DISABLE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "FALSE"] # [inline (always)] pub fn exrstpin_disable_false (self) -> & 'a mut crate :: W < REG > { self . variant (EXRSTPIN_DISABLE_AW :: EXRSTPIN_DISABLE_FALSE) } # [doc = "TRUE"] # [inline (always)] pub fn exrstpin_disable_true (self) -> & 'a mut crate :: W < REG > { self . variant (EXRSTPIN_DISABLE_AW :: EXRSTPIN_DISABLE_TRUE) } } # [doc = "The key value 1Eh must be written together with DISABLE to disable the reset function.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum EXRSTPIN_KEY_AW { # [doc = "30: VALUE"] EXRSTPIN_KEY_VALUE = 30 , } impl From < EXRSTPIN_KEY_AW > for u8 { # [inline (always)] fn from (variant : EXRSTPIN_KEY_AW) -> Self { variant as _ } } impl crate :: FieldSpec for EXRSTPIN_KEY_AW { type Ux = u8 ; } # [doc = "Field `EXRSTPIN_KEY` writer - The key value 1Eh must be written together with DISABLE to disable the reset function."] pub type EXRSTPIN_KEY_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 8 , O , EXRSTPIN_KEY_AW > ; impl < 'a , REG , const O : u8 > EXRSTPIN_KEY_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "VALUE"] # [inline (always)] pub fn exrstpin_key_value (self) -> & 'a mut crate :: W < REG > { self . variant (EXRSTPIN_KEY_AW :: EXRSTPIN_KEY_VALUE) } } impl W { # [doc = "Bit 0 - Set DISABLE to disable the reset function of the NRST pin. Once set, this configuration is locked until the next POR."] # [inline (always)] # [must_use] pub fn exrstpin_disable (& mut self) -> EXRSTPIN_DISABLE_W < EXRSTPIN_SPEC , 0 > { EXRSTPIN_DISABLE_W :: new (self) } # [doc = "Bits 24:31 - The key value 1Eh must be written together with DISABLE to disable the reset function."] # [inline (always)] # [must_use] pub fn exrstpin_key (& mut self) -> EXRSTPIN_KEY_W < EXRSTPIN_SPEC , 24 > { EXRSTPIN_KEY_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Disable the reset function of the NRST pin\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exrstpin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct EXRSTPIN_SPEC ; impl crate :: RegisterSpec for EXRSTPIN_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`exrstpin::W`](W) writer structure"] impl crate :: Writable for EXRSTPIN_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets EXRSTPIN to value 0"] impl crate :: Resettable for EXRSTPIN_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }