use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

fn main() {
    let mut count = 0u32;
    let mut cids = cid();
    loop {
        count += 1;
        if cids.1 == cids.0 {
            println!("Found a match!");
            break;
        } else {
            cids = cid();
            continue;
        }
    }
}

fn cid() -> (String, String) {
    let text: String = rand_text();
    let record_path: &str = "./src/record.json";
    let mut cat: Vec<String> = read_lines(record_path);
    //changes the text value to the newly generated random string
    let text_lhs = String::from("  \"text\": \"");
    let text_rhs = String::from("\",");
    cat[2] = text_lhs + &text + &text_rhs;

    //creates a new File struct with the new record
    let new_record_path = Path::new("./src/new_record.json");
    let mut new_record = match File::create(&new_record_path) {
        Err(why) => panic!("Couldn't access {}: {}", new_record_path.display(), why),
        Ok(new_record) => new_record,
    };

    //generates the body of the new record from cat
    let mut body: String = String::from("");
    for line in &cat {
        body = body + &line + "\n";
    }

    //writes the body with the new text to the new record
    let _ = new_record.write_all(body.as_bytes());

    //generates the new CID
    let output = Command::new("ipfs")
        .arg("dag")
        .arg("put")
        .arg(new_record_path)
        .output();
    let mut cid = String::from_utf8(output.unwrap().stdout).unwrap();
    cid.pop();

    let cid_lhs = String::from("      \"cid\": \"");
    let cid_rhs = String::from("\",");
    cat[9] = cid_lhs + &cid + &cid_rhs;

    body = String::from("");
    for line in &cat {
        body = body + &line + "\n";
    }
    body.pop();
    let mut new_record = match File::create(&new_record_path) {
        Err(why) => panic!("Couldn't access {}: {}", new_record_path.display(), why),
        Ok(new_record) => new_record,
    };
    let _ = new_record.write_all(body.as_bytes());

    let new_output = Command::new("ipfs")
        .arg("dag")
        .arg("put")
        .arg(new_record_path)
        .output();
    let mut new_cid = String::from_utf8(new_output.unwrap().stdout).unwrap();
    new_cid.pop();
    println!("{}\n{}", cid, new_cid); //DEBUG
    return (cid, new_cid);
}

fn rand_text() -> String {
    let range = 300;

    let l = thread_rng().gen_range(1..range);
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(l)
        .map(char::from)
        .collect();

    return rand_string;
}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
