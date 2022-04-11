use crate::cross::cross_main_func;
 

pub fn call_lib_func() {

    let number = cross_main_func();
    println!("number returned from function is {} " , number );

     println!("this function was called from a library inside a module");

     let number2 =  {

        println!("code inside curly braces");
            {
                println!("code inside inside curly braces")
            };

         {
             println!("just before returning 13");
             {
                 println!("last time, promised ! ");
                (13 , 14)
             }
             
         }
     };

     println!("number returned from curly braces is {} {} " , number2.0 , number2.1 );

 }