use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String>{
    //Read input line by line, collecting each line into a vector
    let mut line_vec = Vec::new();
    for line in read_to_string(filename).unwrap().lines(){
        line_vec.push(line.to_string());
    };
    line_vec
}

fn main() {
    let file_path = "input.txt";
    let mut right_vec: Vec<i64> = Vec::new(); //empty vector for all right columns
    let mut left_vec: Vec<i64> = Vec::new();  //empty vector for all leftt columns

    //read input and split text file into the right and left columns and typecast into i64
    for elem in read_lines(file_path){
        let temp_vec: Vec<&str> = elem.split("   ").collect();
        right_vec.push(temp_vec[0].parse().unwrap());
        left_vec.push(temp_vec[1].parse().unwrap());
    }

    //cloning for part 2 :p
    let mut right_sorted = right_vec.clone();
	let mut left_sorted = left_vec.clone();

    //sorting both vectors :D
    right_sorted.sort_unstable();
    left_sorted.sort_unstable();

    //creating an empty vector to store differences
    let mut sum_vec: Vec<i64> = Vec::new();
    
    //check if both vectors are of same length, subtract right from left and push into sum_vec
    if right_sorted.len() == left_sorted.len(){
        for index in 0..right_sorted.len(){
            let difference: i64 = right_sorted[index] - left_sorted[index];
            sum_vec.push(difference.abs()); //pushing absolute values
        }
    }

    //finding sum of sum_vec
    let sum:i64 = sum_vec.iter().sum();
    println!("Sum of differences: {}", sum);
    
    
    // -------- Part 2 :D ----------------

    let mut right_in_left = HashMap::new(); //initalise a hashmap to hold number of occurences

    for index in 0..left_vec.len(){
        let occurence_count = left_vec.iter()   //for each in left
        .filter(|&n| *n == right_vec[index])    //filter if in right - only result in common elements
        .count();                               //count occurences

        right_in_left.insert(right_vec[index], occurence_count as i64);
    }

    //store similarity
    let mut similarity_vec: Vec<i64>  = Vec::new();

    //for key and value in HashMap, push key * value [simliarity as given] into vector
    for(k,v) in right_in_left{
        similarity_vec.push(k*v);
    }

    let similarity_scores: i64 = similarity_vec.iter().sum(); //collate all similarities into similarity score

    println!("Similarity Score: {}", similarity_scores);

}
