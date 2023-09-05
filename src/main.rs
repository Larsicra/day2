use std::io;

// buffer
struct Buffer<T: Copy + std::ops::Add<Output = T>> {
    data: Vec<T>,                                       // buffer
}

impl<T: Copy + std::ops::Add<Output = T>> Buffer<T> {
    fn sum (&self) -> Option<T> {                       // return option, None if the vector is empty
        if self.data.len() == 0 {
            None                                        // return None if empty
        } else {
            let mut res = self.data[0].clone();
            for i in 1..self.data.len() {
                res = res + self.data[i].clone();
            }
            Some(res)                                   // return the sum otherwise
        }
    }
}


// CompareString
fn cmp_str(x: &str, y: &str) -> bool {
    let x_char: Vec<char> = x.chars().collect();            // get the vector of the chars
    let y_char: Vec<char> = y.chars().collect();            // get the vector of the chars

    for i in 0..x_char.len() {
        if i == y_char.len() {
            return true;                    // if y is part of x (shorter than x, and is same as x before)
        }
        if x_char[i] > y_char[i] {          // if y is smaller than x at the i-th char
            return true;
        } else if x_char[i] < y_char[i] {
            return false;                   // if y is larger than x at the i-th char
        }
    }
    false                                   // if x is part of y (shorter than y, and is same as y before)
}


// 闭包
fn func(vec: Vec<char>) -> Vec<char> {
    let iter = vec.iter().map(|x|->char {       // iterator
        let temp: u8 = u32::from(*x).try_into().unwrap();           // char -> u8
        char::from(temp + 1)                                        // u8 -> char after +1
    });
    iter.collect()                      // collect the new vector
}


fn main() {
    println!("Buffer");
    let vec_i: Vec<i32> = vec![1, 2, 3, 4];                 // Buffer test on i32
    println!("i32 type vec: {:?}", vec_i);
    let buf_i = Buffer{
        data: vec_i
    };
    println!("i32 type sum: {:?}", buf_i.sum());        
    println!("");


    let vec_f: Vec<f32> = vec![1.1, 2.2, 3.1];              // Buffer test on f32
    println!("f32 type vec: {:?}", vec_f);
    let buf_f = Buffer{
        data: vec_f
    };
    println!("f32 type sum: {:?}", buf_f.sum());
    println!("");


    let vec_e: Vec<i64> = Vec::new();                       // Buffer test on empty vector
    println!("empty vec:    {:?}", vec_e);
    let buf_e = Buffer{
        data: vec_e
    };
    println!("empty sum:    {:?}", buf_e.sum());

    println!("---------------------------------");            // 闭包
    println!("闭包");
    let vec = vec!['a', 'b', 'c', 'd', 'e'];
    println!("before: {:?}", vec);
    println!("after:  {:?}", func(vec));

    println!("");
    println!("---------------------------------");            // CompareString
    let mut a = String::new();
    let mut b: String = String::new();

    println!("输入比较的第一个字符串");
    io::stdin().read_line(&mut a);
    println!("输入比较的第二个字符串");
    io::stdin().read_line(&mut b);
    
    println!("{:?}", cmp_str(&a[..], &b[..]));

}