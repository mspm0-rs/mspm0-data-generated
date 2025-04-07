// This file is autogenerated by mspm0-metapac-gen: do not edit by hand.
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u32 = 2;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    GROUP0 = 0,
    GROUP1 = 1,
    TIMG1 = 2,
    ADC0 = 4,
    SPI0 = 9,
    UART1 = 13,
    UART0 = 15,
    TIMG0 = 16,
    TIMG2 = 18,
    TIMG4 = 20,
    I2C0 = 24,
    I2C1 = 25,
    DMA = 31,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[repr(u8)]
pub enum Group0 {
    WWDT0 = 0,
    DEBUGSS = 2,
    FLASHCTL = 3,
    SYSCTL = 6,
}
impl TryFrom<u8> for Group0 {
    type Error = ();
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::WWDT0),
            2 => Ok(Self::DEBUGSS),
            3 => Ok(Self::FLASHCTL),
            6 => Ok(Self::SYSCTL),
            _ => Err(()),
        }
    }
}
#[repr(u8)]
pub enum Group1 {
    GPIOA = 0,
    COMP0 = 2,
}
impl TryFrom<u8> for Group1 {
    type Error = ();
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GPIOA),
            2 => Ok(Self::COMP0),
            _ => Err(()),
        }
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    extern "C" {
        fn NonMaskableInt();
        fn HardFault();
        fn SVCall();
        fn PendSV();
        fn SysTick();
        fn GROUP0();
        fn GROUP1();
        fn TIMG1();
        fn ADC0();
        fn SPI0();
        fn UART1();
        fn UART0();
        fn TIMG0();
        fn TIMG2();
        fn TIMG4();
        fn I2C0();
        fn I2C1();
        fn DMA();
    }
    #[repr(C)]
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 32] = [
        Vector { _handler: GROUP0 },
        Vector { _handler: GROUP1 },
        Vector { _handler: TIMG1 },
        Vector { _reserved: 0 },
        Vector { _handler: ADC0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: SPI0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: UART1 },
        Vector { _reserved: 0 },
        Vector { _handler: UART0 },
        Vector { _handler: TIMG0 },
        Vector { _reserved: 0 },
        Vector { _handler: TIMG2 },
        Vector { _reserved: 0 },
        Vector { _handler: TIMG4 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: I2C0 },
        Vector { _handler: I2C1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DMA },
    ];
}
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/cpuss.rs"]
pub mod cpuss;
#[path = "../../peripherals/dma.rs"]
pub mod dma;
#[path = "../../peripherals/gpio.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c.rs"]
pub mod i2c;
#[path = "../../peripherals/iomux.rs"]
pub mod iomux;
#[path = "../../peripherals/sysctl_l110x_l130x_l134x.rs"]
pub mod sysctl;
#[path = "../../peripherals/tim.rs"]
pub mod tim;
#[path = "../../peripherals/uart.rs"]
pub mod uart;
#[doc = "Address: 1073758208"]
pub const ADC0: () = ();
#[doc = "Address: 1073774592"]
pub const COMP0: () = ();
pub const CPUSS: cpuss::Cpuss = unsafe { cpuss::Cpuss::from_ptr(1077936128 as *mut _) };
#[doc = "Address: 1078198272"]
pub const CRC: () = ();
#[doc = "Address: 1074556928"]
pub const DEBUGSS: () = ();
pub const DMA: dma::Dma = unsafe { dma::Dma::from_ptr(1078108160 as *mut _) };
#[doc = "Address: 1074565120"]
pub const EVENT: () = ();
#[doc = "Address: 1074581504"]
pub const FLASHCTL: () = ();
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(1074397184 as *mut _) };
pub const I2C0: i2c::I2c = unsafe { i2c::I2c::from_ptr(1074724864 as *mut _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(1074733056 as *mut _) };
pub const IOMUX: iomux::Iomux = unsafe { iomux::Iomux::from_ptr(1078099968 as *mut _) };
#[doc = "Address: 1073872896"]
pub const OPA0: () = ();
#[doc = "Address: 1073881088"]
pub const OPA1: () = ();
#[doc = "Address: 1078362112"]
pub const SPI0: () = ();
pub const SYSCTL: sysctl::Sysctl = unsafe { sysctl::Sysctl::from_ptr(1074458624 as *mut _) };
pub const TIMG0: tim::Tim = unsafe { tim::Tim::from_ptr(1074282496 as *mut _) };
pub const TIMG1: tim::Tim = unsafe { tim::Tim::from_ptr(1074290688 as *mut _) };
pub const TIMG2: tim::Tim = unsafe { tim::Tim::from_ptr(1074298880 as *mut _) };
pub const TIMG4: tim::Tim = unsafe { tim::Tim::from_ptr(1074315264 as *mut _) };
pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(1074823168 as *mut _) };
pub const UART1: uart::Uart = unsafe { uart::Uart::from_ptr(1074790400 as *mut _) };
#[doc = "Address: 1073938432"]
pub const VREF: () = ();
#[doc = "Address: 1078083584"]
pub const WUC: () = ();
#[doc = "Address: 1074266112"]
pub const WWDT0: () = ();
