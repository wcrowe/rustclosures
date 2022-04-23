#![allow(unused_imports, dead_code)]

#[derive(Debug)]
struct ClassicCars {
    make: &'static str,
    models: Vec<(&'static str, i32)>,
}

impl ClassicCars {
    fn smart_get<F>(&self, f: F)
    where
        F: Fn(&Vec<(&'static str, i32)>),
    {
        f(&self.models)
    }
}

fn main() {
    let car_collection = vec![
        ("Thunderbird", 1956),
        ("Cobra", 1964),
        ("GT-350", 1966),
        ("Mustang", 1965),
    ];
    let ford_models = ClassicCars {
        make: "Ford",
        models: car_collection,
    };
    ford_models.smart_get(|x| {
        let res: Vec<_> = x.into_iter().filter(|x| x.1 > 1960).collect();
        println!("{:?}", res);
        let res: Vec<_> = x.into_iter().map(|x| format!("{}_{}", x.1, x.0)).collect();
        println!("{:?}", res);
    });
}
