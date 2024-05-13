use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};

pub fn print_list(path: &str) -> Result<String, io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    
    Ok(contents)
}

pub fn append_item_to_file(path: &str, append_item: &String) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    let value_str = append_item.to_string();
    file.write_all(value_str.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}

pub fn remove_item_from_list(path: &str, item_to_remove: &String) -> Result<(), io::Error> {
    let mut list:Vec<String> = Vec::new(); 
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let line = line?;
        list.push(line);
    }

    for (index, item) in list.iter().enumerate() {
        if item == item_to_remove {
            list.remove(index);
            break;
        }
    }

    let mut new_file = File::create(path)?;

    for item in &list {
        writeln!(new_file, "{}", item)?;
    }

    Ok(())
}

pub fn clear_list(path: &str) -> Result<(), io::Error> {
    let file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(path)?;
        
    file.set_len(0)?;

    Ok(())
}
