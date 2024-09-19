use boot_core::Bootstrap;

pub struct Print(Vec<String>);

impl Bootstrap for Print {
    fn run(self) {
        self.0.iter().for_each(|x| println!("{}", x));
    }
}


