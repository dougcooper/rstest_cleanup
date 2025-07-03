fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use ctor::dtor;
    use rstest::{fixture, rstest};
    use lazy_static::lazy_static;


    struct Foo{
        bar: i32
    }

    lazy_static! {
        static ref FOO: Arc<Mutex<Foo>> = Arc::new(Mutex::new(Foo { bar: 42 })); // Foo = Foo{bar: 42};
    }

    #[fixture]
    #[once]
    fn foo() -> &'static Arc<Mutex<Foo>> {
        &FOO
    }

    #[rstest]
    fn it_works(foo: &Arc<Mutex<Foo>>) {
        assert_eq!(foo.lock().unwrap().bar, 42);
        foo.lock().unwrap().bar = 0;
    }

    #[dtor]
    fn cleanup() {
        //this will crash if it's not 0
        assert_eq!(FOO.lock().unwrap().bar, 0);
    }
}
