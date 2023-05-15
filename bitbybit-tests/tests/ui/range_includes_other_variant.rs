use bitbybit::bitenum;

#[bitenum(u8, exhaustive: false)]
enum Foo {
    #[range(0x00..=0x20)]
    RangeVariant1(u8),

    Variant(u8) = 0x10,
}

fn main() {}