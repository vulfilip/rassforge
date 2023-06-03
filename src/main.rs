#![allow(unused)]

use core::panic;
use std::mem::replace;
use std::time::Instant;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use rand::Rng;
use md5::Md5;
use md5::Digest as Md5Digest;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use base64::{Engine as _, engine::general_purpose};
use base32::Alphabet;
use base32::encode as base32Encode;
use clap::{ArgAction, Command, Arg};


fn main() {
    //ARGUMENT PARSING
    let matches = Command::new("Rassforge")
        .version("1.0")
        .author("Vulfilip")
        .about("Rassforge - Rust Password Forge")
        .arg_required_else_help(true)
        .subcommand(Command::new("standard")
            .about("Rassforge standard mode wordlist generation")
            .arg_required_else_help(true)
            .arg(Arg::new("wordlist").short('w').long("wordlist").value_name("FILE")
                .help("Specify a file containing keywords to be used in the generation of passwords")
                .long_help("File containing keywords to be used in the generation of passwords. \n
                Best practice is putting only the important, target-related keywords in there \n
                (CompanyName,FirstName,LastName,PetName,ChildName,FavouriteSportsClub etc.). \n
                One line - one keyword.")
            .required(true)
            )
            .arg(Arg::new("years").short('y').long("years")
                    .help("Specify years, or range of years (xxxx-xxxx)")
                    .long_help("Year range to be used in generation of password list. \n
                    Best practice is putting current year, and few previous years, or any important years in \n
                    targets life. For example, 2020-2023,1984 \n
                    Syntax: YEAR-YEAR | YEAR-YEAR,YEAR | YEAR,YEAR,YEAR")
                    .required(true)
                )
            .arg(Arg::new("symbols").short('s').long("symbols")
                .help("Set the symbols which should be used.")
                .long_help("Specifies which special characters will be used while generating a bruteforce password list.").required(false)
                .default_value("None")
            )
            .arg(Arg::new("leet").long("leet").help("Enables leet speak. Replaces characters with their corresponding 1337 num63r5. Extends usually generated wordlist with leet speak results also. Doesn't convert seasons and months into leet speak.")
                .required(false).action(ArgAction::SetTrue))
            .arg(Arg::new("reverse").short('r').long("reverse").action(ArgAction::SetTrue)
                .help("Enables reverse mixing of keywords and years: YearKeywordSymbol and SymbolKeywordYear. Put symbols in between quotation marks: \"!@#\"").required(false))
            )
        .arg(Arg::new("output").short('o').long("output").help("Specify a  file to write results in.").default_value("rassforge_passlist.txt").global(true))
        .arg(Arg::new("head").long("head").help("Specify a value which will ALWAYS be at the beggining of generated words").global(true).required(false))
        .arg(Arg::new("tail").long("tail").help("Specify a value which will ALWAYS be at the end of generated words").global(true).required(false))
        .subcommand(Command::new("crunch")
            .about("Rassforge crunch mode wordlist generation")
            .arg_required_else_help(true)
            .arg(Arg::new("minsize").long("min")
                .help("Defines a combination string minimum size for crunch mode."))
            .arg(Arg::new("maxsize").long("max")
                .help("Defines a combination string maximum size for crunch mode."))
            .arg(Arg::new("characterset").short('c').long("characters")
                .help("Defines a combination string character set for crunch mode."))
        )   
        .subcommand(Command::new("encode").about("Rassforge encoding command.").arg_required_else_help(true)
        .arg(Arg::new("encoding").long("encoding").short('e').help("Encodes the generated words with desired encoding.")
            .long_help("Reads a file and encodes each line to desired hash algorithm. Good if you need a wordlist of encoded passwords. Possible algorithms: md5, sha1, sha256, sha512, base32, base64, rot13")
            .value_parser(["md5", "sha256", "base64", "base32", "sha512", "sha1", "rot13"]))
        .arg(Arg::new("file").long("file").short('f')
            .help("Specifies a file which contains words which need to be encoded.")))
        .get_matches();
    
    let path_write = PathBuf::from_str(matches.get_one::<String>("output").expect("No valid wordlist specified.")).expect("Failure during path conversion");
    let is_tail_specified: bool;
    let is_head_specified: bool;
    // assigns head and tail value if specified
    let head_value = match matches.get_one::<String>("head"){
        Some(value) => {is_head_specified = true; value},
        None => {is_head_specified = false; "None"},
    };
    let tail_value = match matches.get_one::<String>("tail"){
        Some(value) => {is_tail_specified = true; value},
        None => {is_tail_specified = false; "None"},
    };

    let start_time = Instant::now();
    //Does things based on the subcommand specified
    match matches.subcommand_name(){
        Some("standard") => {
            banner();
            let subcommand_matches_standard = matches.subcommand_matches("standard").expect("Failure during subcommand matching.");
            //Testing if symbols are specified and if they have an escape character.
            let symbols = subcommand_matches_standard.get_one::<String>("symbols").expect("Symbol loading failed. Check the syntax");
            let mut symbols_final = String::new();
            if symbols != "None" {
                if symbols.matches('\\').count() > 1 {
                    symbols_final = symbols.replace("\\", "");
                    symbols_final.push('\\');
                }else{
                    symbols_final = symbols.replace("\\", "");
                }
            }
            let path = PathBuf::from_str(subcommand_matches_standard.get_one::<String>("wordlist").expect("No valid wordlist specified.")).expect("Failure during path conversion");
            let years_range = subcommand_matches_standard.get_one::<String>("years").expect("Year range loading failed. Check the syntax");
            let mut years_list = Vec::new();
            years_parser(&years_range, &mut years_list);
            let mut keywords = read_file(&path);
            let is_leet_enabled = subcommand_matches_standard.get_flag("leet");
            if is_leet_enabled{
                for word in &keywords.clone() {
                    keywords.push(leet(word));
                }
            }
            let is_reverse_enabled = subcommand_matches_standard.get_flag("reverse");

            println!("Standard mode selected");
            println!("File containing keywords: {}. Years: {}. Symbols: {}", &path.display(), &years_range, &symbols_final);
            println!("Forging started...");
            //Combining stuff...
            let standard_combine_input_output = standard_combine_input(&keywords, &years_list, &symbols_final);
            let standard_season_combine_output = standard_season_combine(&years_list, &symbols_final);
            let standard_combine_input_reverse_output = standard_combine_input_reverse(&keywords, &years_list, &symbols_final);
            //Writing stuff to the file
            if is_reverse_enabled{
                if is_head_specified{
                    if is_tail_specified == false{
                        write_to_file(head_add(head_value, standard_combine_input_output), &path_write);
                        write_to_file(head_add(head_value, standard_season_combine_output), &path_write);
                        write_to_file(head_add(head_value,standard_combine_input_reverse_output), &path_write);
                    }else{
                        write_to_file(tail_add(tail_value, head_add(head_value, standard_combine_input_output)), &path_write);
                        write_to_file(tail_add(tail_value, head_add(head_value, standard_season_combine_output)), &path_write);
                        write_to_file(tail_add(tail_value, head_add(head_value,standard_combine_input_reverse_output)), &path_write);
                    }
                }else{
                    if is_tail_specified == false{
                        write_to_file(standard_combine_input_output, &path_write);
                        write_to_file(standard_season_combine_output, &path_write);
                        write_to_file(standard_combine_input_reverse_output, &path_write);
                    }else{
                        write_to_file(tail_add(tail_value,standard_combine_input_output), &path_write);
                        write_to_file(tail_add(tail_value, standard_season_combine_output), &path_write);
                        write_to_file(tail_add(tail_value, standard_combine_input_reverse_output), &path_write);
                    }
                }
            }else{
                if is_head_specified{
                    if is_tail_specified == false{
                        write_to_file(head_add(head_value, standard_combine_input_output), &path_write);
                        write_to_file(head_add(head_value, standard_season_combine_output), &path_write);
                    }else{
                        write_to_file(tail_add(tail_value, head_add(head_value, standard_combine_input_output)), &path_write);
                        write_to_file(tail_add(tail_value, head_add(head_value, standard_season_combine_output)), &path_write);
                    }
                }else{
                    if is_tail_specified == false{
                        write_to_file(standard_combine_input_output, &path_write);
                        write_to_file(standard_season_combine_output, &path_write);
                    }else{
                        write_to_file(tail_add(tail_value,standard_combine_input_output), &path_write);
                        write_to_file(tail_add(tail_value, standard_season_combine_output), &path_write);
                    }
                }
            }
            let file_size: u64 = fs::metadata(&path_write).unwrap().len();
            println!("Forging finished! Wordlist generated.");
            file_size_calculator(file_size);
            },
        Some("crunch") => {
            banner();
            let subcommand_matches_crunch = matches.subcommand_matches("crunch").expect("Failure during subcommand matching.");
            let minimum_crunch_size = subcommand_matches_crunch.get_one::<String>("minsize").expect("Invalid minimum size entered.").trim().parse().expect("Failure during conversion of minimum size, check the input.");
            let maximum_crunch_size = subcommand_matches_crunch.get_one::<String>("maxsize").expect("Invalid maximum size entered.").trim().parse().expect("Failure during conversion of minimum size, check the input.");
            let character_set_crunch: Vec<char> = subcommand_matches_crunch.get_one::<String>("characterset").expect("Invalid character set specified").chars().collect();

            println!("Crunch mode selected");
            println!("Character set: {}. Minimum size: {}. Maximum size: {}", subcommand_matches_crunch.get_one::<String>("characterset").expect("Invalid character set specified"), minimum_crunch_size, maximum_crunch_size);
            println!("Forging started...");
            //Writing stuff to the file
            if is_head_specified{
                if is_tail_specified == false{
                    write_to_file(head_add(head_value, crunch(&character_set_crunch, minimum_crunch_size, maximum_crunch_size)), &path_write);
                }else{
                    write_to_file(tail_add(tail_value, head_add(head_value, crunch(&character_set_crunch, minimum_crunch_size, maximum_crunch_size))), &path_write)
                }
            }else{
                if is_tail_specified == false{
                    write_to_file(crunch(&character_set_crunch, minimum_crunch_size, maximum_crunch_size), &path_write);
                }else{
                    write_to_file(tail_add(tail_value, crunch(&character_set_crunch, minimum_crunch_size, maximum_crunch_size)), &path_write);
                }
            }
            println!("Forging finished! Wordlist generated.");
            let file_size = fs::metadata(&path_write).unwrap().len();
            file_size_calculator(file_size);
        },
        Some("encode") => {
            banner();
            let encoding_type = matches.subcommand_matches("encode").expect("Failure during subcommand matching.")
                                .get_one::<String>("encoding").expect("Invalid encoding algorithm specified.");
            let file_input = PathBuf::from_str(matches.subcommand_matches("encode").expect("Failure during subcommand matching.")
                            .get_one::<String>("file").expect("Invalid input file specified.")).expect("Invalid path specified.");
            let file_output = PathBuf::from_str(matches.subcommand_matches("encode").expect("Failure during subcommand matching.")
                                            .get_one::<String>("output").expect("Invalid output file specified.")).expect("Invalid path specified.");
            
            println!("Encode mode selected");
            println!("Encoding type: {}, File input: {}, File output: {}", encoding_type, file_input.display(), file_output.display());
            println!("Forging started...");
            encode(&file_input, &encoding_type, &file_output);
            println!("Forging finished! Wordlist generated.");
            let file_size = fs::metadata(&file_output).unwrap().len();
            file_size_calculator(file_size);
        },
        
        _ => panic!("Invalid mode entered. Check the syntax and available modes"),
    }
    let elapsed = start_time.elapsed();
    println!("Forged in: {} minutes, {} seconds.", elapsed.as_secs()/60, elapsed.as_secs()%60);
}


//STANDARD MODE
fn standard_combine_input(list: &Vec<String>, years_list: &Vec<String>, symbols_list: &str) -> Vec<String>{
    let mut output_list = Vec::new();
    for item in list.iter(){
        for year in years_list.iter(){
            for symbol in symbols_list.chars(){
                output_list.push(standard_combine_material(item, year, symbol));
            }
        }
    }
    for item in list.iter(){
        for year in years_list.iter(){
            output_list.push(standard_combine_material(item, year, 'a'));
        }
    }
    for year in years_list.iter(){
        for item in list.iter(){
            for symbol in symbols_list.chars(){
                output_list.push(standard_combine_material_bac(item, year, symbol));
            }
        }
    }
    for year in years_list.iter(){
        for item in list.iter(){
            output_list.push(standard_combine_material_bac(item, year, 'a'));
        }
    }
    for symbol in symbols_list.chars(){
        for item in list.iter(){
            for year in years_list.iter(){
                output_list.push(standard_combine_material_cab(item, year, symbol));
            }
        }
    }
    return output_list;
}
fn standard_combine_input_reverse(list: &Vec<String>, years_list: &Vec<String>, symbols_list: &str) -> Vec<String>{
    //Combining stuff just in reverse order
    let mut output_list = Vec::new();
    for year in years_list.iter(){
        for item in list.iter(){
            for symbol in symbols_list.chars(){
                output_list.push(standard_combine_material_bac(item, year, symbol));
            }
        }
    }
    for year in years_list.iter(){
        for item in list.iter(){
            output_list.push(standard_combine_material_bac(item, year, 'a'));
        }
    }
    for symbol in symbols_list.chars(){
        for item in list.iter(){
            for year in years_list.iter(){
                output_list.push(standard_combine_material_cab(item, year, symbol));
            }
        }
    }
    return output_list;
}

fn standard_combine_material(word: &str, year_input: &str, symbol_input: char) -> String{
    if symbol_input == 'a'{return format!("{}{}", word, year_input);}
    else{return format!("{}{}{}", word, year_input, symbol_input);}
}
fn standard_combine_material_bac(word: &str, year_input: &str, symbol_input: char) -> String{
    if symbol_input == 'a'{return format!("{}{}", year_input, word);}
    else{return format!("{}{}{}", year_input, word, symbol_input);}
}
fn standard_combine_material_cab(word: &str, year_input: &str, symbol_input: char) -> String{
    if symbol_input == 'a'{return format!("{}{}", word, symbol_input);}
    else{return format!("{}{}{}", symbol_input, word, year_input);}
}
fn standard_season_combine(year_list: &Vec<String>, symbols_list: &str) -> Vec<String>{
    let mut output_list = Vec::new();
    let seasons = ["Summer", "Autumn", "Winter", "Fall", "Spring"];
    let months = ["January", "February", "March", "April", "June", "July", "August", "September", "October", "November", "December"];
    for year in year_list.iter(){
        for season in seasons.iter(){
            output_list.push(format!("{}{}", season, year));
            output_list.push(format!("{}{}", year, season));
            for symbol in symbols_list.chars(){
                output_list.push(format!("{}{}{}", season, year, symbol));
                output_list.push(format!("{}{}{}", year, season, symbol));
            }
        }
        for month in months.iter(){
            output_list.push(format!("{}{}", month, year));
            output_list.push(format!("{}{}", year, month));
            for symbol in symbols_list.chars(){
                output_list.push(format!("{}{}{}", month, year, symbol));
                output_list.push(format!("{}{}{}", year, month, symbol));
            }
        }
    }
    return output_list;
}
fn years_parser(year_input: &str, years: &mut Vec<String>) {
    //Parses the year input, so it can include both single years and year ranges [for example: 2012-2018,2023]
    let year_ranges: Vec<&str> = year_input.split(',').collect();

    for year_range in year_ranges {
        let years_or_range: Vec<&str> = year_range.split('-').collect();

        if years_or_range.len() == 1 {
            // Single year
            years.push(years_or_range[0].trim().to_owned());
        } else if years_or_range.len() == 2 {
            // Year range
            let start_year = years_or_range[0].trim().parse::<i32>().unwrap();
            let end_year = years_or_range[1].trim().parse::<i32>().unwrap();
            for year in start_year..=end_year {
                years.push(year.to_string());
            }
        } else {
            panic!("Invalid year or range: {}", year_range);
        }
    }
    for year in years.clone(){
        years.push(year[year.len()-2..].to_string());
    }
}

//END STANDARD MODE

//CRUNCH MODE
fn crunch(characters: &[char], min_size: usize, max_size: usize) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut current_combination: Vec<char> = Vec::new();
    
    fn combine(characters: &[char], min_size: usize, max_size: usize, result: &mut Vec<String>, current_combination: &mut Vec<char>) {
        if current_combination.len() >= min_size && current_combination.len() <= max_size {
            result.push(current_combination.iter().collect());
        }
        
        if current_combination.len() == max_size {
            return;
        }
        
        for i in 0..characters.len() {
            current_combination.push(characters[i]);
            combine(characters, min_size, max_size, result, current_combination);
            current_combination.pop();
        }
    }
    
    combine(characters, min_size, max_size, &mut output, &mut current_combination);
    output
}
//END CRUNCH MODE

//ENCODE MODE
fn encode(input: &PathBuf, encodetype: &str, output: &PathBuf){
    let file = File::open(input).expect("Invalid file specified.");
    let reader = BufReader::new(file);
    let file_encoded = OpenOptions::new().create(true).write(true).append(true).open(output).unwrap();
    let mut writer = BufWriter::new(file_encoded);
    match encodetype {
        "md5" => {
            for lines in reader.lines(){
                let hash = Md5::digest(lines.unwrap().as_bytes());
                let human_readable_hash = format!("{:x}\n", hash);
                writer.write_all(human_readable_hash.as_bytes()).unwrap();
                
            }
            writer.flush();
        },
        "sha1" => {
            for lines in reader.lines(){
                let hash = Sha1::digest(lines.unwrap().as_bytes());
                let human_readable_hash = format!("{:x}\n", hash);
                writer.write_all(human_readable_hash.as_bytes()).unwrap();
                
            }
            writer.flush();
        },
        "sha256" => {
            for lines in reader.lines(){
                let hash = Sha256::digest(lines.unwrap().as_bytes());
                let human_readable_hash = format!("{:x}\n", hash);
                writer.write_all(human_readable_hash.as_bytes()).unwrap();
                
            }
            writer.flush();
        },
        "sha512" => {
            for lines in reader.lines(){
                let hash = Sha512::digest(lines.unwrap().as_bytes());
                let human_readable_hash = format!("{:x}\n", hash);
                writer.write_all(human_readable_hash.as_bytes()).unwrap();
                
            }
            writer.flush();
        }, 
        "rot13" =>{
            fn rot13_encode(text: &str) -> String {
                let mut encoded = String::with_capacity(text.len());
                
                for c in text.chars() {
                    let encoded_char = match c {
                        'A'..='M' | 'a'..='m' => (c as u8 + 13) as char,
                        'N'..='Z' | 'n'..='z' => (c as u8 - 13) as char,
                        _ => c,
                    };
                    
                    encoded.push(encoded_char);
                }
                
                encoded
            }
            for line in reader.lines() {
                let encoded_line = rot13_encode(&line.unwrap());
                let result = format!("{}\n", encoded_line);
                writer.write_all(result.as_bytes()).expect("Failed to write encoded line.");

            }
            writer.flush();
        },
        "base32" => {
            let alphabet = Alphabet::RFC4648 { padding: (false) };
            for lines in reader.lines(){
                let line = lines.unwrap();
                let encoded_line = format!("{}\n", base32Encode(alphabet, line.trim().as_bytes()));
                writer.write_all(&encoded_line.as_bytes()).unwrap();
            
            
            }
            writer.flush();
        },
        "base64" => {
            for lines in reader.lines(){
                let line = lines.unwrap();
                let encoded_line = format!("{}\n", general_purpose::STANDARD_NO_PAD.encode(line.trim()));
                writer.write_all(&encoded_line.as_bytes()).unwrap();
            
            
            }
            writer.flush();
        },
        _ => { 
            panic!("Invalid encoding type specified.")
        },
    }
}
//END ENCODE MODE

//GLOBAL FUNCTIONS
fn head_add(head: &str, material: Vec<String>) -> Vec<String>{
    let mut new_output = Vec::new();
    for value in material.iter(){
        new_output.push(format!("{}{}", head, value));
    }
    new_output
}

fn tail_add(tail: &str, material: Vec<String>) -> Vec<String>{
    let mut new_output = Vec::new();
    for value in material.iter(){
        new_output.push(format!("{}{}", value, tail));
    }
    new_output
}
fn leet(original: &str) -> String{
    let leet_speak = original.to_lowercase().replace("a", "4")
                                    .replace("g", "6")
                                    .replace("e", "3")
                                    .replace("l","1")
                                    .replace("z", "2")
                                    .replace("t", "7")
                                    .replace("o", "0")
                                    .replace("s", "5")
                                    .replace("b", "8");
    leet_speak
}
//END GLOBAL FUNCTIONS

fn write_to_file(material: Vec<String>, path_to_write: &PathBuf){
    let mut file_to_write = OpenOptions::new().create(true).write(true).append(true).open(path_to_write).unwrap();
    let mut file_writer = BufWriter::new(file_to_write);
    for i in material.iter(){
        let password = format!("{}\n", i);
        if let Err(_) = file_writer.write_all(password.as_bytes()) {
            println!("Error writing to output file");
            return;
        }
    }
    file_writer.flush();
}

fn read_file(file_path: &PathBuf) -> Vec<String>{
    let mut output_vector = Vec::new();
    let file_to_read = File::open(&file_path).unwrap();
    let file_reader = BufReader::new(file_to_read);
    for line in file_reader.lines(){
        let put_line = match line {
            Ok(value) => value,
            Err(_) => {println!("Error reading wordlist"); panic!("Error reading wordlist");},
        };
        output_vector.push(put_line.trim().to_string());
    }
    return output_vector;
}
fn file_size_calculator(size: u64){
    match size {
        0..=1023 => {
            println!("File size: {} bytes", size);
        }
        1024..=1048575 => {
            println!("File size: {:.2} KB", size as f64 / 1024.0);
        }
        1048576..=1073741823 => {
            println!("File size: {:.2} MB", size as f64 / 1048576.0);
        }
        _ => {
            println!("File size: {:.2} GB", size as f64 / 1073741824.0);
        }
    }
}
//Banner generator
fn banner(){
    let banner_random = rand::thread_rng().gen_range(1..=5);
    match banner_random{
        1 => {
            println!("7MM\"\"\"Mq.                            .d' \"\"                               ");
            println!("  MM   `MM.                           dM`                                  ");
            println!("  MM   ,M9   ,6\"Yb.  ,pP\"Ybd ,pP\"Ybd mMMmm,pW\"Wq.`7Mb,od8 .P\"Ybmmm .gP\"Ya  ");
            println!("  MMmmdM9   8)   MM  8I   `\" 8I   `\"  MM 6W'   `Wb MM' \":MI  I8  ,M'   Yb ");
            println!("  MM  YM.    ,pm9MM  `YMMMa. `YMMMa.  MM 8M     M8 MM     WmmmP\"  8M\"\"\"\"\"\" ");
            println!("  MM   `Mb. 8M   MM  L.   I8 L.   I8  MM YA.   ,A9 MM    8M       YM.    , ");
            println!(".JMML. .JMM.`Moo9^Yo.M9mmmP' M9mmmP'.JMML.`Ybmd9'.JMML.   YMMMMMb  `Mbmmd' ");
            println!("                                                         6'     dP         ");
            println!("                                                         Ybmmmd'          Made by Vulfilip       ");
            println!();
        },
        2 => {
            println!(" _______  _______  _______  _______  _______  _______  _______  _______  _______ ");
            println!("(  ____ )(  ___  )(  ____ \\(  ____ \\(  ____ \\(  ___  )(  ____ )(  ____ \\(  ____ \\");
            println!("| (    )|| (   ) || (    \\/| (    \\/| (    \\/| (   ) || (    )|| (    \\/| (    \\/");
            println!("| (____)|| (___) || (_____ | (_____ | (__    | |   | || (____)|| |      | (__    ");
            println!("|     __)|  ___  |(_____  )(_____  )|  __)   | |   | ||     __)| | ____ |  __)   ");
            println!("| (\\ (   | (   ) |      ) |      ) || (      | |   | || (\\ (   | | \\_  )| (      ");
            println!("| ) \\ \\__| )   ( |/\\____) |/\\____) || )      | (___) || ) \\ \\__| (___) || (____/\\");
            println!("|/   \\__/|/     \\|\\_______)\\_______)|/       (_______)|/   \\__/(_______)(_______/");
            println!("                                                                              Made by Vulfilip");
        },
        3 => {
            println!(" ██▀███   ▄▄▄        ██████   ██████   █████▒▒█████   ██▀███    ▄████ ▓█████ ");
            println!("▓██ ▒ ██▒▒████▄    ▒██    ▒ ▒██    ▒ ▓██   ▒▒██▒  ██▒▓██ ▒ ██▒ ██▒ ▀█▒▓█   ▀ ");
            println!("▓██ ░▄█ ▒▒██  ▀█▄  ░ ▓██▄   ░ ▓██▄   ▒████ ░▒██░  ██▒▓██ ░▄█ ▒▒██░▄▄▄░▒███   ");
            println!("▒██▀▀█▄  ░██▄▄▄▄██   ▒   ██▒  ▒   ██▒░▓█▒  ░▒██   ██░▒██▀▀█▄  ░▓█  ██▓▒▓█  ▄ ");
            println!("░██▓ ▒██▒ ▓█   ▓██▒▒██████▒▒▒██████▒▒░▒█░   ░ ████▓▒░░██▓ ▒██▒░▒▓███▀▒░▒████▒");
            println!("░ ▒▓ ░▒▓░ ▒▒   ▓▒█░▒ ▒▓▒ ▒ ░▒ ▒▓▒ ▒ ░ ▒ ░   ░ ▒░▒░▒░ ░ ▒▓ ░▒▓░ ░▒   ▒ ░░ ▒░ ░");
            println!("  ░▒ ░ ▒░  ▒   ▒▒ ░░ ░▒  ░ ░░ ░▒  ░ ░ ░       ░ ▒ ▒░   ░▒ ░ ▒░  ░   ░  ░ ░  ░");
            println!("  ░░   ░   ░   ▒   ░  ░  ░  ░  ░   ░ ░   ░ ░ ░ ▒    ░░   ░ ░ ░   ░    ░   ");
            println!("   ░           ░  ░      ░        ░             ░ ░     ░           ░    ░  ░");
            println!("                                                                  Made by Vulfilip");

        },
        4 => {
            println!(" ,ggggggggggg,");
            println!("dP\"\"\"88\"\"\"\"\"\"Y8,                                  ,dPYb,");
            println!("Yb,  88      `8b                                  IP'`Yb,");
            println!(" `\"  88      ,8P                                  I8  8I,");
            println!("     88aaaad8P\"                                   I8  8',");
            println!("     88\"\"\"\"Yb,      ,gggg,gg    ,g,       ,g,     I8 dP     ,ggggg,     ,gggggg,    ,gggg,gg   ,ggg,");
            println!("     88     \"8b    dP\"  \"Y8I   ,8'8,     ,8'8,    I8dP     dP\"  \"Y8ggg  dP\"\"\"\"8I   dP\"  \"Y8I  i8\" \"8i");
            println!("     88      `8i  i8'    ,8I  ,8'  Yb   ,8'  Yb   I8P     i8'    ,8I   ,8'    8I  i8'    ,8I  I8, ,8I");
            println!("     88       Yb,,d8,   ,d8b,,8'_   8) ,8'_   8) ,d8b,_  ,d8,   ,d8'  ,dP     Y8,,d8,   ,d8I  `YbadP'");
            println!("     88        Y8P\"Y8888P\"`Y8P' \"YY8P8PP' \"YY8P8PPI8\"8888P\"Y8888P\"    8P      `Y8P\"Y8888P\"888888P\"Y888");
            println!("                                                  I8 `8,                          Made by Vulfilip");
            println!("                                                  I8  `8,");
            println!("                                                  I8   8I");
            println!("                                                  I8   8I");
            println!("                                                  I8, ,8'");
            println!("                                                   \"Y8P'");
            println!();
        },
        _ => {
            println!("                                                  ,");
            println!("                                                  Et           :");
            println!("                                    .        .    E#t         t#,                                 ,;");
            println!("  j.                               ;W       ;W    E##t       ;##W.   j.               .Gt       f#i");
            println!("  EW,                   ..        f#E      f#E    E#W#t     :#L:WE   EW,             j#W:     .E#t");
            println!("  E##j                 ;W,      .E#f     .E#f     E#tfL.   .KG  ,#D  E##j          ;K#f      i#W,");
            println!("  E###D.              j##,     iWW;     iWW;      E#t      EE    ;#f E###D.      .G#D.      L#D.");
            println!("  E#jG#W;            G###,    L##Lffi  L##Lffi ,ffW#Dffj. f#.     t#iE#jG#W;    j#K;      :K#Wfff;");
            println!("  E#t t##f         :E####,   tLLG##L  tLLG##L   ;LW#ELLLf.:#G     GK E#t t##f ,K#f   ,GD; i##WLLLLt");
            println!("  E#t  :K#E:      ;W#DG##,     ,W#i     ,W#i      E#t      ;#L   LW. E#t  :K#E:j#Wi   E#t  .E#L");
            println!("  E#KDDDD###i    j###DW##,    j#E.     j#E.       E#t       t#f f#:  E#KDDDD###i.G#D: E#t    f#E:");
            println!("  E#f,t#Wi,,,   G##i,,G##,  .D#j     .D#j         E#t        f#D#;   E#f,t#Wi,,,  ,K#fK#t     ,WW;");
            println!("  E#t  ;#W:   :K#K:   L##, ,WK,     ,WK,          E#t         G#t    E#t  ;#W:      j###t      .D#;");
            println!("  DWi   ,KK: ;##D.    L##, EG.      EG.           E#t          t     DWi   ,KK:      .G#t        tt");
            println!("             ,,,      .,,  ,        ,             ;#t                                  ;;");
            println!("                                                   :;         Made by Vulfilip");
            println!();
        },
    }
    
}