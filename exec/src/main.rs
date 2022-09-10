
#![allow(unused)]
fn main() {
    enum GeneratorState<Y, R> {
        Yielded(Y),
        Complete(R),
    }

    trait Generator {
        type Yield;
        type Return;
        fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
    }

    enum GeneratorA<'a> {
        Enter,
        Yield1 {
            to_borrow: String,
            borrowed: &'a String, // uh, what lifetime should this have?
        },
        Exit,
    }

    impl GeneratorA<'_> {
        fn start() -> Self {
            GeneratorA::Enter
        }
    }

    impl Generator for GeneratorA<'_> {
        type Yield = usize;
        type Return = ();
        fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
            // lets us get ownership over current state
            match std::mem::replace(self, GeneratorA::Exit) {
                GeneratorA::Enter => {
                    let to_borrow = String::from("Hello");
                    let borrowed = &to_borrow; // <--- NB!
                    let res = borrowed.len();

                    let y = GeneratorA::Yield1 {to_borrow, borrowed};
                    if let GeneratorA::Yield1 {ref to_borrow, mut borrowed} = y {
                        borrowed = to_borrow;
                    }
                    *self = y;
                    GeneratorState::Yielded(res)
                }

                GeneratorA::Yield1 {to_borrow, borrowed} => {
                    println!("Hello {}", borrowed);
                    *self = GeneratorA::Exit;
                    GeneratorState::Complete(())
                }
                GeneratorA::Exit => panic!("Can't advance an exited generator!"),
            }
        }
    }
}
