use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let urls = get_urls()?;
    println!("{:?}", urls);


    Ok(())
}

fn get_urls() -> io::Result<Vec<String>> {
    let mut file_path = String::new();    
    println!("Enter File Path!");
    io::stdin().read_line(&mut file_path)?;
    let contents = fs::read_to_string(file_path.trim())?;
    let lines: Vec<&str> = contents.lines().collect();
    let mut urls = Vec::new();
    for line in lines {
        let temp: Vec<&str> = line.split_whitespace().collect();
        if temp[0].eq("*"){
            urls.push(temp[2].to_string());
        }

    }

    Ok(urls)
}
