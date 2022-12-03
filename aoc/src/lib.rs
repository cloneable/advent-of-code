use std::io;

pub trait LineParser {
    fn start(&mut self) {}
    fn parse_line(&mut self, line: &str);
    fn finish(&mut self) {}
}

pub fn read_from_stdin<T: LineParser>(parser: &mut T) {
    let mut line = String::new();
    loop {
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                parser.finish();
                break;
            }
            Ok(_) => parser.parse_line(&line.trim()),
            Err(e) => {
                println!("Err: {}", e);
                break;
            }
        }
        line.clear();
    }
}
