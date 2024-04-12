fn main() -> usize{
    let mut s = String::from("hello");
    let mut ss = s.push_str(", world!");
    let bytes = &s.as_bytes(); //array of bytes
    
    println!("{:?}", bytes); 

    for(i ,&item) in bytes.iter().enumerate(){
        //println!("{}: {}", i, item); // 0: 104, 1: 101, 2: 108, 3: 108, 4: 111
        if item == b' '{
            return i;
        }
    }
    s.len()
}