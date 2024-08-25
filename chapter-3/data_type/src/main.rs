use std::io;

fn main() {
    let mut index = String::new();
    
    let my_array : [i32;5] = [1;5];
    
    
    io::stdin()
        .read_line(&mut index)
        .expect("Error: reading input");
    
    let index : usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    
    
    
    let item : i32 = my_array[index];
    
    
    println!("{index},{item}");
}







// fn main() {
//     println!("Hello, Data Type!");
//     let a : isize = 50;
//     let b : isize = 500; // my cpu arch is 64 so the b is unsigned 64 integer
//     let c : isize = a + b; // i can't add i64 + u64 or i64 + isize just same + same = same
    
//     let f1 : f32 = 1.0111111111111111011111111111119; // max 1.0111111
//     let f2 : f64 = 1.0111111111111113; // max 1.011111111111111
//     //let v = f1 + f2; can't done different type
//     // tuple
//     let vec3 = (5,6,8);

//     log_vec3(vec3);
    
//     log_f64(f2);
    
//     let array : [f64; 5] = [1.1;5];
    
//     log_array_5len(array);
    
    
    
//     println!("c {c} - \nf1 {f1} \nf2 {} \n {}",f2 , vec3.1);
    
// }

// fn log_vec3(vec3:(i64, i64, i64)) {
//     println!("(x:{}, y:{}, z:{})", vec3.0, vec3.1, vec3.2);
// }
// fn log_f64(value: f64) {
//     let m = value;
//     println!("{}",m);
// }
// fn log_array_5len(array :[f64;5]) {
//     println!("[{},{},{},{},{}]",array[0], array[1],array[2],array[3],array[4]);
// }