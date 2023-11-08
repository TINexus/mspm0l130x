# [doc = "Register `WRITELOCK` reader"] pub type R = crate :: R < WRITELOCK_SPEC > ; # [doc = "Register `WRITELOCK` writer"] pub type W = crate :: W < WRITELOCK_SPEC > ; # [doc = "Field `WRITELOCK_ACTIVE` reader - ACTIVE controls whether critical SYSCTL registers are write protected or not."] pub type WRITELOCK_ACTIVE_R = crate :: BitReader < WRITELOCK_ACTIVE_A > ; # [doc = "ACTIVE controls whether critical SYSCTL registers are write protected or not.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum WRITELOCK_ACTIVE_A { # [doc = "0: DISABLE"] WRITELOCK_ACTIVE_DISABLE = 0 , # [doc = "1: ENABLE"] WRITELOCK_ACTIVE_ENABLE = 1 , } impl From < WRITELOCK_ACTIVE_A > for bool { # [inline (always)] fn from (variant : WRITELOCK_ACTIVE_A) -> Self { variant as u8 != 0 } } impl WRITELOCK_ACTIVE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> WRITELOCK_ACTIVE_A { match self . bits { false => WRITELOCK_ACTIVE_A :: WRITELOCK_ACTIVE_DISABLE , true => WRITELOCK_ACTIVE_A :: WRITELOCK_ACTIVE_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_writelock_active_disable (& self) -> bool { * self == WRITELOCK_ACTIVE_A :: WRITELOCK_ACTIVE_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_writelock_active_enable (& self) -> bool { * self == WRITELOCK_ACTIVE_A :: WRITELOCK_ACTIVE_ENABLE } } # [doc = "Field `WRITELOCK_ACTIVE` writer - ACTIVE controls whether critical SYSCTL registers are write protected or not."] pub type WRITELOCK_ACTIVE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , WRITELOCK_ACTIVE_A > ; impl < 'a , REG , const O : u8 > WRITELOCK_ACTIVE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn writelock_active_disable (self) -> & 'a mut crate :: W < REG > { self . variant (WRITELOCK_ACTIVE_A :: WRITELOCK_ACTIVE_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn writelock_active_enable (self) -> & 'a mut crate :: W < REG > { self . variant (WRITELOCK_ACTIVE_A :: WRITELOCK_ACTIVE_ENABLE) } } impl R { # [doc = "Bit 0 - ACTIVE controls whether critical SYSCTL registers are write protected or not."] # [inline (always)] pub fn writelock_active (& self) -> WRITELOCK_ACTIVE_R { WRITELOCK_ACTIVE_R :: new ((self . bits & 1) != 0) } } impl W { # [doc = "Bit 0 - ACTIVE controls whether critical SYSCTL registers are write protected or not."] # [inline (always)] # [must_use] pub fn writelock_active (& mut self) -> WRITELOCK_ACTIVE_W < WRITELOCK_SPEC , 0 > { WRITELOCK_ACTIVE_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "SYSCTL register write lockout\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`writelock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writelock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct WRITELOCK_SPEC ; impl crate :: RegisterSpec for WRITELOCK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`writelock::R`](R) reader structure"] impl crate :: Readable for WRITELOCK_SPEC { } # [doc = "`write(|w| ..)` method takes [`writelock::W`](W) writer structure"] impl crate :: Writable for WRITELOCK_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets WRITELOCK to value 0"] impl crate :: Resettable for WRITELOCK_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }