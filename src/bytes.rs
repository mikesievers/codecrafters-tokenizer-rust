use std::fs;

pub fn print_file_as_bytes(args: &[String]) {
    let fname = &args[2];
    let bytes = bytes_from_file(fname);
    let bytes_json = serde_json::to_string(&bytes).unwrap();
    println!("{bytes_json}");
}

fn bytes_from_file(fname: &String) -> Vec<u8> {
    fs::read(fname).unwrap()
}
