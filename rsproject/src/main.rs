use crate::Color::red;

fn func1() {
    print!("func1");
    return ();
}

fn make_team(u: usize) -> Result<Team,String> {
    if u<2 {
         Ok(Team::new())
    } else {
        Err("You can not make a team bigger then 2" .to_string())
    }
}

fn call_make_team() -> Result<Team,String> {
    
    let t = make_team(1) ?;

    Ok(t)
}


pub enum Color {
    red(i32),
    white(String),
}

struct Team {
    size: usize,

    manager: String,

    members: [String; 2],
}

struct Person {
    name: String,
}

trait Developer {

    fn stopCoding() {
        println!(" stop coding now");
    }

     fn code(&self);
}

trait Citizen {
    fn vote(&self , u: usize);
}

trait HumanBeing {
    fn breath();
}

impl Developer for Person {
    fn code(&self) {
        println!("I am coding {:?}" , self.name);
    }
}


impl Developer for Team {
    fn code(&self) {
        println!("Team is coding {:?}" , self.manager);
    }
}



impl Team {
    pub fn new() -> Team {
        Team {
            size: 3,
            manager: "Mahdi".to_string(),
            members: ["Ahmed".to_string(), "Omar".to_string()],
        }
    }
}

fn main() {
    // i i8 i16 i32
    // u u8 u16 u32 ...
    // b bool

    // String s

    let mut x: u128 = 10;

    x = x + 11;
    let y: u128 = 10u128;

    println!(" ---- numerical types ---- ");
    println!("x = {} y = {} ", x, y);

    let b: bool = true;

    println!(" ---- logicla types ---- ");
    println!("type bool = {} ", b);

    let mut s: String = String::new();

    s.push_str("this is a string");

    println!(" ---- complex types ---- ");
    println!("String  = {} ", s);

    let mut a = [1, 2u32, 3];

    for (i, element) in a.iter().enumerate() {
        println!(" element number {} is {}", i, element);
    }

    a[1] = 6;

    for (i, element) in a.iter().enumerate() {
        println!(" element number {} is {}", i, element);
    }

    // tuples

    let tuple_2 = (10, "bacem".to_string());

    // let tuple_5 = (10 , "bacem".to_string() , 12 , true , String::new());

    println!("tuple 2 = {} {} ", tuple_2.0, tuple_2.1);

    let mut tup0 = ();

    tup0 = func1();

    println!(" ---- Enum types ---- ");
    // Enum
    let color = Color::white("white".to_string());

    let ret = match color {
        Color::red(i) => {
            println!("color is red");
            1
        }

        _ => {
            println!("color is not red");
            0
        }
    };

    // function types

    let y = 10;

    let add = |x: i32| {
        fn f() {}
        mod t {}

        x + y
    };
    println!("adding y to 10 gives {}", add(10));

    {
        impl Team {
            pub fn set_size(&mut self, newsize: usize) {
                self.size = newsize * 100;
            }
        }
        let mut t = Team::new();
        t.set_size(20);

        println!("{} {} {:?} ", t.size, t.manager, t.members);
    }

    {
        impl Team {
            pub fn set_size2(&mut self, newsize: usize) {
                self.size = newsize * 10;
            }
        }
        let mut t = Team::new();
        t.set_size2(20);

        println!("{} {} {:?} ", t.size, t.manager, t.members);
    }

    // --------------------

    let mut ot: Option<Team> = Option::None;

    match ot {
        Some(otr) => {
            println!("{} {} {:?} ", otr.size, otr.manager, otr.members);
        }

        None => {
            println!("Team is empty");
        }
    }
    ot = Some(Team::new());

    match &ot {
        Some(otr) => {
            println!("{} {} {:?} ", otr.size, otr.manager, otr.members);
        }

        None => {
            println!("Team is empty");
        }
    }

    let t = ot.unwrap();

    ot = Option::None;
 // t -> console.log(t)

    let print = |t: Team| println!("{}" , t.size);

    let t = ot.unwrap_or_else( ||   {
        Team::new()
    } );

    println!("{} {} {:?} ", t.size, t.manager, t.members);

    t.code();

    Developer::code(&t);

    let p = Person{name: "bacem".to_string()};

    p.code();



}
