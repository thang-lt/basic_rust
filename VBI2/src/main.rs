// fn main(){

// }

//Exercise 1
// Mục đích: giải quyết vấn đề ownership and borrowing không dùng clone()
// fn main() {
//     let mut input :u32 = 20;
//     let x = change_value(10, &mut input);
//     println!("{}", x);
// }



// fn change_value(input:u32, output: &mut u32) -> u32{
//     if input ==1 {
//         *output = 3;
//     }
//     else {
//         *output = 4;
//     }

//     *output
// }


//Exercise 2
// Mục đích: giải quyết vấn đề ownership và borrowing ko dùng clone()
// Các bạn có thể sửa thêm logic để đúng với mục đichs bài này là liệt kê các số nguyên tố 
fn main() {
    let mut count: u32 = 1;
    let mut num: u64 = 1;
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);

    while count < 10 {
        num += 2;
        if vector_is_prime(num, &primes) {
            count += 1;
            primes.push(num);
        }
    }
    println!("{:?}", primes);
}

fn vector_is_prime(num: u64, p: &Vec<u64>) -> bool {
    for i in p {
        if num > *i && num % i == 0 {
            return false;
        }
    }

    true
}



//Exercise 3
// Mục đích: giải quyết vấn đề ownership and borrowing ko dùng clone()
// fn main() {
//     let mut values = vec![10, 11, 12];
//     let v = & values;

//     let mut max = 0;
    
//     for n in v {
//         max = std::cmp::max(max, *n);
//     }

//     println!("max is {}", max);
//     println!("Converting to percentages of maximum value...");

//     let v = &mut values;
//     for n in v {
//         *n = 100 * (*n) / max;
//     }
//     println!("values: {:#?}", values);
// }


//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
// fn main(){
//     let mut a = vec![1,2,3,4,5];
//     let b = test(&mut a);
//     println!("Vector sau khai dao : {:?}", b);
// }

// pub fn test(a: &mut Vec<u8>) -> Vec<u8> {
//     let mut b:Vec<u8>  = Vec::new();

//     while let Some(top) = a.pop() {
//         b.push(top);
//     }

//     // loop {
//     //     if a.len() == 0 { break; }
//     //     let d = a.pop().unwrap();
//     //     b.push(d);
//     // }
//     b
// }