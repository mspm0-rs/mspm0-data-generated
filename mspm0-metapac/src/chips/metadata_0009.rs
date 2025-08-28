
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        kind: "adc",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "1",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "12",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "13",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "14",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "15",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "2",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "22",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "25",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "26",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "3",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "4",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "7",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "8",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "COMP0",
        kind: "comp",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "DAC_OUT",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "IN0+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "IN1+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "IN1-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IN3+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "OUT",
                pf: Some(14u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "OUT",
                pf: Some(14u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "OUT",
                pf: Some(14u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "CPUSS",
        kind: "cpuss",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "CRC",
        kind: "crc",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "DEBUGSS",
        kind: "debugss",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "SWCLK",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "SWDIO",
                pf: Some(2u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "DMA",
        kind: "dma",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
    },
    Peripheral {
        name: "EVENT",
        kind: "event",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "FLASHCTL",
        kind: "flashctl",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "GPIOA",
        kind: "gpio",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "PA0",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "PA1",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "PA1",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "PA10",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "PA11",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "PA15",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "PA16",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "PA17",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "PA18",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "PA19",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "PA2",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "PA20",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "PA21",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "PA22",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "PA23",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "PA24",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "PA25",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "PA26",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "PA3",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "PA30",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "PA30",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "PA4",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "PA9",
                pf: Some(1u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "GPIOB",
        kind: "gpio",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "I2C0",
        kind: "i2c",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SCL",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "SCL",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "SCL",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SCL",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "SCL",
                pf: Some(14u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "SDA",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "SDA",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "SDA",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "SDA",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "SDA",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "SDA",
                pf: Some(13u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "SDA",
                pf: Some(13u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SDA",
                pf: Some(14u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "I2C1",
        kind: "i2c",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                pf: Some(8u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "IOMUX",
        kind: "iomux",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "LFSS",
        kind: "lfss",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "RTC_OUT",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "RTC_OUT",
                pf: Some(12u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "SPI0",
        kind: "spi",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CS0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CS0",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CS0",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CS0",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CS1_POCI1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CS1_POCI1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CS1_POCI1",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CS2_POCI2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CS2_POCI2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CS2_POCI2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CS3_CD_POCI3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CS3_CD_POCI3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CS3_CD_POCI3",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CS3_CD_POCI3",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "PICO",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "PICO",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "PICO",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "PICO",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "PICO",
                pf: Some(13u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "PICO",
                pf: Some(14u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "POCI",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "POCI",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "POCI",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "POCI",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "POCI",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "SCLK",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "SCLK",
                pf: Some(10u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "TIMA0",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CCP0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP0",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP0",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP0",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP0_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CCP0_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CCP0_CMPL",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CCP0_CMPL",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP0_CMPL",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP1_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP1_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP1_CMPL",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CCP1_CMPL",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CCP2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP2",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CCP2",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CCP2_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "CCP2_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CCP2_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP2_CMPL",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP3",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP3_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP3_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP3_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP3_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "FAULT0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "FAULT0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "FAULT0",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "FAULT1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "FAULT1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "FAULT1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "FAULT1",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "FAULT1",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "FAULT2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "FAULT2",
                pf: Some(13u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "FAULT2",
                pf: Some(13u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG1",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA21",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP0",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP1",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CCP1",
                pf: Some(13u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG14",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA19",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CCP0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CCP0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CCP1",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "CCP2",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "CCP2",
                pf: Some(12u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG2",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA17",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CCP0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "CCP1",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CCP1",
                pf: Some(11u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG8",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "CCP0",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CCP0",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP1",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP1",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP1",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CCP1",
                pf: Some(13u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IDX",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "IDX",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IDX",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IDX",
                pf: Some(7u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "UART0",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA19",
                signal: "CTS",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CTS",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CTS",
                pf: Some(13u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "RTS",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "RX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RX",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RX",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "TX",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "TX",
                pf: Some(12u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "TX",
                pf: Some(12u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "UART1",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA21",
                signal: "CTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CTS",
                pf: Some(14u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "RTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "RTS",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                pf: Some(13u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RX",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "RX",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "RX",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RX",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "RX",
                pf: Some(13u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "TX",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "TX",
                pf: Some(11u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "UART2",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA21",
                signal: "CTS",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CTS",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CTS",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "CTS",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RTS",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "RTS",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "RX",
                pf: Some(13u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "TX",
                pf: Some(13u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "VREF",
        kind: "vref",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA23",
                signal: "VREF+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "VREF-",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "WUC",
        kind: "wuc",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "WWDT0",
        kind: "wwdt",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "SYSCTL",
        number: 0,
    },
    Interrupt {
        name: "DEBUGSS",
        number: 1,
    },
    Interrupt {
        name: "TIMG8",
        number: 2,
    },
    Interrupt {
        name: "UART1",
        number: 3,
    },
    Interrupt {
        name: "ADC0",
        number: 4,
    },
    Interrupt {
        name: "COMP0",
        number: 7,
    },
    Interrupt {
        name: "UART2",
        number: 8,
    },
    Interrupt {
        name: "SPI0",
        number: 9,
    },
    Interrupt {
        name: "UART0",
        number: 15,
    },
    Interrupt {
        name: "TIMG14",
        number: 16,
    },
    Interrupt {
        name: "TIMG2",
        number: 17,
    },
    Interrupt {
        name: "TIMA0",
        number: 18,
    },
    Interrupt {
        name: "TIMG1",
        number: 19,
    },
    Interrupt {
        name: "GPIOA",
        number: 22,
    },
    Interrupt {
        name: "GPIOB",
        number: 23,
    },
    Interrupt {
        name: "I2C0",
        number: 24,
    },
    Interrupt {
        name: "I2C1",
        number: 25,
    },
    Interrupt {
        name: "FLASHCTL",
        number: 27,
    },
    Interrupt {
        name: "WWDT0",
        number: 29,
    },
    Interrupt {
        name: "LFSS",
        number: 30,
    },
    Interrupt {
        name: "DMA",
        number: 31,
    },
];
pub(crate) static INTERRUPT_GROUPS: &[InterruptGroup] = &[];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        number: 0,
        full: true,
    },
    DmaChannel {
        number: 1,
        full: true,
    },
    DmaChannel {
        number: 2,
        full: false,
    },
];
pub(crate) static PINS: &[Pin] = &[
    Pin {
        pin: "PA1",
        pincm: 2u8,
    },
    Pin {
        pin: "PA11",
        pincm: 16u8,
    },
    Pin {
        pin: "PA15",
        pincm: 27u8,
    },
    Pin {
        pin: "PA16",
        pincm: 28u8,
    },
    Pin {
        pin: "PA17",
        pincm: 29u8,
    },
    Pin {
        pin: "PA18",
        pincm: 30u8,
    },
    Pin {
        pin: "PA19",
        pincm: 32u8,
    },
    Pin {
        pin: "PA20",
        pincm: 33u8,
    },
    Pin {
        pin: "PA21",
        pincm: 37u8,
    },
    Pin {
        pin: "PA22",
        pincm: 38u8,
    },
    Pin {
        pin: "PA23",
        pincm: 41u8,
    },
    Pin {
        pin: "PA24",
        pincm: 42u8,
    },
    Pin {
        pin: "PA25",
        pincm: 43u8,
    },
    Pin {
        pin: "PA26",
        pincm: 44u8,
    },
    Pin {
        pin: "PA30",
        pincm: 46u8,
    },
    Pin {
        pin: "PA0",
        pincm: 1u8,
    },
    Pin {
        pin: "PA2",
        pincm: 5u8,
    },
    Pin {
        pin: "PA3",
        pincm: 6u8,
    },
    Pin {
        pin: "PA4",
        pincm: 7u8,
    },
    Pin {
        pin: "PA9",
        pincm: 14u8,
    },
    Pin {
        pin: "PA10",
        pincm: 15u8,
    },
];
