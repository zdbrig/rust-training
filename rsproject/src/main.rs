


use crate::Color::red;

fn func1()  {
    print!("func1");
    return ();
}

pub enum Color {
    red(i32),
    white(String)
}


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

    let  mut a  = [ 1 , 2u32  , 3];

    for (i , element) in a.iter().enumerate() {
        println!(" element number {} is {}" , i , element);
    }

    a[1] = 6;

    for (i , element) in a.iter().enumerate() {
        println!(" element number {} is {}" , i , element);
    }


    // tuples

    let tuple_2 = (10 , "bacem".to_string());

    // let tuple_5 = (10 , "bacem".to_string() , 12 , true , String::new());

    println!("tuple 2 = {} {} " , tuple_2.0 , tuple_2.1);

    let  mut tup0 = ();

    tup0 =  func1();

    println!(" ---- Enum types ---- ");
    // Enum 
    let color = Color::white("white".to_string());

    let ret =  match color {
        Color::red(i)  =>   {
            println!("color is red");
            1
         } ,

         _ => {
            println!("color is not red");
            0
         }
     };

     // function types 

     let y = 10;

     let  add = |x:i32|   {
                                            fn f() {

                                            }
                                            mod t  {

                                            }
                                            
                                             x + y  
                                    };  
     println!("adding y to 10 gives {}" , add(10));

    
   

}
