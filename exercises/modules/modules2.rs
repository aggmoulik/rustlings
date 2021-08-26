// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)

// I AM DONE

mod delicious_snacks {
    use self::fruits::PEAR as fruit;
    use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }

    pub fn getFruit() -> String{
        self::fruit.to_string()
    }

    pub fn getVeggie() -> String{
        self::veggie.to_string()
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::getFruit(),
        delicious_snacks::getVeggie()
    );
}
