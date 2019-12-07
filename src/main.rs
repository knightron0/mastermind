use std::io;

fn check1(x: i32, y:i32) -> i32{
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut n = x;
    let mut m = y;
    while n != 0{
        vec1.push(n % 10);
        n = n/10;
    }
    while m != 0{
        vec2.push(m % 10);
        m = m/10;
    }
    let mut ans = 0;
    for i in 0..6 {
        if vec1[i] == vec2[i]{
            ans+=1;
        }
    }
    return ans;
}
fn check2(x: i32, y:i32) -> i32{
    let mut ans = 0;
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut n = x;
    let mut m = y;
    while n != 0{
        vec1.push(n % 10);
        n = n/10;

    }
    while m != 0{
        vec2.push(m % 10);
        m = m/10;
    }
    let mut vec3 = Vec::new();
    let mut vec4 = Vec::new();
    for i in 0..6 {
        if vec1[i] != vec2[i]{
            vec3.push(vec1[i]);
            vec4.push(vec2[i]);
        }
    }

    for i in 0..vec3.len() { 
        for j in 0..vec4.len(){
            if vec3[i] == vec4[j]{
                vec4.remove(j);
                ans += 1;
                break;
            }
        }
    }
    return ans;
}

fn main(){
    let mut poss = Vec::new();
    for i in 100000..1000000{
        poss.push(i);
    }
    let mut last = 111222;
    let mut x;
    let mut y;
    while poss.len() > 0{
        println!("{}", last);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut n: i32 = input.trim().parse().unwrap();
        y = n % 10;
        n = n/ 10;
        x = n;
        if x == 6 && y == 0 {
            println!("I guessed it! {} was the answer!", last);
            break;
        }
        poss.retain(|&i| check1(last, i) == x && check2(last, i) == y);
        last = poss[0];
        
    }

}

