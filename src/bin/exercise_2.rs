use regex::Regex;

fn main() {
    let mut s1 = String::new();
    let mut s2 = String::new();
    std::io::stdin().read_line(&mut s1);
    std::io::stdin().read_line(&mut s2);
    s1.pop();
    s2.pop();

    let mut number_occurennce = count_occurence(&s1, &s2);
    println!("'{}' appears {} times", s2, number_occurennce);

    number_occurennce = count_occurrence_using_regrex(&s1, &s2);
    println!("'{}' appears {} times", s2, number_occurennce);

}

fn count_occurrence_using_regrex(s: &str, w: &str) -> usize {
    let expression = format!(r"(?i){}", w);

    let re = Regex::new(&expression).unwrap();
    return re.find_iter(&s).count();
}

fn count_occurence(s : &str, w: &str) -> usize {
    println!("{} --- {}", s, w);

    let s_vec: Vec<char> = s.chars().collect();
    let w_vec: Vec<char> = w.chars().collect();

    let s_vec_len = s_vec.len();
    let w_vec_len = w_vec.len();
    let mut i = 0;
    let mut j = 0;

    let mut count = 0;

    while i < s_vec_len {
        if s_vec[i] == w_vec[j] {
            j += 1;
            
            if j == w_vec_len { 
                count += 1;
                j = 0;
            }
        } else {
            j = 0;
        }

        i += 1;
    }

    return count;
}