



fn do_something(s : String) -> (String , usize) {
    let i = s.len();
    (s,i)
}

fn push_to_string( mut s:  String) -> String  {
    s.push_str("bacem");

    s
}

fn push_to_string_with_ref(   s: &mut String)  {
    s.push_str("bacem with ref");
}

fn make_string( ) -> String {
    "bacem".to_string()
}

//'a

fn make_string_ref<'a> (s1: &'a mut String  , s2: &'a String) -> &'a String {
    if 6 == 6  {
        s1.push_str(s2.as_str());
        s1
    } else {
        s2
    }

}




fn main() {
    println!("Hello, world!");

    // copied types

    let var1 : usize = 14;

    let var2 = var1; // no move

    println!("var 1 = {} " , var1);


    let name1 = "bacem".to_string();

    let mut name2 = name1.clone();

    println!("printing name1 {:?}" , name1);

    (name2,_) = do_something(name2);
    println!("printing name2 {:?}" , name2);
   

    // reference
    let ref1: &String;
    let name3  = name1; // 'a
        {
            let s = "bacem".to_string(); // 'b
            ref1 =  &s;
        }

    let  mut empty :  String = String::new();
        
    empty = push_to_string(empty);
    
    println!("printing name {:?}" , empty);


    let mut empty = String::new();
    push_to_string_with_ref(&mut empty);
    println!("printing name {:?}" , empty);


    let b = make_string();

    let ref1 = &mut "bacem2".to_string(); 
    let ref3 ;
    
    let ref2 = &"bacem1".to_string();
    ref3 = make_string_ref( ref1 , ref2);
       
    
    println!("printing name {:?}" , ref3);
    

}
