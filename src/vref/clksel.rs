# [doc = "Register `CLKSEL` reader"] pub type R = crate :: R < CLKSEL_SPEC > ; # [doc = "Register `CLKSEL` writer"] pub type W = crate :: W < CLKSEL_SPEC > ; # [doc = "Field `CLKSEL_LFCLK_SEL` reader - Selects LFCLK as clock source if enabled"] pub type CLKSEL_LFCLK_SEL_R = crate :: BitReader ; # [doc = "Field `CLKSEL_LFCLK_SEL` writer - Selects LFCLK as clock source if enabled"] pub type CLKSEL_LFCLK_SEL_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O > ; # [doc = "Field `CLKSEL_MFCLK_SEL` reader - Selects MFCLK as clock source if enabled"] pub type CLKSEL_MFCLK_SEL_R = crate :: BitReader ; # [doc = "Field `CLKSEL_MFCLK_SEL` writer - Selects MFCLK as clock source if enabled"] pub type CLKSEL_MFCLK_SEL_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O > ; # [doc = "Field `CLKSEL_BUSCLK_SEL` reader - Selects BUSCLK as clock source if enabled"] pub type CLKSEL_BUSCLK_SEL_R = crate :: BitReader ; # [doc = "Field `CLKSEL_BUSCLK_SEL` writer - Selects BUSCLK as clock source if enabled"] pub type CLKSEL_BUSCLK_SEL_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O > ; impl R { # [doc = "Bit 1 - Selects LFCLK as clock source if enabled"] # [inline (always)] pub fn clksel_lfclk_sel (& self) -> CLKSEL_LFCLK_SEL_R { CLKSEL_LFCLK_SEL_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Selects MFCLK as clock source if enabled"] # [inline (always)] pub fn clksel_mfclk_sel (& self) -> CLKSEL_MFCLK_SEL_R { CLKSEL_MFCLK_SEL_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - Selects BUSCLK as clock source if enabled"] # [inline (always)] pub fn clksel_busclk_sel (& self) -> CLKSEL_BUSCLK_SEL_R { CLKSEL_BUSCLK_SEL_R :: new (((self . bits >> 3) & 1) != 0) } } impl W { # [doc = "Bit 1 - Selects LFCLK as clock source if enabled"] # [inline (always)] # [must_use] pub fn clksel_lfclk_sel (& mut self) -> CLKSEL_LFCLK_SEL_W < CLKSEL_SPEC , 1 > { CLKSEL_LFCLK_SEL_W :: new (self) } # [doc = "Bit 2 - Selects MFCLK as clock source if enabled"] # [inline (always)] # [must_use] pub fn clksel_mfclk_sel (& mut self) -> CLKSEL_MFCLK_SEL_W < CLKSEL_SPEC , 2 > { CLKSEL_MFCLK_SEL_W :: new (self) } # [doc = "Bit 3 - Selects BUSCLK as clock source if enabled"] # [inline (always)] # [must_use] pub fn clksel_busclk_sel (& mut self) -> CLKSEL_BUSCLK_SEL_W < CLKSEL_SPEC , 3 > { CLKSEL_BUSCLK_SEL_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Clock Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CLKSEL_SPEC ; impl crate :: RegisterSpec for CLKSEL_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`clksel::R`](R) reader structure"] impl crate :: Readable for CLKSEL_SPEC { } # [doc = "`write(|w| ..)` method takes [`clksel::W`](W) writer structure"] impl crate :: Writable for CLKSEL_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CLKSEL to value 0"] impl crate :: Resettable for CLKSEL_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }