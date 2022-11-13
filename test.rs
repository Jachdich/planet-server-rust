struct A {
    f: Option<String>,
}

impl A {
    fn get_f(&mut self) -> &str {
        match &self.f {
            Some(f) => &f,
            None => {
                let val = "some string".to_string();
                self.f = Some(val);
                &self.f.as_ref().unwrap()
            }
        }
    }
}

fn main() {
    let mut a = A { f: None };
    println!("{}", a.get_f());
}
