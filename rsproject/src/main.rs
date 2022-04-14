


fn main() {

    // i i8 i16 i32
    // u u8 u16 u32 ...
    // b bool

    // String s

    let mut x : u128 = 10;
    
    x = x  + 11;
    let y : u128 = 10u128;

    println!(" ---- numerical types ---- ");
    println!("x = {} y = {} " , x , y );

    let b : bool = true;

    println!(" ---- logicla types ---- ");
    println!("type bool = {} " , b );


    let mut s : String = String::new();

    s.push_str("this is a string");

    println!(" ---- complex types ---- ");
    println!("String  = {} " , s );


}
