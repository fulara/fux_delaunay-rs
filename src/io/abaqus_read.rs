use types::*;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use std::io::Reader;

pub fn load_from_abaqus_format(path_to_file : &str) -> Result<Triangulation, &str> {
    let f = match File::open(path_to_file) {
        Ok(file) => file,
        Err(e) => return Err("potato")
    };

    let mut fbuf = BufReader::new(f);
    load_from_abaqus_format_buf(fbuf);

    Err("potato2")
}

fn load_from_abaqus_format_buf(buffor : Reader) {
}

fn read_buf<R : Read> (buffor : BufReader<R>) {
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        //let s = "str";
        //let mut reader = BufReader::new(s);
        //load_from_abaqus_format_buf()
    }
}