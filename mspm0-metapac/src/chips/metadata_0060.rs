
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
                pin: "PB18",
                signal: "10",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "11",
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
                pin: "PB16",
                signal: "19",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "2",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "20",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "21",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "22",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "23",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "24",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "25",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "3",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "4",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "5",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "6",
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
            PeripheralPin {
                pin: "PB19",
                signal: "9",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "AESADV",
        kind: "aesadv",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
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
                pin: "PB19",
                signal: "OUT",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "OUT",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "OUT",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "OUT",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "OUT",
                pf: Some(5u8),
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
                pin: "PB8",
                signal: "OUT",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "OUT",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "OUT",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB26",
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
        name: "CRCP0",
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
        name: "GPAMP",
        kind: "gpamp",
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
                pin: "PA28",
                signal: "PA28",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "PA29",
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
        sys_fentries: None,
    },
    Peripheral {
        name: "GPIOB",
        kind: "gpio",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "PB0",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "PB1",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "PB10",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "PB11",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "PB12",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "PB13",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "PB14",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "PB15",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "PB16",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "PB17",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "PB18",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "PB19",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "PB2",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "PB20",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "PB21",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "PB22",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "PB23",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "PB24",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "PB25",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "PB26",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "PB27",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "PB28",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "PB29",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "PB3",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "PB30",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "PB31",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "PB4",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "PB5",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "PB6",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "PB7",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "PB8",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "PB9",
                pf: Some(1u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "GPIOC",
        kind: "gpio",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PC0",
                signal: "PC0",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "PC1",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "PC2",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "PC3",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "PC4",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "PC5",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "PC6",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "PC7",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "PC8",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "PC9",
                pf: Some(1u8),
            },
        ],
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
                pin: "PB0",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB21",
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
                pin: "PA28",
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
            PeripheralPin {
                pin: "PB1",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "SDA",
                pf: Some(9u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(8usize),
    },
    Peripheral {
        name: "I2C1",
        kind: "i2c",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA29",
                signal: "SCL",
                pf: Some(2u8),
            },
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
                pin: "PB2",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "SDA",
                pf: Some(2u8),
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
                pin: "PB3",
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
        sys_fentries: Some(8usize),
    },
    Peripheral {
        name: "I2C2",
        kind: "i2c",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PB28",
                signal: "SCL",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "SCL",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "SCL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SCL",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "SDA",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "SDA",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "SDA",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "SDA",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "SDA",
                pf: Some(7u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(8usize),
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
        name: "KEYSTORECTL",
        kind: "keystorectl",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "LCD",
        kind: "lcd",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "LCD0",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "LCD1",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "LCD10",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "LCD11",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "LCD12",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "LCD13",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "LCD14",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "LCD15",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "LCD16",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "LCD17",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "LCD18",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "LCD19",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "LCD2",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "LCD20",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "LCD21",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "LCD22",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "LCD23",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "LCD24",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "LCD25",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "LCD26",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "LCD27",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "LCD28",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "LCD29",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "LCD3",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "LCD30",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "LCD31",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "LCD32",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "LCD33",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "LCD34",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "LCD35",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "LCD36",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "LCD37",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "LCD38",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "LCD39",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "LCD4",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "LCD40",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "LCD41",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "LCD42",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "LCD43",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "LCD44",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "LCD45",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "LCD46",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "LCD47",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "LCD48",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "LCD49",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "LCD5",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "LCD50",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "LCD51",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "LCD52",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "LCD53",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "LCD54",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "LCD55",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "LCD56",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "LCD57",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "LCD58",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "LCD6",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "LCD7",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "LCD8",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "LCD9",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "LCDCAP0",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "LCDCAP1",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "LCDEN",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "LCDEN",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "LCDEN",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "LCDLFCLK",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "LCDLFCLK",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "LCDLFCLK",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "LCDSON",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "LCDSON",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "LCDSON",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "R13",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "R23",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "R24",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "R33",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
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
            PeripheralPin {
                pin: "PB2",
                signal: "TIO1",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "TIO2",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Backup,
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
                pin: "PA8",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PC7",
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
                pin: "PB24",
                signal: "CS1_POCI1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "CS1_POCI1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PC6",
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
                pin: "PB6",
                signal: "CS1_POCI1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CS2_POCI2",
                pf: Some(2u8),
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
                pin: "PB1",
                signal: "CS2_POCI2",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "CS2_POCI2",
                pf: Some(8u8),
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
                pin: "PB7",
                signal: "CS2_POCI2",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "CS3_CD_POCI3",
                pf: Some(2u8),
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
                pin: "PB0",
                signal: "CS3_CD_POCI3",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CS3_CD_POCI3",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "CS3_CD_POCI3",
                pf: Some(8u8),
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
                pin: "PB17",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "PICO",
                pf: Some(11u8),
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
                pin: "PB19",
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
            PeripheralPin {
                pin: "PB18",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCLK",
                pf: Some(11u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
        sys_fentries: Some(4usize),
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
                pin: "PB20",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CS0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CS0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CS0",
                pf: Some(7u8),
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
                pin: "PB27",
                signal: "CS1_POCI1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CS1_POCI1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CS1_POCI1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CS1_POCI1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CS1_POCI1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "CS1_POCI1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB24",
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
                pin: "PB0",
                signal: "CS2_POCI2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CS2_POCI2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CS2_POCI2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CS2_POCI2",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CS2_POCI2",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "CS2_POCI2",
                pf: Some(7u8),
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
                pin: "PB14",
                signal: "CS3_CD_POCI3",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CS3_CD_POCI3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CS3_CD_POCI3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CS3_CD_POCI3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CS3_CD_POCI3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CS3_CD_POCI3",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CS3_CD_POCI3",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB8",
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
                pin: "PB14",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "POCI",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB7",
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
            PeripheralPin {
                pin: "PB16",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SCLK",
                pf: Some(3u8),
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
                pin: "PB26",
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
                pin: "PB14",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CCP0",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP0",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "CCP0_CMPL",
                pf: Some(4u8),
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
                pin: "PB29",
                signal: "CCP0_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CCP0_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CCP0_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CCP0_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CCP1",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CCP1_CMPL",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CCP1_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CCP1_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "CCP1_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CCP1_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CCP1_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP1_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "CCP1_CMPL",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB12",
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
                pin: "PA7",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CCP2",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PC0",
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
                pin: "PB1",
                signal: "CCP2_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "CCP2_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CCP2_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PC1",
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
                pin: "PB13",
                signal: "CCP3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "CCP3",
                pf: Some(4u8),
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
                pin: "PB2",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "CCP3",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB26",
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
                pin: "PB27",
                signal: "CCP3_CMPL",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB3",
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
                pin: "PB23",
                signal: "FAULT0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "FAULT0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "FAULT0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "FAULT0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "FAULT0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA28",
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
                pin: "PB12",
                signal: "FAULT1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "FAULT1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "FAULT1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "FAULT1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "FAULT1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "FAULT1",
                pf: Some(6u8),
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
                pin: "PB30",
                signal: "FAULT2",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "FAULT2",
                pf: Some(4u8),
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
            PeripheralPin {
                pin: "PB25",
                signal: "FAULT2",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "FAULT2",
                pf: Some(8u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG0",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "CCP0",
                pf: Some(2u8),
            },
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
                pin: "PB0",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PC2",
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
                pin: "PB11",
                signal: "CCP1",
                pf: Some(2u8),
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
                pin: "PB1",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PC3",
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
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG12",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PB13",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CCP0",
                pf: Some(4u8),
            },
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
                pin: "PB2",
                signal: "CCP0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CCP0",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB24",
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
            PeripheralPin {
                pin: "PB3",
                signal: "CCP1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CCP1",
                pf: Some(10u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG4",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA29",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CCP0",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "CCP0",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP0",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CCP1",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "CCP1",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP1",
                pf: Some(10u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TIMG5",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PC8",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "CCP0",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB15",
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
                pin: "PC9",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "CCP1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB19",
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
                pin: "PA5",
                signal: "CCP0",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CCP0",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CCP0",
                pf: Some(5u8),
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
                pin: "PB11",
                signal: "CCP1",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CCP1",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "CCP1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CCP1",
                pf: Some(5u8),
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
                pin: "PB31",
                signal: "IDX",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IDX",
                pf: Some(4u8),
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
                pin: "PB14",
                signal: "IDX",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IDX",
                pf: Some(7u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "IDX",
                pf: Some(7u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: None,
    },
    Peripheral {
        name: "TRNG",
        kind: "trng",
        version: None,
        pins: &[],
        power_domain: PowerDomain::Pd1,
        sys_fentries: None,
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
                pin: "PB25",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CTS",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "CTS",
                pf: Some(7u8),
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
                pin: "PB26",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "RTS",
                pf: Some(7u8),
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
                pin: "PB1",
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
            PeripheralPin {
                pin: "PA28",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "TX",
                pf: Some(2u8),
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
                pin: "PB23",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CTS",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CTS",
                pf: Some(11u8),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                pf: Some(6u8),
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
                pin: "PB5",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "RX",
                pf: Some(6u8),
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
                pin: "PB4",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "TX",
                pf: Some(6u8),
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
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "UART2",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA30",
                signal: "CTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CTS",
                pf: Some(6u8),
            },
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
                pin: "PB19",
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
                pin: "PA29",
                signal: "RTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RTS",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RTS",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "RTS",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PB24",
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
                pin: "PB16",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                pf: Some(8u8),
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
                pin: "PB15",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "TX",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "TX",
                pf: Some(10u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
        sys_fentries: Some(4usize),
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
                pin: "PC4",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                pf: Some(4u8),
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
                pin: "PC5",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "RTS",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RTS",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "RTS",
                pf: Some(4u8),
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
                pin: "PB13",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PC7",
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
                pin: "PB12",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PC6",
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
        sys_fentries: Some(4usize),
    },
    Peripheral {
        name: "UART4",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "CTS",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CTS",
                pf: Some(9u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CTS",
                pf: Some(10u8),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "RTS",
                pf: Some(6u8),
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
            PeripheralPin {
                pin: "PB22",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "RX",
                pf: Some(8u8),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "TX",
                pf: Some(8u8),
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
        name: "LCD",
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
    Pin {
        pin: "PA6",
        pincm: 11u8,
    },
    Pin {
        pin: "PB0",
        pincm: 12u8,
    },
    Pin {
        pin: "PB1",
        pincm: 13u8,
    },
    Pin {
        pin: "PA7",
        pincm: 14u8,
    },
    Pin {
        pin: "PB2",
        pincm: 15u8,
    },
    Pin {
        pin: "PB3",
        pincm: 16u8,
    },
    Pin {
        pin: "PA1",
        pincm: 2u8,
    },
    Pin {
        pin: "PB4",
        pincm: 17u8,
    },
    Pin {
        pin: "PB5",
        pincm: 18u8,
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
        pin: "PB28",
        pincm: 21u8,
    },
    Pin {
        pin: "PB29",
        pincm: 22u8,
    },
    Pin {
        pin: "PB30",
        pincm: 23u8,
    },
    Pin {
        pin: "PB31",
        pincm: 24u8,
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
        pin: "PA28",
        pincm: 3u8,
    },
    Pin {
        pin: "PB6",
        pincm: 27u8,
    },
    Pin {
        pin: "PB7",
        pincm: 28u8,
    },
    Pin {
        pin: "PB8",
        pincm: 29u8,
    },
    Pin {
        pin: "PB9",
        pincm: 30u8,
    },
    Pin {
        pin: "PB10",
        pincm: 31u8,
    },
    Pin {
        pin: "PB11",
        pincm: 32u8,
    },
    Pin {
        pin: "PB12",
        pincm: 33u8,
    },
    Pin {
        pin: "PB13",
        pincm: 34u8,
    },
    Pin {
        pin: "PB14",
        pincm: 35u8,
    },
    Pin {
        pin: "PB15",
        pincm: 36u8,
    },
    Pin {
        pin: "PA29",
        pincm: 4u8,
    },
    Pin {
        pin: "PB16",
        pincm: 37u8,
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
        pin: "PA16",
        pincm: 42u8,
    },
    Pin {
        pin: "PC0",
        pincm: 43u8,
    },
    Pin {
        pin: "PC1",
        pincm: 44u8,
    },
    Pin {
        pin: "PA30",
        pincm: 5u8,
    },
    Pin {
        pin: "PC2",
        pincm: 45u8,
    },
    Pin {
        pin: "PC3",
        pincm: 46u8,
    },
    Pin {
        pin: "PC4",
        pincm: 47u8,
    },
    Pin {
        pin: "PC5",
        pincm: 48u8,
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
        pin: "PB17",
        pincm: 53u8,
    },
    Pin {
        pin: "PB18",
        pincm: 54u8,
    },
    Pin {
        pin: "PB19",
        pincm: 55u8,
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
        pin: "PC6",
        pincm: 58u8,
    },
    Pin {
        pin: "PC7",
        pincm: 59u8,
    },
    Pin {
        pin: "PC8",
        pincm: 60u8,
    },
    Pin {
        pin: "PC9",
        pincm: 61u8,
    },
    Pin {
        pin: "PB20",
        pincm: 62u8,
    },
    Pin {
        pin: "PB21",
        pincm: 63u8,
    },
    Pin {
        pin: "PB22",
        pincm: 64u8,
    },
    Pin {
        pin: "PB23",
        pincm: 65u8,
    },
    Pin {
        pin: "PB24",
        pincm: 66u8,
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
        pin: "PB25",
        pincm: 70u8,
    },
    Pin {
        pin: "PB26",
        pincm: 71u8,
    },
    Pin {
        pin: "PB27",
        pincm: 72u8,
    },
    Pin {
        pin: "PA26",
        pincm: 73u8,
    },
    Pin {
        pin: "PA27",
        pincm: 74u8,
    },
];
