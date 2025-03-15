// This file is autogenerated by mspm0-metapac-gen: do not edit by hand.
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u32 = 2;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    GROUP0 = 0,
    GROUP1 = 1,
    TIMG8 = 2,
    UART3 = 3,
    ADC0 = 4,
    ADC1 = 5,
    CANFD0 = 6,
    DAC0 = 7,
    SPI0 = 9,
    SPI1 = 10,
    UART1 = 13,
    UART2 = 14,
    UART0 = 15,
    TIMG0 = 16,
    TIMG6 = 17,
    TIMA0 = 18,
    TIMA1 = 19,
    TIMG7 = 20,
    TIMG12 = 21,
    I2C0 = 24,
    I2C1 = 25,
    AES = 28,
    RTC = 30,
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
    WWDT1 = 1,
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
            1 => Ok(Self::WWDT1),
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
    GPIOB = 1,
    COMP0 = 2,
    COMP1 = 3,
    COMP2 = 4,
    TRNG = 5,
}
impl TryFrom<u8> for Group1 {
    type Error = ();
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GPIOA),
            1 => Ok(Self::GPIOB),
            2 => Ok(Self::COMP0),
            3 => Ok(Self::COMP1),
            4 => Ok(Self::COMP2),
            5 => Ok(Self::TRNG),
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
        fn TIMG8();
        fn UART3();
        fn ADC0();
        fn ADC1();
        fn CANFD0();
        fn DAC0();
        fn SPI0();
        fn SPI1();
        fn UART1();
        fn UART2();
        fn UART0();
        fn TIMG0();
        fn TIMG6();
        fn TIMA0();
        fn TIMA1();
        fn TIMG7();
        fn TIMG12();
        fn I2C0();
        fn I2C1();
        fn AES();
        fn RTC();
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
        Vector { _handler: TIMG8 },
        Vector { _handler: UART3 },
        Vector { _handler: ADC0 },
        Vector { _handler: ADC1 },
        Vector { _handler: CANFD0 },
        Vector { _handler: DAC0 },
        Vector { _reserved: 0 },
        Vector { _handler: SPI0 },
        Vector { _handler: SPI1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: UART1 },
        Vector { _handler: UART2 },
        Vector { _handler: UART0 },
        Vector { _handler: TIMG0 },
        Vector { _handler: TIMG6 },
        Vector { _handler: TIMA0 },
        Vector { _handler: TIMA1 },
        Vector { _handler: TIMG7 },
        Vector { _handler: TIMG12 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: I2C0 },
        Vector { _handler: I2C1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: AES },
        Vector { _reserved: 0 },
        Vector { _handler: RTC },
        Vector { _handler: DMA },
    ];
}
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/cpuss.rs"]
pub mod cpuss;
#[path = "../../peripherals/gpio.rs"]
pub mod gpio;
#[path = "../../peripherals/iomux.rs"]
pub mod iomux;
#[path = "../../peripherals/sysctl_g350x_g310x_g150x_g110x.rs"]
pub mod sysctl;
#[path = "../../peripherals/tim.rs"]
pub mod tim;
#[path = "../../peripherals/uart.rs"]
pub mod uart;
#[doc = "Address: 1073741824"]
pub const ADC0: () = ();
#[doc = "Address: 1073750016"]
pub const ADC1: () = ();
#[doc = "Address: 1079017472"]
pub const CANFD0: () = ();
#[doc = "Address: 1073774592"]
pub const COMP0: () = ();
#[doc = "Address: 1073782784"]
pub const COMP1: () = ();
#[doc = "Address: 1073790976"]
pub const COMP2: () = ();
pub const CPUSS: cpuss::Cpuss = unsafe { cpuss::Cpuss::from_ptr(1077936128 as *mut _) };
#[doc = "Address: 1073840128"]
pub const DAC0: () = ();
#[doc = "Address: 1074556928"]
pub const DEBUGSS: () = ();
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(1074397184 as *mut _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(1074405376 as *mut _) };
#[doc = "Address: 1074724864"]
pub const I2C0: () = ();
#[doc = "Address: 1074733056"]
pub const I2C1: () = ();
pub const IOMUX: iomux::Iomux = unsafe { iomux::Iomux::from_ptr(1078099968 as *mut _) };
#[doc = "Address: 1073872896"]
pub const OPA0: () = ();
#[doc = "Address: 1073881088"]
pub const OPA1: () = ();
#[doc = "Address: 1074348032"]
pub const RTC: () = ();
#[doc = "Address: 1078362112"]
pub const SPI0: () = ();
#[doc = "Address: 1078370304"]
pub const SPI1: () = ();
pub const SYSCTL: sysctl::Sysctl = unsafe { sysctl::Sysctl::from_ptr(1074458624 as *mut _) };
pub const TIMA0: tim::Tim = unsafe { tim::Tim::from_ptr(1082523648 as *mut _) };
pub const TIMA1: tim::Tim = unsafe { tim::Tim::from_ptr(1082531840 as *mut _) };
pub const TIMG0: tim::Tim = unsafe { tim::Tim::from_ptr(1074282496 as *mut _) };
pub const TIMG12: tim::Tim = unsafe { tim::Tim::from_ptr(1082589184 as *mut _) };
pub const TIMG6: tim::Tim = unsafe { tim::Tim::from_ptr(1082556416 as *mut _) };
pub const TIMG7: tim::Tim = unsafe { tim::Tim::from_ptr(1082564608 as *mut _) };
pub const TIMG8: tim::Tim = unsafe { tim::Tim::from_ptr(1074331648 as *mut _) };
pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(1074823168 as *mut _) };
pub const UART1: uart::Uart = unsafe { uart::Uart::from_ptr(1074790400 as *mut _) };
pub const UART2: uart::Uart = unsafe { uart::Uart::from_ptr(1074798592 as *mut _) };
pub const UART3: uart::Uart = unsafe { uart::Uart::from_ptr(1078984704 as *mut _) };
