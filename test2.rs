struct S {
    z: i32,
}

trait T {
    fn get_z(&self) -> i32 {
        self.z
    }
}

impl T for S {}

fn main() {
    let s = S { z: 1 };
    println!("{}", s.get_z());
}
