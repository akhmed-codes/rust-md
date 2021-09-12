use std::fs;
use serde::*;
use toml;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Configs {
    pub h1_class: String,
    pub h2_class: String,
    pub h3_class: String,
    pub paragraph_class: String,
    pub blockquote_class: String,
}

// impl Configs {
//     fn make_config(&self, config: Configs) -> Configs {
//         Configs {
//             h1_class: "default".to_owned(),
//             h2_class: "default".to_owned(),
//             h3_class: "default".to_owned(),
//             paragraph_class: "default".to_owned(),
//             blockquote_class: "default".to_owned(),
//         }      
//     }
// }


pub fn parse() -> Configs {

    let content = fs::read_to_string("config.toml").expect("Expected file name");

    let parsed: Configs = toml::from_str(content.as_str()).unwrap();

    dbg!(&parsed.h1_class);
    dbg!(&parsed.h2_class);
    dbg!(&parsed.h3_class);
    dbg!(&parsed.paragraph_class);
    dbg!(&parsed.blockquote_class);

    let new_config = Configs {
        h1_class: parsed.h1_class,
        h2_class: parsed.h2_class,
        h3_class: parsed.h3_class,
        paragraph_class: parsed.paragraph_class,
        blockquote_class: parsed.blockquote_class,
    };
    new_config
}