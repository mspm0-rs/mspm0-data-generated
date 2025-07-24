pub static ALL_PERIPHERAL_VERSIONS: &[(&str, &[&str])] = &[
    ("adc", &["v1"]),
    ("cpuss", &["v1"]),
    ("dma", &["v1"]),
    ("gpio", &["v1"]),
    ("i2c", &["v1"]),
    ("iomux", &["v1"]),
    (
        "sysctl",
        &[
            "c110x",
            "g350x_g310x_g150x_g110x",
            "g351x_g151x",
            "l110x_l130x_l134x",
            "l122x_l222x",
        ],
    ),
    ("tim", &["v1"]),
    ("uart", &["v1"]),
];
