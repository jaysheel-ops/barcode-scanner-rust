use std::fs;
use rxing;

fn main() {
    let folder_path = "images"; // Replace with the actual folder path
    let mut decoded_results: Vec<String> = Vec::new();
    let mut undecoded_file_names: Vec<String> = Vec::new();
    let mut i = 0;
    for entry in fs::read_dir(folder_path).expect("Unable to read folder") {
        if let Ok(entry) = entry {
            let file_path = entry.path();

            if let Some(extension) = file_path.extension() {
                if extension == "jpg" || extension == "jpeg" || extension == "png" {
                    match rxing::helpers::detect_multiple_in_file(file_path.to_str().unwrap()) {
                        Ok(results) => {
                            println!("Scanned {i}");
                            i += 1;
                            for result in results {
                                decoded_results.push(result.getText().to_string());
                                break;
                            }
                        }
                        Err(_) => {
                            // If decoding fails, add 0 to the vector
                            decoded_results.push("0".to_string());
                            let path = file_path.to_str().unwrap();
                            undecoded_file_names.push(path.to_string());
                        }
                    }
                }
            }
        }
    }

    // Print the length of the vector
    println!("Undecoded: {:?}", undecoded_file_names);

    println!("Length of the vector: {}", decoded_results.len());

    // Count the number of 0s in the vector
    let count_of_zeros = decoded_results.iter().filter(|&x| x == "0").count();
    println!("Count of unscanned in the vector: {}", count_of_zeros);
    println!("Count of scanned in the vector: {}", decoded_results.len() - count_of_zeros);
}
