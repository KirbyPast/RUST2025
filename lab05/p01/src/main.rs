use std::fs;
/*struct Student {
    name: String,
    number: String,
    age: i32
}*/

fn main() {
    let s = fs::read_to_string("src/input.txt");
    let mut i = 0;
    let mut stud_array: [(&str, &str, &str); 100] = [("", "", ""); 100];
    let binding = s.unwrap();
    for line in binding.lines() {
        let mut j = 0;
        for word in line.split(",") {
            if j == 0 {
                stud_array[i].0 = word;
                j = j + 1;
            }
            else if j == 1 {
                stud_array[i].1 = word;
                j = j + 1;
            }
            else if j == 2 {
                stud_array[i].2 = word;
                j = j + 1;
            }
        }
        i=i+1;
    }
    let mut idx = 0;
    let mut youngest = 999;
    let mut youngest_idx = 0;
    let mut oldest = -1;
    let mut oldest_idx = 0;
     
    while idx < i {
        if stud_array[idx].2.parse::<i32>().unwrap() > oldest {
            oldest_idx = idx;
            oldest = stud_array[idx].2.parse::<i32>().unwrap();
        }
        if stud_array[idx].2.parse::<i32>().unwrap() < youngest {
            youngest_idx = idx;
            youngest = stud_array[idx].2.parse::<i32>().unwrap();
        }
        idx =idx + 1;
    }
    println!("{:?} is the oldest student, aged {:?}", stud_array[oldest_idx].0, stud_array[oldest_idx].2);
    println!("{:?} is the youngest student, aged {:?}", stud_array[youngest_idx].0, stud_array[youngest_idx].2);

}
