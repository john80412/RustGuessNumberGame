use rand::{thread_rng, Rng};
use std::io;
fn main() {
    let mut v = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let check = String::from("0123456789");
    let mut rng = thread_rng();
    let mut ans = String::new();
    let mut inp = String::new();
    let mut cnt = 0;
    for e in 0..4 {
        let r = rng.gen_range(0, v.len());
        ans += &v[r].to_string();
        v.remove(r);
    }
    println!("{}", ans);
    while inp != ans {
        let mut correctformat = true;
        let mut anum = 0;
        let mut bnum = 0;
        inp = String::new();
        println!("請輸入不重複4位數字");
        io::stdin().read_line(&mut inp);
        inp = inp.trim_right().to_string();
        if(inp.len() != 4) {
            println!("輸入長度不是4位,請重新輸入");
            continue;
        }
        for (i, c) in inp.chars().enumerate() {
            if let Some(e) = inp[i + 1..].find(c) {
                println!("有重複數字,請重新輸入");
                correctformat = false;
                break;
            }
            if check.find(c) == None {
                println!("有非數字字元,請重新輸入");
                correctformat = false;
                break;
            }
            match ans.find(c) {
                Some(pos) if pos == i => anum += 1,
                Some(pos) => bnum += 1,
                None => {}
            }
        }
        if(correctformat) {
            cnt += 1;
            println!("{} A {} B", anum, bnum);
        }
    }
    println!("恭喜答對\n總共猜了 {} 次",cnt);
}

fn IsDuplicate(inp: &String) -> bool {
    for (i, c) in inp.chars().enumerate() {
        if let Some(pos) = inp[i + 1..].find(c) {
            return true;
        }
    }
    false
}
