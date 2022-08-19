use std::fs;

pub fn read_file_content(filename: String) -> Vec<String> {
    let filename2 = filename.as_str();
    if let Ok(_metadata) = fs::metadata(filename2) {
        fs::read_to_string(filename2).unwrap().split("\n").map(|e: &str| String::from(e)).collect()
    }
    else {
        vec![]
    }
}

pub fn write<'a>(filename: String, data: &'a mut Vec<String>) {
    let mut data_to_write = read_file_content(filename);

    data_to_write.append(data);
}
