
use std::fmt::Debug;

#[derive(Copy,Clone,Debug)]
struct A{
     num:i32,
     age:u8,
//     name:String // 由于Clone采用了深拷贝
}

struct Container<T>{
    elements:Vec<T>, //封闭的系统看不到 Rc  RefCell Arc
    index:usize

}

impl<T:Copy+Sized> Container<T>
{

    fn new() -> Container<T>
    {
        Container {
            elements: Vec::new(),
            index: 0,
        }
    }


    fn add(&mut self, t: T)
    {
        self.elements.push(t);
    }
}


impl<T:Sized+Copy> Iterator for Container<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>
    {
        match self.index < self.elements.len() {
            true  => {
                let t = self.elements[self.index];


                self.index += 1;
                Some(t)
            }
            false => None,
        }
    }
}


fn main(){
    let mut c: Container<A> = Container::<A>::new();
    c.add(A{num:100,age:12});
    c.add(A{num:110,age:12});
    c.add(A{num:120,age:12});

    for i in c {
        println!("{:#?}", i);
    }


}