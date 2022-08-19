use std::fs;

pub fn read_file_content(filename: &str) -> Vec<&str> {
    if let Ok(metadata) = fs::metadata(filename) {
        fs::read_to_string(filename).unwrap().split("\n").collect()
    }
    else {
        let empty: Vec<&str> = vec![];
        empty
    }
}

pub fn write(filename: &str, &'a mut data: Vec<&str>) {
    let mut data_to_write = read_file_content(filename);

    data_to_write.append(data);
}
