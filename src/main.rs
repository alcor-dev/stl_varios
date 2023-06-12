use byteorder::{self, LittleEndian, ReadBytesExt};
use stl_varios::execute_analysis;
use std::{fs, env, io, str};
use std::io::{prelude::*, Error};
use std::fs::File;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    //los args comienzan con el comando, no con los atributos introducidos, por eso pasamos al 1
    let filename : &str = &args[1];
    
    execute_analysis(filename);
}
