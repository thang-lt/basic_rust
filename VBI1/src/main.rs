fn get_i_in_arr(val:  i32, start_i : i32, org_arr: & [i32]) -> i32 {
    // Get index Sub_array's char in Org_array
    let index = start_i as usize;
    for i in index..org_arr.len()  {
        if val == org_arr[i] {
            return i.try_into().unwrap()
        }
    }
    return -1;
}

fn check_arr(org_arr: & [i32], sub_arr: & [i32]) -> bool {
    // sub array
    let mut org_i = 0;
    let mut sub_i_in_org:i32;
    let mut res = true;

    for x in sub_arr {
        sub_i_in_org = get_i_in_arr(*x, org_i, org_arr);
        if sub_i_in_org>-1{
            org_i = sub_i_in_org + 1;
        }else{
            res = false;
            break;
        };
        
    }

    return res;
}

fn main() {
    let org_arr = [1, 2,3,5,6,8, 10, 11];
    let sub_arr = [6,8, 10, 1];
    if check_arr( & org_arr,  & sub_arr){
        println!("Right!!");
    }else{
        println!("Not right!");
    };
    
}

