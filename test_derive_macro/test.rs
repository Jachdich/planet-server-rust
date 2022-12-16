trait MyTrait {
    fn a(&self);
}

struct A;
impl A {
    fn a(&self) {
        println!("Called from struct");
    }
}

impl MyTrait for A {
    fn a(&self) {
        println!("Called from trait");
    }
}

fn main() {
    let something = A;
    something.a(); //I would like this to print "called from struct"
}
