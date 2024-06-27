use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Read;

fn main() {



    //复合类型


    let a: (u8, u64, u128) = (1, 23, 134);
    //第一种访问值方式
    //let (x, y, z) = a;

    //println!("x={x},y={y},z={z}");
//第二种
    let x = a.0;
    let y = a.1;
    let z = a.2;

    println!("x={x},y={y},z={z}");







    // let heart_eyed_cat = '😻';//喵喵喵？
    // println!("{heart_eyed_cat}");
    //
    //
    //
    // //喵喵喵？还能这么写，隐藏？？？
    // let a = 10;
    // {
    //     let a = a + 20;
    //     println!("aa = {a}");
    // }
    // println!("a={a}");






    // let i = rand::thread_rng().gen_range(1..100);
    //
    // println!("猜数字小游戏");
    // print!("请输入数字(1-100):");
    // loop {
    //     let mut a = String::new();
    //     io::stdin().read_line(&mut a).expect("null");
    //
    //     let a: u32 = a.trim().parse().expect("null");
    //
    //     match i.cmp(&a) {
    //         Ordering::Less => {
    //             println!("big")
    //         }
    //         Ordering::Equal => {
    //             println!("done");
    //             break;
    //         }
    //         Ordering::Greater => {
    //             println!("small")
    //         }
    //     };
    // }
    // print!("i={i}");
    //
}
