//! Contains an enum for the POS tags used by Senna

use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum POS {
    CC,          // coordinating conjunction (and, or)
    CD,          // cardinal numeral
    COL,         // :
    COM,         // ,
    DOL,         // $
    DT,          // singular determiner (this, that)
    EX,          // existential there
    FW,          // foregin word (hyphenated before regular tag)
    IN,          // preposition
    JJ,          // adjective
    JJR,         // comparative adjective
    JJS,         // semantically superlative adjective (chief, top)
    LRB,         // open parenthesis
    LS,          // list item marker
    MD,          // modal auxiliary (can, should, will)
    NN,          // singular or mass noun
    NNP,         // proper noun, singular
    NNPS,        // proper noun, plural
    NNS,         // plural noun
    NOT_SET,     // not set
    PADDING,     // padding
    PDT,         // predeterminer
    POS,         // possessive ending
    POUND,       // #
    PRP,         // personal pronoun
    PRP_POSS,    // possessive pronoun
    PUNCT,       // .
    QUOT_B,      // ''
    QUOT_S,      // ``
    RB,          // adverb
    RBR,         // comparative adverb
    RBS,         // adverb, superlative
    RP,          // adverb/particle (about, off, up)
    RRB,         // close parenthesis
    SYM,         // symbol
    TO,          // to
    UH,          // interjection, exclamation
    UNAVAILABLE, // tag unavailable
    VB,          // verb, base form
    VBD,         // verb, past tense
    VBG,         // verb, present particle/gerund
    VBN,         // verb, past participle
    VBP,         // verb, non 3rd person, singular, present
    VBZ,         // verb, 3rd singular present
    WDT,         // wh- determiner (what, which)
    WP,          // possessive wh- pronoun (whose)
    WP_POSS,     // nominative wh- pronoun (who, which, that)
    WRB,         // wh- adverb (how, where, when)
}

impl POS {
    pub fn to_str(&self) -> &'static str {
        match self {
            &POS::NNP => "NNP",
            &POS::COM => ",",
            &POS::CD => "CD",
            &POS::NNS => "NNS",
            &POS::JJ => "JJ",
            &POS::MD => "MD",
            &POS::VB => "VB",
            &POS::DT => "DT",
            &POS::NN => "NN",
            &POS::IN => "IN",
            &POS::PUNCT => ".",
            &POS::VBZ => "VBZ",
            &POS::VBG => "VBG",
            &POS::CC => "CC",
            &POS::VBD => "VBD",
            &POS::VBN => "VBN",
            &POS::RB => "RB",
            &POS::TO => "TO",
            &POS::PRP => "PRP",
            &POS::RBR => "RBR",
            &POS::WDT => "WDT",
            &POS::VBP => "VBP",
            &POS::RP => "RP",
            &POS::PRP_POSS => "PRP$",
            &POS::JJS => "JJS",
            &POS::POS => "POS",
            &POS::QUOT_S => "``",
            &POS::WP => "WP",
            &POS::QUOT_B => "''",
            &POS::COL => ":",
            &POS::JJR => "JJR",
            &POS::WRB => "WRB",
            &POS::EX => "EX",
            &POS::DOL => "$",
            &POS::NNPS => "NNPS",
            &POS::WP_POSS => "WP$",
            &POS::LRB => "-LRB-",
            &POS::RRB => "-RRB-",
            &POS::PDT => "PDT",
            &POS::RBS => "RBS",
            &POS::FW => "FW",
            &POS::UH => "UH",
            &POS::SYM => "SYM",
            &POS::LS => "LS",
            &POS::POUND => "#",
            &POS::PADDING => "PADDING",
            &POS::UNAVAILABLE => "UNAVAILABLE",
            &POS::NOT_SET => "POS IS NOT SET",
        }
    }

    pub fn generate_str_to_pos_map<'a>() -> HashMap<&'a str, POS> {
        let mut map = HashMap::new();
        map.insert("NNP", POS::NNP);
        map.insert(",", POS::COM);
        map.insert("CD", POS::CD);
        map.insert("NNS", POS::NNS);
        map.insert("JJ", POS::JJ);
        map.insert("MD", POS::MD);
        map.insert("VB", POS::VB);
        map.insert("DT", POS::DT);
        map.insert("NN", POS::NN);
        map.insert("IN", POS::IN);
        map.insert(".", POS::PUNCT);
        map.insert("VBZ", POS::VBZ);
        map.insert("VBG", POS::VBG);
        map.insert("CC", POS::CC);
        map.insert("VBD", POS::VBD);
        map.insert("VBN", POS::VBN);
        map.insert("RB", POS::RB);
        map.insert("TO", POS::TO);
        map.insert("PRP", POS::PRP);
        map.insert("RBR", POS::RBR);
        map.insert("WDT", POS::WDT);
        map.insert("VBP", POS::VBP);
        map.insert("RP", POS::RP);
        map.insert("PRP$", POS::PRP_POSS);
        map.insert("JJS", POS::JJS);
        map.insert("POS", POS::POS);
        map.insert("``", POS::QUOT_S);
        map.insert("WP", POS::WP);
        map.insert("''", POS::QUOT_B);
        map.insert(":", POS::COL);
        map.insert("JJR", POS::JJR);
        map.insert("WRB", POS::WRB);
        map.insert("EX", POS::EX);
        map.insert("$", POS::DOL);
        map.insert("NNPS", POS::NNPS);
        map.insert("WP$", POS::WP_POSS);
        map.insert("-LRB-", POS::LRB);
        map.insert("-RRB-", POS::RRB);
        map.insert("PDT", POS::PDT);
        map.insert("RBS", POS::RBS);
        map.insert("FW", POS::FW);
        map.insert("UH", POS::UH);
        map.insert("SYM", POS::SYM);
        map.insert("LS", POS::LS);
        map.insert("#", POS::POUND);
        map.insert("PADDING", POS::PADDING);
        map.insert("UNAVAILABLE", POS::UNAVAILABLE);
        map
    }
}
