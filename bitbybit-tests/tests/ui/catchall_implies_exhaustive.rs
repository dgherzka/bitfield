use bitbybit::bitenum;

#[bitenum(u8, exhaustive: false)]
enum Foo {
    #[catchall]
    CatchallVariant(u8),
}

fn main() {}