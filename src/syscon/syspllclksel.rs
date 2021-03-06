#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSPLLCLKSEL {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "FRO 12 MHz (fro_12m)"]
    FRO_12_MHZ,
    #[doc = "CLKIN (clk_in)"]
    CLKIN,
    #[doc = "Watchdog oscillator (wdt_clk)"]
    WATCHDOG_OSCILLATOR,
    #[doc = "RTC oscillator 32 kHz output (32k_clk)"]
    RTC_OSC_OUTPUT,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE,
}
impl crate::ToBits<u8> for SELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SELR::FRO_12_MHZ => 0,
            SELR::CLKIN => 1,
            SELR::WATCHDOG_OSCILLATOR => 2,
            SELR::RTC_OSC_OUTPUT => 3,
            SELR::NONE => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEL_R = crate::FR<u8, SELR>;
impl SEL_R {
    #[doc = "Checks if the value of the field is `FRO_12_MHZ`"]
    #[inline(always)]
    pub fn is_fro_12_mhz(&self) -> bool {
        *self == SELR::FRO_12_MHZ
    }
    #[doc = "Checks if the value of the field is `CLKIN`"]
    #[inline(always)]
    pub fn is_clkin(&self) -> bool {
        *self == SELR::CLKIN
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        *self == SELR::WATCHDOG_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `RTC_OSC_OUTPUT`"]
    #[inline(always)]
    pub fn is_rtc_osc_output(&self) -> bool {
        *self == SELR::RTC_OSC_OUTPUT
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SELR::NONE
    }
}
#[doc = "Values that can be written to the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELW {
    #[doc = "FRO 12 MHz (fro_12m)"]
    FRO_12_MHZ,
    #[doc = "CLKIN (clk_in)"]
    CLKIN,
    #[doc = "Watchdog oscillator (wdt_clk)"]
    WATCHDOG_OSCILLATOR,
    #[doc = "RTC oscillator 32 kHz output (32k_clk)"]
    RTC_OSC_OUTPUT,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::FRO_12_MHZ => 0,
            SELW::CLKIN => 1,
            SELW::WATCHDOG_OSCILLATOR => 2,
            SELW::RTC_OSC_OUTPUT => 3,
            SELW::NONE => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FRO 12 MHz (fro_12m)"]
    #[inline(always)]
    pub fn fro_12_mhz(self) -> &'a mut W {
        self.variant(SELW::FRO_12_MHZ)
    }
    #[doc = "CLKIN (clk_in)"]
    #[inline(always)]
    pub fn clkin(self) -> &'a mut W {
        self.variant(SELW::CLKIN)
    }
    #[doc = "Watchdog oscillator (wdt_clk)"]
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut W {
        self.variant(SELW::WATCHDOG_OSCILLATOR)
    }
    #[doc = "RTC oscillator 32 kHz output (32k_clk)"]
    #[inline(always)]
    pub fn rtc_osc_output(self) -> &'a mut W {
        self.variant(SELW::RTC_OSC_OUTPUT)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SELW::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - System PLL clock source selection."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits() & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - System PLL clock source selection."]
    #[inline(always)]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
