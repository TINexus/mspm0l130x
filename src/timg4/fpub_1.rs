# [doc = "Register `FPUB_1` reader"] pub type R = crate :: R < FPUB_1_SPEC > ; # [doc = "Register `FPUB_1` writer"] pub type W = crate :: W < FPUB_1_SPEC > ; # [doc = "Field `FPUB_1_CHANID` reader - 0 = disconnected. 1-15 = connected to channelID = CHANID."] pub type FPUB_1_CHANID_R = crate :: FieldReader < FPUB_1_CHANID_A > ; # [doc = "0 = disconnected. 1-15 = connected to channelID = CHANID.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum FPUB_1_CHANID_A { # [doc = "0: UNCONNECTED"] FPUB_1_CHANID_UNCONNECTED = 0 , } impl From < FPUB_1_CHANID_A > for u8 { # [inline (always)] fn from (variant : FPUB_1_CHANID_A) -> Self { variant as _ } } impl crate :: FieldSpec for FPUB_1_CHANID_A { type Ux = u8 ; } impl FPUB_1_CHANID_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < FPUB_1_CHANID_A > { match self . bits { 0 => Some (FPUB_1_CHANID_A :: FPUB_1_CHANID_UNCONNECTED) , _ => None , } } # [doc = "UNCONNECTED"] # [inline (always)] pub fn is_fpub_1_chanid_unconnected (& self) -> bool { * self == FPUB_1_CHANID_A :: FPUB_1_CHANID_UNCONNECTED } } # [doc = "Field `FPUB_1_CHANID` writer - 0 = disconnected. 1-15 = connected to channelID = CHANID."] pub type FPUB_1_CHANID_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 2 , O , FPUB_1_CHANID_A > ; impl < 'a , REG , const O : u8 > FPUB_1_CHANID_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "UNCONNECTED"] # [inline (always)] pub fn fpub_1_chanid_unconnected (self) -> & 'a mut crate :: W < REG > { self . variant (FPUB_1_CHANID_A :: FPUB_1_CHANID_UNCONNECTED) } } impl R { # [doc = "Bits 0:1 - 0 = disconnected. 1-15 = connected to channelID = CHANID."] # [inline (always)] pub fn fpub_1_chanid (& self) -> FPUB_1_CHANID_R { FPUB_1_CHANID_R :: new ((self . bits & 3) as u8) } } impl W { # [doc = "Bits 0:1 - 0 = disconnected. 1-15 = connected to channelID = CHANID."] # [inline (always)] # [must_use] pub fn fpub_1_chanid (& mut self) -> FPUB_1_CHANID_W < FPUB_1_SPEC , 0 > { FPUB_1_CHANID_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Publisher Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpub_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpub_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct FPUB_1_SPEC ; impl crate :: RegisterSpec for FPUB_1_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`fpub_1::R`](R) reader structure"] impl crate :: Readable for FPUB_1_SPEC { } # [doc = "`write(|w| ..)` method takes [`fpub_1::W`](W) writer structure"] impl crate :: Writable for FPUB_1_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets FPUB_1 to value 0"] impl crate :: Resettable for FPUB_1_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }