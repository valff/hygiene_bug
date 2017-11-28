#![feature(decl_macro)]

macro impl_mod($name:ident) {
  pub mod $name {
    pub struct Bar;

    impl Bar {
      pub fn hello() {
        println!("Hello from Bar!");
      }
    }
  }
}

impl_mod!(foo);
