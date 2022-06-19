// -------Exercise 1-----------
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
    let mut sub_i_in_org:i32 = 0;
    let mut res = true;

    for x in sub_arr {
        sub_i_in_org = get_i_in_arr(*x, sub_i_in_org, org_arr);
        if sub_i_in_org>-1{
            sub_i_in_org += 1;
        }else{
            res = false;
            break;
        };
    }

    return res;
}

fn ex1_draft (){
    let org_arr = [1, 6,3,5,6,8, 6, 11];
    let test_arr = [6,8, 6, 11];
    if check_arr( & org_arr,  & test_arr){
        println!("The test array is a child of root array!!!");
    }else{
        println!("The test array is not a child of root array!!");
    };
}

fn ex1 (){
    let org_arr = [1, 6,3,5,6,8, 6, 11, 34, 67];
    let sub_arr = [8, 6, 11];
    let len_org_arr = org_arr.len();
    let len_sub_arr = sub_arr.len();
    let mut result:bool = false;

    if len_org_arr >= len_sub_arr{
        'outer: for org_item in org_arr.iter().enumerate(){
            let (o_i, o_x): (usize, &i32) = org_item;
            if len_org_arr - o_i >= len_sub_arr{
                if *o_x == sub_arr[0]{
                    
                    let mut right_num = 0;
                    // Check another element
                    '_inner: for sub_item in sub_arr.iter().enumerate(){
                        let (s_i, s_x): (usize, &i32) = sub_item;
                        let mut org_v = org_arr[o_i+s_i];
                        if org_v == *s_x{
                            // Count numbers right element
                            right_num += 1;

                            if (right_num) == len_sub_arr{
                                result = true;
                                break 'outer;
                            }
                        }else{
                            right_num = 0;
                        } 
                    }
                }
            }
        }
    }

    if result {
        println!("The test array is a child of root array!!!");
    }else{
        println!("The test array is not a child of root array!!");
    }
}

// -------Exercise 2-----------

fn ex2(){
    let org_string = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    let input = "is";
    let input_len = input.len();
    let org_string_len = org_string.len();

    // A counter variable
    let mut n = 0;

    // Loop while `n` is less than 101
    if org_string_len >= input_len{
        while n < org_string.len() {
            let mut index = org_string.find(input);
            println!("Not right!");
            // let guess: i32 = index.try_into().unwrap();
            n += 1;
        }
    }
}

fn compare_sub_string(s1: &str, s2: String) -> (String, i32) {
    let mut s3 = String::new();
    let mut count =0 ;
    for s in s1.split_whitespace(){
        if s == s2 {
            count += 1;
            s3 = s.to_string();
        }
    }

    (s3, count)
}

fn main() {
    ex1();
    // ex2();
    
}

