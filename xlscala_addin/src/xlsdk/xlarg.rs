use crate::semantics::Str;

pub enum Xlarg {
    CSTRING, PSTRING,
    CSTRING_REF, PSTRING_REF,

    UNICODE_CSTRING, UNICODE_PSTRING,
    UNICODE_CSTRING_REF, UNICODE_PSTRING_REF,

    BOOL, USHORT, SHORT, INT, DOUBLE,
    LP_BOOL, LP_SHORT, LP_INT, LP_DOUBLE,

    LP_OPER, LP_REF,
    LP_OPER12, LP_REF12,

    ARRAY, ARRAY12,

    FP, FP12,
}

impl Xlarg {

    #[inline(always)]
    pub(crate) fn to_code(&self) -> Str {
        match self {
            Xlarg::CSTRING => "C",
            Xlarg::PSTRING => "D",
            Xlarg::CSTRING_REF => "F",
            Xlarg::PSTRING_REF => "G",

            Xlarg::UNICODE_CSTRING => "C%",
            Xlarg::UNICODE_PSTRING => "D%",
            Xlarg::UNICODE_CSTRING_REF => "F%",
            Xlarg::UNICODE_PSTRING_REF => "G%",

            Xlarg::BOOL => "A",
            Xlarg::USHORT => "H",
            Xlarg::SHORT => "I",
            Xlarg::INT => "J",
            Xlarg::DOUBLE => "B",

            Xlarg::LP_BOOL => "L",
            Xlarg::LP_SHORT => "M",
            Xlarg::LP_INT => "N",
            Xlarg::LP_DOUBLE => "E",

            Xlarg::LP_OPER => "P",
            Xlarg::LP_REF => "R",

            Xlarg::LP_OPER12 => "Q",
            Xlarg::LP_REF12 => "U",

            Xlarg::FP => "K",
            Xlarg::FP12 => "K%",

            Xlarg::ARRAY => "O",
            Xlarg::ARRAY12 => "O%",
        }
    }
}

impl From<Xlarg> for Str {
    fn from(value: Xlarg) -> Self {
        value.to_code()
    }
}