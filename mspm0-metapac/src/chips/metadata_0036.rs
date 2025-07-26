
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        kind: "adc",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA27",
                signal: "0",
                pf: Some(0u8),
            },
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
                pin: "PA14",
                signal: "16",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "17",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "18",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "2",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "3",
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
    },
    Peripheral {
        name: "AESADV",
        kind: "aesadv",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd1,
    },
    Peripheral {
        name: "COMP0",
        kind: "comp",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "IN0+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "IN0-",
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
                pin: "PA14",
                signal: "IN2+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "IN2-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IN3+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "OUT",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "OUT",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "OUT",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "OUT",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "OUT",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "OUT",
                pf: Some(9u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "CPUSS",
        kind: "cpuss",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd1,
    },
    Peripheral {
        name: "CRCP0",
        kind: "crc",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd1,
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
    },
    Peripheral {
        name: "DMA",
        kind: "dma",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd1,
    },
    Peripheral {
        name: "EVENT",
        kind: "event",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "FLASHCTL",
        kind: "flashctl",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "GPAMP",
        kind: "gpamp",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
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
                pin: "PA12",
                signal: "PA12",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "PA13",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "PA14",
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
                pin: "PA27",
                signal: "PA27",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "PA3",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "PA4",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "PA5",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PA6",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "PA7",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "PA8",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "PA9",
                pf: Some(1u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "GPIOB",
        kind: "gpio",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "GPIOC",
        kind: "gpio",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd0,
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
                pin: "PA8",
                signal: "SDA",
                pf: Some(4u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
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
                pin: "PA6",
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
                pin: "PA5",
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
    },
    Peripheral {
        name: "I2C2",
        kind: "i2c",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA23",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "SDA",
                pf: Some(6u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "IOMUX",
        kind: "iomux",
        version: Some("v1"),
        pins: &[],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "KEYSTORECTL",
        kind: "keystorectl",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "LFSS",
        kind: "lfss",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "RTC_OUT",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "RTC_OUT",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RTC_OUT",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "TIO0",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Backup,
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
                pin: "PA8",
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
                pin: "PA3",
                signal: "CS1_POCI1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CS1_POCI1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CS1_POCI1",
                pf: Some(9u8),
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
                pin: "PA14",
                signal: "CS2_POCI2",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CS2_POCI2",
                pf: Some(9u8),
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
                pin: "PA8",
                signal: "CS3_CD_POCI3",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CS3_CD_POCI3",
                pf: Some(9u8),
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
                pin: "PA14",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "POCI",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SCLK",
                pf: Some(3u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
    },
    Peripheral {
        name: "SPI1",
        kind: "spi",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CS0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CS0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CS1_POCI1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CS1_POCI1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CS1_POCI1",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CS1_POCI1",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CS2_POCI2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CS2_POCI2",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CS2_POCI2",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CS3_CD_POCI3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "SCLK",
                pf: Some(3u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
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
                pin: "PA8",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP0",
                pf: Some(11u8),
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
                pin: "PA7",
                signal: "CCP1",
                pf: Some(8u8),
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
                pin: "PA7",
                signal: "CCP2",
                pf: Some(5u8),
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
                pin: "PA6",
                signal: "CCP2_CMPL",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CCP3",
                pf: Some(5u8),
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
                pin: "PA13",
                signal: "CCP3_CMPL",
                pf: Some(5u8),
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
                pin: "PA8",
                signal: "FAULT0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "FAULT0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "FAULT0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "FAULT0",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "FAULT1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "FAULT1",
                pf: Some(8u8),
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
                pin: "PA1",
                signal: "FAULT2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "FAULT2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "FAULT2",
                pf: Some(6u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "TIMG0",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "CCP0",
                pf: Some(5u8),
            },
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
                pin: "PA12",
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
                pin: "PA6",
                signal: "CCP1",
                pf: Some(5u8),
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
                pin: "PA13",
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
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "TIMG12",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CCP0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CCP0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CCP0",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CCP0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP1",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "CCP1",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP1",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CCP1",
                pf: Some(9u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "TIMG4",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA21",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CCP0",
                pf: Some(7u8),
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
                pin: "PA6",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP1",
                pf: Some(10u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "TIMG5",
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
                pin: "PA27",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CCP1",
                pf: Some(9u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
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
                pin: "PA5",
                signal: "CCP0",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA7",
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
                pin: "PA6",
                signal: "CCP1",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CCP1",
                pf: Some(4u8),
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
                pin: "PA1",
                signal: "IDX",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA7",
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
    },
    Peripheral {
        name: "TRNG",
        kind: "trng",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd1,
    },
    Peripheral {
        name: "UART0",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CTS",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CTS",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RTS",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RTS",
                pf: Some(11u8),
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
                pin: "PA0",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "TX",
                pf: Some(2u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
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
                pin: "PA12",
                signal: "CTS",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RTS",
                pf: Some(11u8),
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
                pin: "PA4",
                signal: "RX",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RX",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "TX",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "TX",
                pf: Some(11u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
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
                pin: "PA12",
                signal: "CTS",
                pf: Some(10u8),
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
                pin: "PA14",
                signal: "RX",
                pf: Some(10u8),
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
                pin: "PA13",
                signal: "TX",
                pf: Some(10u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "UART3",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CTS",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RTS",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RX",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "TX",
                pf: Some(9u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "UART4",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "CTS",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "RTS",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "RTS",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RTS",
                pf: Some(10u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
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
    },
    Peripheral {
        name: "WUC",
        kind: "wuc",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "WWDT0",
        kind: "wwdt",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "GROUP0",
        number: 0,
    },
    Interrupt {
        name: "GROUP1",
        number: 1,
    },
    Interrupt {
        name: "TIMG12",
        number: 2,
    },
    Interrupt {
        name: "UART4",
        number: 3,
    },
    Interrupt {
        name: "ADC0",
        number: 4,
    },
    Interrupt {
        name: "SPI0",
        number: 9,
    },
    Interrupt {
        name: "SPI1",
        number: 10,
    },
    Interrupt {
        name: "UART2",
        number: 13,
    },
    Interrupt {
        name: "UART3",
        number: 14,
    },
    Interrupt {
        name: "UART0",
        number: 15,
    },
    Interrupt {
        name: "UART1",
        number: 16,
    },
    Interrupt {
        name: "TIMA0",
        number: 18,
    },
    Interrupt {
        name: "TIMG8",
        number: 20,
    },
    Interrupt {
        name: "TIMG0",
        number: 21,
    },
    Interrupt {
        name: "TIMG4",
        number: 22,
    },
    Interrupt {
        name: "TIMG5",
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
        name: "I2C2",
        number: 26,
    },
    Interrupt {
        name: "AESADV",
        number: 28,
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
pub(crate) static INTERRUPT_GROUPS: &[InterruptGroup] = &[
    InterruptGroup {
        name: "GROUP0",
        number: 0,
        interrupts: &[
            GroupInterrupt {
                name: "WWDT0",
                number: 0,
            },
            GroupInterrupt {
                name: "DEBUGSS",
                number: 2,
            },
            GroupInterrupt {
                name: "FLASHCTL",
                number: 3,
            },
            GroupInterrupt {
                name: "SYSCTL",
                number: 6,
            },
        ],
    },
    InterruptGroup {
        name: "GROUP1",
        number: 1,
        interrupts: &[
            GroupInterrupt {
                name: "GPIOA",
                number: 0,
            },
            GroupInterrupt {
                name: "GPIOB",
                number: 1,
            },
            GroupInterrupt {
                name: "COMP0",
                number: 2,
            },
            GroupInterrupt {
                name: "TRNG",
                number: 5,
            },
            GroupInterrupt {
                name: "GPIOC",
                number: 6,
            },
        ],
    },
];
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
        full: true,
    },
    DmaChannel {
        number: 3,
        full: false,
    },
    DmaChannel {
        number: 4,
        full: false,
    },
    DmaChannel {
        number: 5,
        full: false,
    },
    DmaChannel {
        number: 6,
        full: false,
    },
];
pub(crate) static PINS: &[Pin] = &[
    Pin {
        pin: "PA0",
        pincm: 1u8,
    },
    Pin {
        pin: "PA6",
        pincm: 11u8,
    },
    Pin {
        pin: "PA7",
        pincm: 14u8,
    },
    Pin {
        pin: "PA8",
        pincm: 19u8,
    },
    Pin {
        pin: "PA9",
        pincm: 20u8,
    },
    Pin {
        pin: "PA10",
        pincm: 25u8,
    },
    Pin {
        pin: "PA11",
        pincm: 26u8,
    },
    Pin {
        pin: "PA12",
        pincm: 38u8,
    },
    Pin {
        pin: "PA13",
        pincm: 39u8,
    },
    Pin {
        pin: "PA14",
        pincm: 40u8,
    },
    Pin {
        pin: "PA15",
        pincm: 41u8,
    },
    Pin {
        pin: "PA1",
        pincm: 2u8,
    },
    Pin {
        pin: "PA16",
        pincm: 42u8,
    },
    Pin {
        pin: "PA17",
        pincm: 49u8,
    },
    Pin {
        pin: "PA18",
        pincm: 50u8,
    },
    Pin {
        pin: "PA19",
        pincm: 51u8,
    },
    Pin {
        pin: "PA20",
        pincm: 52u8,
    },
    Pin {
        pin: "PA21",
        pincm: 56u8,
    },
    Pin {
        pin: "PA22",
        pincm: 57u8,
    },
    Pin {
        pin: "PA23",
        pincm: 67u8,
    },
    Pin {
        pin: "PA24",
        pincm: 68u8,
    },
    Pin {
        pin: "PA25",
        pincm: 69u8,
    },
    Pin {
        pin: "PA26",
        pincm: 73u8,
    },
    Pin {
        pin: "PA27",
        pincm: 74u8,
    },
    Pin {
        pin: "PA2",
        pincm: 7u8,
    },
    Pin {
        pin: "PA3",
        pincm: 8u8,
    },
    Pin {
        pin: "PA4",
        pincm: 9u8,
    },
    Pin {
        pin: "PA5",
        pincm: 10u8,
    },
];
