use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Currency {
    AUD,
    CAD,
    CHF,
    CNH,
    CZK,
    DKK,
    EUR,
    HKD,
    HUF,
    JPY,
    MXN,
    NOK,
    NZD,
    PLN,
    SEK,
    SGD,
    USD,
    ZAR,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
