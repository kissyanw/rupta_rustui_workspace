use oop_rs::class;

#[class]
type Class = class<
    {
        let r#move: i32 = 0;

        #[allow(dead_code)]
        fn r#move(&self) {}
    },
>;
