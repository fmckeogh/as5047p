pub enum VolatileRegister {
    Nop,
    Errfl,
    Prog,
    Diaagc,
    Mag,
    Angleunc,
    Anglecom,
}

impl VolatileRegister {
    pub fn address(&self) -> u16 {
        match *self {
            VolatileRegister::Nop => 0x0000,
            VolatileRegister::Errfl => 0x0001,
            VolatileRegister::Prog => 0x0003,
            VolatileRegister::Diaagc => 0x3FFC,
            VolatileRegister::Mag => 0x3FFD,
            VolatileRegister::Angleunc => 0x3FFE,
            VolatileRegister::Anglecom => 0x3FFF,
        }
    }
}
