

use std::rc::Rc;


trait Sprite{
    fn update(&self);
}


struct Monster{

}
struct Hero{

}

impl Sprite for  Monster{

    fn update(&self)
    {
        println!("monster update ...");
    }

}

impl Sprite for Hero{
    fn update(&self)
    {
        println!("Hero update ...");
    }
}

struct Game{
    elements:Vec<Rc<Sprite>>

}

impl Game{

    fn update(&self){

        for item in self.elements.iter(){
            item.update();
        }

    }

}

//这不修改成员 组合成员的 内容;问题来了，如果要修改成员的内容？怎么组织呢？
fn main() {

    let game = Game{
       elements: vec![
           Rc::new(Hero{}),
           Rc::new(Monster{})
       ]
    };

    game.update();



}

//RefCell 可以看成就是一个 指针变量，可修改的指针变量（可以多次赋值）

//Rc 可以看出一个不可以修改的指针变量，只有一次赋值