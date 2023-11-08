# [doc = "Register `CLKCFG` reader"] pub type R = crate :: R < CLKCFG_SPEC > ; # [doc = "Register `CLKCFG` writer"] pub type W = crate :: W < CLKCFG_SPEC > ; # [doc = "Field `CLKCFG_SAMPCLK` reader - ADC sample clock source selection."] pub type CLKCFG_SAMPCLK_R = crate :: FieldReader < CLKCFG_SAMPCLK_A > ; # [doc = "ADC sample clock source selection.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CLKCFG_SAMPCLK_A { # [doc = "0: SYSOSC"] CLKCFG_SAMPCLK_SYSOSC = 0 , # [doc = "1: ULPCLK"] CLKCFG_SAMPCLK_ULPCLK = 1 , # [doc = "2: HFCLK"] CLKCFG_SAMPCLK_HFCLK = 2 , } impl From < CLKCFG_SAMPCLK_A > for u8 { # [inline (always)] fn from (variant : CLKCFG_SAMPCLK_A) -> Self { variant as _ } } impl crate :: FieldSpec for CLKCFG_SAMPCLK_A { type Ux = u8 ; } impl CLKCFG_SAMPCLK_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < CLKCFG_SAMPCLK_A > { match self . bits { 0 => Some (CLKCFG_SAMPCLK_A :: CLKCFG_SAMPCLK_SYSOSC) , 1 => Some (CLKCFG_SAMPCLK_A :: CLKCFG_SAMPCLK_ULPCLK) , 2 => Some (CLKCFG_SAMPCLK_A :: CLKCFG_SAMPCLK_HFCLK) , _ => None , } } # [doc = "SYSOSC"] # [inline (always)] pub fn is_clkcfg_sampclk_sysosc (& self) -> bool { * self == CLKCFG_SAMPCLK_A :: CLKCFG_SAMPCLK_SYSOSC } # [doc = "ULPCLK"] # [inline (always)] pub fn is_clkcfg_sampclk_ulpclk (& self) -> bool { * self == CLKCFG_SAMPCLK_A :: CLKCFG_SAMPCLK_ULPCLK } # [doc = "HFCLK"] # [inline (always)] pub fn is_clkcfg_sampclk_hfclk (& self) -> bool { * self == CLKCFG_SAMPCLK_A :: CLKCFG_SAMPCLK_HFCLK } } # [doc = "Field `CLKCFG_SAMPCLK` writer - ADC sample clock source selection."] pub type CLKCFG_SAMPCLK_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 2 , O , CLKCFG_SAMPCLK_A > ; impl < 'a , REG , const O : u8 > CLKCFG_SAMPCLK_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "SYSOSC"] # [inline (always)] pub fn clkcfg_sampclk_sysosc (self) -> & 'a mut crate :: W < REG > { self . variant (CLKCFG_SAMPCLK_A :: CLKCFG_SAMPCLK_SYSOSC) } # [doc = "ULPCLK"] # [inline (always)] pub fn clkcfg_sampclk_ulpclk (self) -> & 'a mut crate :: W < REG > { self . variant (CLKCFG_SAMPCLK_A :: CLKCFG_SAMPCLK_ULPCLK) } # [doc = "HFCLK"] # [inline (always)] pub fn clkcfg_sampclk_hfclk (self) -> & 'a mut crate :: W < REG > { self . variant (CLKCFG_SAMPCLK_A :: CLKCFG_SAMPCLK_HFCLK) } } # [doc = "Field `CLKCFG_CCONRUN` reader - CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source."] pub type CLKCFG_CCONRUN_R = crate :: BitReader < CLKCFG_CCONRUN_A > ; # [doc = "CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CLKCFG_CCONRUN_A { # [doc = "0: DISABLE"] CLKCFG_CCONRUN_DISABLE = 0 , # [doc = "1: ENABLE"] CLKCFG_CCONRUN_ENABLE = 1 , } impl From < CLKCFG_CCONRUN_A > for bool { # [inline (always)] fn from (variant : CLKCFG_CCONRUN_A) -> Self { variant as u8 != 0 } } impl CLKCFG_CCONRUN_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CLKCFG_CCONRUN_A { match self . bits { false => CLKCFG_CCONRUN_A :: CLKCFG_CCONRUN_DISABLE , true => CLKCFG_CCONRUN_A :: CLKCFG_CCONRUN_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_clkcfg_cconrun_disable (& self) -> bool { * self == CLKCFG_CCONRUN_A :: CLKCFG_CCONRUN_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_clkcfg_cconrun_enable (& self) -> bool { * self == CLKCFG_CCONRUN_A :: CLKCFG_CCONRUN_ENABLE } } # [doc = "Field `CLKCFG_CCONRUN` writer - CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source."] pub type CLKCFG_CCONRUN_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CLKCFG_CCONRUN_A > ; impl < 'a , REG , const O : u8 > CLKCFG_CCONRUN_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn clkcfg_cconrun_disable (self) -> & 'a mut crate :: W < REG > { self . variant (CLKCFG_CCONRUN_A :: CLKCFG_CCONRUN_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn clkcfg_cconrun_enable (self) -> & 'a mut crate :: W < REG > { self . variant (CLKCFG_CCONRUN_A :: CLKCFG_CCONRUN_ENABLE) } } # [doc = "Field `CLKCFG_CCONSTOP` reader - CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source."] pub type CLKCFG_CCONSTOP_R = crate :: BitReader < CLKCFG_CCONSTOP_A > ; # [doc = "CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CLKCFG_CCONSTOP_A { # [doc = "0: DISABLE"] CLKCFG_CCONSTOP_DISABLE = 0 , # [doc = "1: ENABLE"] CLKCFG_CCONSTOP_ENABLE = 1 , } impl From < CLKCFG_CCONSTOP_A > for bool { # [inline (always)] fn from (variant : CLKCFG_CCONSTOP_A) -> Self { variant as u8 != 0 } } impl CLKCFG_CCONSTOP_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CLKCFG_CCONSTOP_A { match self . bits { false => CLKCFG_CCONSTOP_A :: CLKCFG_CCONSTOP_DISABLE , true => CLKCFG_CCONSTOP_A :: CLKCFG_CCONSTOP_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_clkcfg_cconstop_disable (& self) -> bool { * self == CLKCFG_CCONSTOP_A :: CLKCFG_CCONSTOP_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_clkcfg_cconstop_enable (& self) -> bool { * self == CLKCFG_CCONSTOP_A :: CLKCFG_CCONSTOP_ENABLE } } # [doc = "Field `CLKCFG_CCONSTOP` writer - CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source."] pub type CLKCFG_CCONSTOP_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CLKCFG_CCONSTOP_A > ; impl < 'a , REG , const O : u8 > CLKCFG_CCONSTOP_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn clkcfg_cconstop_disable (self) -> & 'a mut crate :: W < REG > { self . variant (CLKCFG_CCONSTOP_A :: CLKCFG_CCONSTOP_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn clkcfg_cconstop_enable (self) -> & 'a mut crate :: W < REG > { self . variant (CLKCFG_CCONSTOP_A :: CLKCFG_CCONSTOP_ENABLE) } } # [doc = "Unlock key\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CLKCFG_KEY_AW { # [doc = "169: _TO_UNLOCK_W_"] CLKCFG_KEY_UNLOCK_W = 169 , } impl From < CLKCFG_KEY_AW > for u8 { # [inline (always)] fn from (variant : CLKCFG_KEY_AW) -> Self { variant as _ } } impl crate :: FieldSpec for CLKCFG_KEY_AW { type Ux = u8 ; } # [doc = "Field `CLKCFG_KEY` writer - Unlock key"] pub type CLKCFG_KEY_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 8 , O , CLKCFG_KEY_AW > ; impl < 'a , REG , const O : u8 > CLKCFG_KEY_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "_TO_UNLOCK_W_"] # [inline (always)] pub fn clkcfg_key_unlock_w (self) -> & 'a mut crate :: W < REG > { self . variant (CLKCFG_KEY_AW :: CLKCFG_KEY_UNLOCK_W) } } impl R { # [doc = "Bits 0:1 - ADC sample clock source selection."] # [inline (always)] pub fn clkcfg_sampclk (& self) -> CLKCFG_SAMPCLK_R { CLKCFG_SAMPCLK_R :: new ((self . bits & 3) as u8) } # [doc = "Bit 4 - CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source."] # [inline (always)] pub fn clkcfg_cconrun (& self) -> CLKCFG_CCONRUN_R { CLKCFG_CCONRUN_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source."] # [inline (always)] pub fn clkcfg_cconstop (& self) -> CLKCFG_CCONSTOP_R { CLKCFG_CCONSTOP_R :: new (((self . bits >> 5) & 1) != 0) } } impl W { # [doc = "Bits 0:1 - ADC sample clock source selection."] # [inline (always)] # [must_use] pub fn clkcfg_sampclk (& mut self) -> CLKCFG_SAMPCLK_W < CLKCFG_SPEC , 0 > { CLKCFG_SAMPCLK_W :: new (self) } # [doc = "Bit 4 - CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source."] # [inline (always)] # [must_use] pub fn clkcfg_cconrun (& mut self) -> CLKCFG_CCONRUN_W < CLKCFG_SPEC , 4 > { CLKCFG_CCONRUN_W :: new (self) } # [doc = "Bit 5 - CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source."] # [inline (always)] # [must_use] pub fn clkcfg_cconstop (& mut self) -> CLKCFG_CCONSTOP_W < CLKCFG_SPEC , 5 > { CLKCFG_CCONSTOP_W :: new (self) } # [doc = "Bits 24:31 - Unlock key"] # [inline (always)] # [must_use] pub fn clkcfg_key (& mut self) -> CLKCFG_KEY_W < CLKCFG_SPEC , 24 > { CLKCFG_KEY_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "ADC clock configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CLKCFG_SPEC ; impl crate :: RegisterSpec for CLKCFG_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`clkcfg::R`](R) reader structure"] impl crate :: Readable for CLKCFG_SPEC { } # [doc = "`write(|w| ..)` method takes [`clkcfg::W`](W) writer structure"] impl crate :: Writable for CLKCFG_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CLKCFG to value 0"] impl crate :: Resettable for CLKCFG_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }