use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String>{
    //reads input line by line, collecting each line into a vector
    let mut line_vec = Vec::new();
    for line in read_to_string(filename).unwrap().lines(){
        line_vec.push(line.to_string());
    };
    line_vec
}

fn increasing(level_list: &Vec<i64>) -> bool{
    for i in 0..(level_list.len()-1){
        if level_list[i] >= level_list[i+1]{
            return false;
        }
        else{
            let difference:i64 = level_list[i]-level_list[i+1];
            if difference < -3 || difference > 0{
                return false;
            }
        }
    }
    true
}

fn increasing_p2(unlevel_list: &Vec<i64>) -> bool{
    let mut level_list = unlevel_list.clone();
    for i in 0..(level_list.len()-1){
        if level_list[i] >= level_list[i+1]{
            if level_list.len() == unlevel_list.len(){
                level_list.remove(i);
                if increasing(&level_list){
                    return true;
                }
                else if decreasing(&level_list){
                    return true;
                }
            }
            return false;
        }
        else{
            let difference:i64 = level_list[i]-level_list[i+1];
            if difference < -3 || difference > 0{
                if level_list.len() == unlevel_list.len(){
                    level_list.remove(i);
                    if increasing(&level_list){
                        return true;
                    }
                    else if decreasing(&level_list){
                        return true;
                    }
                }
                return false;
            }
        }
    }
    true
}


fn decreasing(level_list: &Vec<i64>) -> bool{
    for i in 0..(level_list.len()-1){
        if level_list[i] <= level_list[i+1]{
            return false;
        }
        else{
            let difference:i64 = level_list[i]-level_list[i+1];
            if difference > 3{
                return false;
            }
        }
    }
    true
}

fn decreasing_p2(unlevel_list: &Vec<i64>) -> bool{
    let mut level_list = unlevel_list.clone();
    for i in 0..(level_list.len()-1){
        if level_list[i] <= level_list[i+1]{
            if level_list.len() == unlevel_list.len(){
                level_list.remove(i);
                if decreasing(&level_list){
                    return true;
                }
                else if increasing(&level_list){
                    return true;
                }
            }
            return false;
        }
        else{
            let difference:i64 = level_list[i]-level_list[i+1];
            if difference > 3{
                if level_list.len() == unlevel_list.len(){
                    level_list.remove(i);
                    if decreasing(&level_list){
                        return true;
                    }
                    else if increasing(&level_list){
                        return true;
                    }
                }
                return false;
            }
        }
    }
    true
}

fn main() {
    let file_path = "input.txt";
    //let file_path = "sampleinput.txt";
    let mut level_vec: Vec<Vec<i64>> = Vec::new(); //empty vector to house all levels, typecasted
    let mut result = 0;
    for elem in read_lines(file_path){
        //reading the each line and stripping whitespaces and typecasting into i64
        //each line becomes its own vector
        let temp_vec: Vec<i64> = elem.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        level_vec.push(temp_vec);
    }

    for index in 0..level_vec.len(){
        if increasing(&level_vec[index]){
            println!("Increasing: {:?}", level_vec[index]);
            result += 1;
        }
        else if decreasing(&level_vec[index]){
            println!("Decreasing: {:?}", level_vec[index]);
            result += 1;
        }
        else if increasing_p2(&level_vec[index]){
            println!("Increasing cut: {:?}", level_vec[index]);
            result += 1;
        }
        else if decreasing_p2(&level_vec[index]){
            println!("Decreasing cut: {:?}", level_vec[index]);
            result += 1;
        }
    }

    println!("{:?}", result);
    
}
