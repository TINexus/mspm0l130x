# [doc = "Register `IMASK` reader"] pub type R = crate :: R < IMASK_SPEC > ; # [doc = "Register `IMASK` writer"] pub type W = crate :: W < IMASK_SPEC > ; # [doc = "Field `IMASK_INTTIM` reader - Interval Timer Interrupt."] pub type IMASK_INTTIM_R = crate :: BitReader < IMASK_INTTIM_A > ; # [doc = "Interval Timer Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IMASK_INTTIM_A { # [doc = "0: CLR"] IMASK_INTTIM_CLR = 0 , # [doc = "1: SET"] IMASK_INTTIM_SET = 1 , } impl From < IMASK_INTTIM_A > for bool { # [inline (always)] fn from (variant : IMASK_INTTIM_A) -> Self { variant as u8 != 0 } } impl IMASK_INTTIM_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IMASK_INTTIM_A { match self . bits { false => IMASK_INTTIM_A :: IMASK_INTTIM_CLR , true => IMASK_INTTIM_A :: IMASK_INTTIM_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_imask_inttim_clr (& self) -> bool { * self == IMASK_INTTIM_A :: IMASK_INTTIM_CLR } # [doc = "SET"] # [inline (always)] pub fn is_imask_inttim_set (& self) -> bool { * self == IMASK_INTTIM_A :: IMASK_INTTIM_SET } } # [doc = "Field `IMASK_INTTIM` writer - Interval Timer Interrupt."] pub type IMASK_INTTIM_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IMASK_INTTIM_A > ; impl < 'a , REG , const O : u8 > IMASK_INTTIM_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn imask_inttim_clr (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_INTTIM_A :: IMASK_INTTIM_CLR) } # [doc = "SET"] # [inline (always)] pub fn imask_inttim_set (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_INTTIM_A :: IMASK_INTTIM_SET) } } impl R { # [doc = "Bit 0 - Interval Timer Interrupt."] # [inline (always)] pub fn imask_inttim (& self) -> IMASK_INTTIM_R { IMASK_INTTIM_R :: new ((self . bits & 1) != 0) } } impl W { # [doc = "Bit 0 - Interval Timer Interrupt."] # [inline (always)] # [must_use] pub fn imask_inttim (& mut self) -> IMASK_INTTIM_W < IMASK_SPEC , 0 > { IMASK_INTTIM_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct IMASK_SPEC ; impl crate :: RegisterSpec for IMASK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`imask::R`](R) reader structure"] impl crate :: Readable for IMASK_SPEC { } # [doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"] impl crate :: Writable for IMASK_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets IMASK to value 0"] impl crate :: Resettable for IMASK_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }