# [doc = "Register `CC_01[%s]` reader"] pub type R = crate :: R < CC_01_SPEC > ; # [doc = "Register `CC_01[%s]` writer"] pub type W = crate :: W < CC_01_SPEC > ; # [doc = "Field `CC_01_CCVAL` reader - Capture or compare value"] pub type CC_01_CCVAL_R = crate :: FieldReader < u16 > ; # [doc = "Field `CC_01_CCVAL` writer - Capture or compare value"] pub type CC_01_CCVAL_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 16 , O , u16 > ; impl R { # [doc = "Bits 0:15 - Capture or compare value"] # [inline (always)] pub fn cc_01_ccval (& self) -> CC_01_CCVAL_R { CC_01_CCVAL_R :: new ((self . bits & 0xffff) as u16) } } impl W { # [doc = "Bits 0:15 - Capture or compare value"] # [inline (always)] # [must_use] pub fn cc_01_ccval (& mut self) -> CC_01_CCVAL_W < CC_01_SPEC , 0 > { CC_01_CCVAL_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CC_01_SPEC ; impl crate :: RegisterSpec for CC_01_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`cc_01::R`](R) reader structure"] impl crate :: Readable for CC_01_SPEC { } # [doc = "`write(|w| ..)` method takes [`cc_01::W`](W) writer structure"] impl crate :: Writable for CC_01_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CC_01[%s]
to value 0"] impl crate :: Resettable for CC_01_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }