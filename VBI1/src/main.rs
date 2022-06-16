fn get_i_in_arr(val:  i32, start_i : i32, org_arr: & [i32]) -> i32 {

    for i in 0..org_arr.len()  {
        println!("value = {}", org_arr[i]);
        if val == org_arr[i] {
            return i.try_into().unwrap()
        }
    }
    return -1;
}

fn check_arr(org_arr: & [i32], sub_arr: & [i32]) -> bool {
    let mut org_i = 0;
    let mut sub_i_in_org = 0;
    let mut res = true;

    for x in sub_arr {
        println!("sub: {}", x);
        sub_i_in_org = get_i_in_arr(*x, org_i, org_arr);
        println!("sub_i_in_org: {}", sub_i_in_org);
        if sub_i_in_org>-1{
            org_i = sub_i_in_org + 1;
        }else{
            res = false;
            break;
        };
        
    }

    return res;
}

fn test() {
    // Rust 2021:
    
    let array= [1, 2,3,5,6,8, 10, 11];
    
    // This iterates by reference:
    for item in array.iter().enumerate() {
        let (i, x): (usize, &i32) = item;
        println!("array[{i}] = {x}");
    }
    
    // This iterates by value:
    for item in array.into_iter().enumerate() {
        let (i, x): (usize, i32) = item;
        println!("array[{i}] = {x}");
    }
    
}

fn main() {
    let org_arr = [1, 2,3,5,6,8, 10, 11];
    let sub_arr = [6,8, 11, 10];
    if check_arr( & org_arr,  & sub_arr){
        println!("Tap con!!");
    }else{
        println!("Khong phai!");
    };
    
}

