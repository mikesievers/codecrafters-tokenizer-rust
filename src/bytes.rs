use std::fs::read_to_string;

pub fn print_file_as_bytes(args: &[String]) {
    let fname = &args[2];
    let data = read_to_string(fname).unwrap();
    let bytes = bytes_from_string(&data);
    let bytes_json = serde_json::to_string(&bytes).unwrap();
    println!("{bytes_json}");
}

pub fn bytes_from_string(data: &str) -> Vec<u8> {
    data.as_bytes().to_vec()
}
