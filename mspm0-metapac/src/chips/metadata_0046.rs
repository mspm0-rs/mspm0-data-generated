
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
                signal: "4",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "6",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "7",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
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
                pin: "PA20",
                signal: "IN1+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "IN1-",
                pf: Some(0u8),
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
        name: "CRC",
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
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "IN+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "IN-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "OUT",
                pf: Some(0u8),
            },
        ],
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
                pin: "PA1",
                signal: "PA1",
                pf: Some(1u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "PA17",
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
                pin: "PA6",
                signal: "PA6",
                pf: Some(1u8),
            },
        ],
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
                pin: "PA0",
                signal: "SDA",
                pf: Some(3u8),
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
                pin: "PA17",
                signal: "SCL",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "SCL",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "SDA",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "SDA",
                pf: Some(4u8),
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
        name: "OPA0",
        kind: "opa",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA25",
                signal: "IN0+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "IN1-",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "OUT",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "OPA1",
        kind: "opa",
        version: None,
        pins: &[
            PeripheralPin {
                pin: "PA18",
                signal: "IN0+",
                pf: Some(0u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "IN1-",
                pf: Some(0u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
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
                pin: "PA0",
                signal: "CS1_POCI1",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CS1_POCI1",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CS2_POCI2",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CS3_CD_POCI3",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "PICO",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "PICO",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "POCI",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "POCI",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SCLK",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "SCLK",
                pf: Some(4u8),
            },
        ],
        power_domain: PowerDomain::Pd1,
    },
    Peripheral {
        name: "TIMG0",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA23",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CCP1",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CCP1",
                pf: Some(3u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "TIMG1",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "CCP0",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CCP1",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CCP1",
                pf: Some(4u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "TIMG2",
        kind: "tim",
        version: Some("v1"),
        pins: &[PeripheralPin {
            pin: "PA22",
            signal: "CCP1",
            pf: Some(3u8),
        }],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "TIMG4",
        kind: "tim",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "CCP0",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CCP0",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CCP1",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "CCP1",
                pf: Some(5u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "UART0",
        kind: "uart",
        version: Some("v1"),
        pins: &[
            PeripheralPin {
                pin: "PA23",
                signal: "CTS",
                pf: Some(5u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RTS",
                pf: Some(4u8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RX",
                pf: Some(3u8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "TX",
                pf: Some(3u8),
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
                pin: "PA1",
                signal: "RX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RX",
                pf: Some(6u8),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                pf: Some(2u8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "TX",
                pf: Some(6u8),
            },
        ],
        power_domain: PowerDomain::Pd0,
    },
    Peripheral {
        name: "VREF",
        kind: "vref",
        version: None,
        pins: &[PeripheralPin {
            pin: "PA23",
            signal: "VREF+",
            pf: Some(0u8),
        }],
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
        name: "TIMG1",
        number: 2,
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
        name: "UART1",
        number: 13,
    },
    Interrupt {
        name: "UART0",
        number: 15,
    },
    Interrupt {
        name: "TIMG0",
        number: 16,
    },
    Interrupt {
        name: "TIMG2",
        number: 18,
    },
    Interrupt {
        name: "TIMG4",
        number: 20,
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
                name: "COMP0",
                number: 2,
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
        full: false,
    },
    DmaChannel {
        number: 2,
        full: false,
    },
];
pub(crate) static PINS: &[Pin] = &[
    Pin {
        pin: "PA26",
        pincm: 27u8,
    },
    Pin {
        pin: "PA18",
        pincm: 19u8,
    },
    Pin {
        pin: "PA19",
        pincm: 20u8,
    },
    Pin {
        pin: "PA20",
        pincm: 21u8,
    },
    Pin {
        pin: "PA22",
        pincm: 23u8,
    },
    Pin {
        pin: "PA23",
        pincm: 24u8,
    },
    Pin {
        pin: "PA24",
        pincm: 25u8,
    },
    Pin {
        pin: "PA25",
        pincm: 26u8,
    },
    Pin {
        pin: "PA0",
        pincm: 1u8,
    },
    Pin {
        pin: "PA1",
        pincm: 2u8,
    },
    Pin {
        pin: "PA2",
        pincm: 3u8,
    },
    Pin {
        pin: "PA6",
        pincm: 7u8,
    },
    Pin {
        pin: "PA17",
        pincm: 18u8,
    },
];
