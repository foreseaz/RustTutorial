trait Food {
    fn eat(&self);
}

#[derive(Clone, Default, Debug)]
struct Apple {
    price: i32,
}

impl Apple {}

impl Food for Apple {
    fn eat(&self) {
        println!("eat apple {}", self.price);
    }
}

#[derive(Clone, Default, Debug)]
struct Udon {
    price: i32,
}
impl Udon {}

impl Food for Udon {
    fn eat(&self) {
        println!("eat Udon {}", self.price);
    }
}

type MyFood = Box<Food>;
#[derive(Default)]
struct Store {
    pub foods: Vec<MyFood>,
}

impl Store {
    fn eat(&self) {
        for a in &self.foods {
            a.eat();
        }
    }
}

fn main() {
    let mut store = Store::default();
    store.foods.push(Box::new(Apple { price: 20 }));
    store.foods.push(Box::new(Udon { price: 70 }));
    store.eat();
}
