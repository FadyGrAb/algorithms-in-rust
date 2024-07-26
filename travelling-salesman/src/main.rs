use itertools::Itertools;
use scilib::coordinate::cartesian::Cartesian;
use std::fmt;

struct City {
    name: String,
    coordinates: Cartesian,
}

impl City {
    fn new(name: &str, coor: (f32, f32, i32)) -> Self {
        Self {
            name: name.to_string(),
            coordinates: Cartesian::from(coor.0, coor.1, coor.2),
        }
    }
    fn distance(&self, rhs: &City) -> f64 {
        self.coordinates.distance(rhs.coordinates)
    }
}

impl fmt::Debug for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.name.clone())
    }
}
fn main() {
    let cities = [
        City::new("Egypt", (26.8206, 30.8025, 0)),
        City::new("Sudan", (12.8628, 30.2176, 0)),
        City::new("Lybia", (26.3351, 17.2283, 0)),
        City::new("UAE", (23.4241, 53.8478, 0)),
        City::new("Iran", (32.4279, 53.6880, 0)),
    ];

    let perm = cities.iter().permutations(cities.len());
    let mut select_path = Vec::new();
    let mut select_dist = f64::MAX;
    let mut steps = 0;
    for p in perm {
	steps += 1;
        let mut total_distance = 0.0;
        for i in 0..p.len() {
            if i == p.len() - 1 {
                break;
            };
            total_distance += p[i].distance(p[i + 1]);
        }
        if total_distance < select_dist {
            select_dist = total_distance;
            select_path = p.clone();
        }

        println!(">>>>>>> {:?} {}", p, total_distance);
    }

    println!(
        "SELECTED: {:?} {} in {} steps",
        select_path, select_dist, steps
    )
}
