fn main() {
   let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
   let sub_arr = [8, 10, 11];
  
    let is_subarray = is_subarray(&org_arr, &sub_arr);
    println!("Is a sub-array: {}", is_subarray);
}

fn is_subarray(org_arr: &[i32], sub_arr: &[i32]) -> bool {
    let org_arr_len = org_arr.len();
    let sub_arr_len = sub_arr.len();
    
    let mut i = 0;
    let mut j = 0;
    
    while i < org_arr_len && j < sub_arr_len {
        if org_arr[i] == sub_arr[j] {
            i += 1;
            j += 1;
            
            if j == sub_arr_len { 
                return true;
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    
    return false;
}