#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TXEFC {
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
#[doc = r"Reader of the field"]
pub type EFSA_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _EFSAW<'a> {
    w: &'a mut W,
}
impl<'a> _EFSAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EFS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _EFSW<'a> {
    w: &'a mut W,
}
impl<'a> _EFSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EFWM_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _EFWMW<'a> {
    w: &'a mut W,
}
impl<'a> _EFWMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 2:15 - Event FIFO start address."]
    #[inline(always)]
    pub fn efsa(&self) -> EFSA_R {
        EFSA_R::new(((self.bits() >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Event FIFO size 0 = Tx event FIFO disabled."]
    #[inline(always)]
    pub fn efs(&self) -> EFS_R {
        EFS_R::new(((self.bits() >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Event FIFO watermark 0 = Watermark interrupt disabled."]
    #[inline(always)]
    pub fn efwm(&self) -> EFWM_R {
        EFWM_R::new(((self.bits() >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 2:15 - Event FIFO start address."]
    #[inline(always)]
    pub fn efsa(&mut self) -> _EFSAW {
        _EFSAW { w: self }
    }
    #[doc = "Bits 16:21 - Event FIFO size 0 = Tx event FIFO disabled."]
    #[inline(always)]
    pub fn efs(&mut self) -> _EFSW {
        _EFSW { w: self }
    }
    #[doc = "Bits 24:29 - Event FIFO watermark 0 = Watermark interrupt disabled."]
    #[inline(always)]
    pub fn efwm(&mut self) -> _EFWMW {
        _EFWMW { w: self }
    }
}
