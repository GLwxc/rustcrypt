extern crate clap;

use clap::{Arg, App};

pub struct ArgParser {
    pub mode: String,
    pub cypher: String,
    pub padder: String,
    pub action: String,
    pub file_path: String,
    pub key_path: String,
}

impl ArgParser {

    pub fn new() -> Self {
        // basic app information
        let app = App::new("rustcrypt parser")
            .version("1.0")
            .about("a cryptography program")
            .author("Guillaume");

        let name_option = Arg::with_name("padder")
            .long("padder") // allow --padder
            .short("p") // allow -p
            .takes_value(true)
            .help("give a padder name please")
            .required(true);
        
        let app = app.arg(name_option);
        
        let name_option = Arg::with_name("cypher")
            .long("cypher") // allow --cypher
            .short("c") // allow -c
            .takes_value(true)
            .help("give cypher name please")
            .required(true);
        
        let app = app.arg(name_option);

        let name_option = Arg::with_name("mode")
            .long("mode") // allow --mode
            .short("m") // allow -m
            .takes_value(true)
            .help("give a mode name please")
            .required(true);
        
        let app = app.arg(name_option);
        
        let name_option = Arg::with_name("action")
            .long("action") // allow --action
            .short("a") // allow -a
            .takes_value(true)
            .help("give a action name please")
            .required(true);
        
        let app = app.arg(name_option);

        let name_option = Arg::with_name("key_path")
            .long("key_path") // allow --key_path
            .short("k") // allow -k
            .takes_value(true)
            .help("give a key path please")
            .required(true);
        
        let app = app.arg(name_option);

        // Extract the actual name
        let name_option = Arg::with_name("file_path")
            .long("file_path") // allow --ffile_path
            .short("f") // allow -f
            .takes_value(true)
            .help("give a file_path please")
            .required(true);
        
        let app = app.arg(name_option);
        
        let matches = app.get_matches();
        
        let padder = matches.value_of("padder")
            .expect("This can't be None, we said it was required");
        
        let cypher = matches.value_of("cypher")
            .expect("This can't be None, we said it was required");
        
        let mode = matches.value_of("mode")
            .expect("This can't be None, we said it was required");
        
        let action = matches.value_of("action")
            .expect("This can't be None, we said it was required");

        let key_path = matches.value_of("key_path")
            .expect("This can't be None, we said it was required");
        
        let file_path = matches.value_of("file_path")
            .expect("This can't be None, we said it was required");
        
        ArgParser { 
            padder: padder.to_string(),
            cypher: cypher.to_string(),
            mode: mode.to_string(),
            action: action.to_string(),
            key_path: key_path.to_string(),
            file_path: file_path.to_string(),
        }
    }
}
