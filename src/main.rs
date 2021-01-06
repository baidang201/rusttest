use std::ops::Mul;

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            Self::Red => 10,
            Self::Green => 60,
            Self::Yellow => 5,
        }
    }
}

fn sum(rg: &[u32]) ->Option<u32> {
    let mut s : u32  = 0;
    for item in rg.iter() {
        match s.checked_add(*item) {
            Some(v) => {
                s = v;
            },
            None=>{
                return None
            },
        }
    }
    Some(s)
}


struct Cicle<T> {
    r: T,
    pi: T,
}

struct Square<T>{
    w: T,
    h: T,
}

struct Triangel<T> {
    bottom: T,
    h: T,
    half: T,
}

pub trait Area<T> {
    fn area(&self) -> T;
}

impl<T> Area<T> for Cicle<T>
    where T: Mul<Output=T> + Copy{
    fn area(&self) -> T{
        self.pi* self.r *self.r
    }
}
impl<T> Area<T> for Square<T>
    where T: Mul<Output=T> + Copy {
    fn area(&self) -> T{
        self.w*self.h
    }
}
impl<T> Area<T> for Triangel<T>
    where T: Mul<Output=T> + Copy {
    fn area(&self) -> T{
        self.bottom*self.h*self.half
    }
}

pub fn print_area<T, U>(item: T) 
    where T: Area<U>, U: Mul<Output=U> + Copy + std::fmt::Display  {
    println!("area is {}", item.area());
}

fn main() {
    //imp test
    let light = TrafficLight::Red;
    println!("time is {}", light.time());
    let light = TrafficLight::Yellow;
    println!("time is {}", light.time());


    //match and checked_addtest
    let rg1: [u32; 5] = [1,2,3,4,5];
    match sum(&rg1) {
        Some(value) => {println!("sum is {}", value);}
        None => {println!("sum is {}", "None");}
    }

    let rg2 : [u32; 6] = [1,2,3,4,5, u32::max_value()];  
    match sum(&rg2) {
        Some(value) => {println!("sum is {}", value);}
        None => {println!("sum is {}", "None");}
    }

    //Generic
    let c = Cicle {r: 2.0, pi: 3.14};
    let s = Square {w:2.2, h:3.2};
    let t = Triangel {bottom: 2.0, h:3.0, half: 0.5};

    print_area(c);
    print_area(s);
    print_area(t);

}
