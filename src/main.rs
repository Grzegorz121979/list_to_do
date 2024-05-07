use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};

fn main() -> io::Result<()> {
    let list: Vec<String> = env::args().collect();
    let mut lines: Vec<String> = Vec::new();
    let path_file = "list_to_do.txt";

    let mut file = OpenOptions::new()
                            .append(true)
                            .open(path_file)?;
    
    if list[1] == "add" {
        let mut i = 2;
        while i < list.len() {
            file.write_all(list[i].as_bytes())?;
            let _ = file.write_all(b"\n");
            i += 1;
        }
    } else if list[1] == "print" {
        file = File::open(path_file)?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;

        println!("{}", contents);
    } else if list[1] == "remove" {
        file = File::open(path_file)?;
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            let line = line?;
            lines.push(line);
        }

        for (i, element) in lines.iter().enumerate() {
            if element == &list[2] {
                lines.remove(i);
                break;
            } else {
                println!("Error");
            }
        }

        file = File::create(path_file)?;

        for value in &lines {
            writeln!(file, "{}", value)?;
        }
    } else if list[1] == "clear" {
        file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(path_file)?;
        
        file.set_len(0)?;
    }

    Ok(())
}


