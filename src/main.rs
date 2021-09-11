#[allow(unused_imports)]
use std::env;
use std::fs;
use std::process;

mod toml_parser;

#[allow(dead_code)]
pub struct Configs {
    h1_class: &'static str,
    h2_class: &'static str,
    h3_class: &'static str,
    paragraph_class: &'static str,
    blockquote_class: &'static str,
}

impl Configs {
    #[allow(dead_code)]
    fn make_config(&self){
        //todo
    }
}

fn debug_toml_parser (){
    toml_parser::parse();
    std::process::exit(1);
}

fn get_filename(args: Vec<String>) -> String {

    if args.len() < 3 {
        println!("Expected file name");
        process::exit(1);
    }
    fs::read_to_string(&args[2]).expect("Expected file name")
}

fn main() {
    // debug_toml_parser();
    let file_contents = get_filename(env::args().collect());
    // println!("{}", file_contents);

    let one_liner_chunks_vec = file_contents.lines().collect::<Vec<&str>>();
    // println!("{:?}", one_liner_chunks_vec);

    for (i, e) in one_liner_chunks_vec.iter().enumerate() {
        // let plusone = one_liner_chunks_vec[i];
        
        let word_chunks: Vec<&str> = e.split_whitespace().collect();
        let characterized_chunks: Vec<char> = e.trim().chars().collect();

        if characterized_chunks.len() == 0 {
            continue;
        }
        let first_word = word_chunks[0];
        let _top_level_parse = match first_word {
            "#" => println!("<h1>{}</h1>", &e[first_word.len()..].trim()),
            "##" => println!("<h2>{}</h2>", &e[first_word.len()..].trim()),
            "###" => println!("<h3>{}</h3>", &e[first_word.len()..].trim()),
            ">" => {
                //check if one space is present after '>'
                if characterized_chunks[1] == ' ' {
                    println!(
                        "<blockquote>{}</blockquote>",
                        &e[first_word.len()..].trim()
                    )
                } else {
                    println!("{}", &e[first_word.len()..].trim())
                }
            }
            _ => {
                let formatting = if one_liner_chunks_vec[i] == "" { 
                    println!("")
                }
                else if i == 0 && one_liner_chunks_vec[i + 1] != "" {
                    println!("<p>{}", e)
                } else if i == 0 && one_liner_chunks_vec[i + 1] == "" {
                    println!("<p>{}</p>", e)
                } else if i == one_liner_chunks_vec.len() - 1 && one_liner_chunks_vec[i - 1] != "" {
                    println!("{}</p>", e)
                } else if i == one_liner_chunks_vec.len() - 1 && one_liner_chunks_vec[i - 1] == "" {
                    println!("<p>{}</p>", e)
                } else if one_liner_chunks_vec[i - 1] == "" && one_liner_chunks_vec[i + 1] == "" {
                    println!("<p>{}<p>", e)
                } else if one_liner_chunks_vec[i - 1] == "" && one_liner_chunks_vec[i + 1] != "" {
                    println!("<p>{}", e)
                } else if one_liner_chunks_vec[i - 1] != "" && one_liner_chunks_vec[i + 1] != "" {
                    println!("{}", e)
                } else {
                    println!("{}</p>", e)
                };
                formatting
            }
        };
        // println!("{}", top_level_parse);

    }
    // println!("{} sec {} ms", now.elapsed().as_secs(), now.elapsed().subsec_nanos() as u64 / 1_000_000);
}

// for testing
// fn main() {
//     let mut i = 0;
//     loop {
//         println!("# foo");
//         if i == 1000000 {
//             break;
//         }
//         i+=1;
//     }
// }
