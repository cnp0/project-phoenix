const MILES_PER_KILOMETER: f32 = 0.621371;

trait MilesThing {
    fn print_miles(&self);
}

struct ImperialDistanceThing {
    miles: f32,
}

impl MilesThing for ImperialDistanceThing {
    fn print_miles(&self) {
        println!("{} Miles", self.miles);
    }
}

trait KilometersThing {
    fn print_kilometers(&self);
}

struct MetricDistanceThing {
    kilometers: f32,
}

impl KilometersThing for MetricDistanceThing {
    fn print_kilometers(&self) {
        println!("{} Kilometers", self.kilometers);
    }
}

struct DistanceThingAdapter {
    thing: MetricDistanceThing,
}

impl MilesThing for DistanceThingAdapter {
    fn print_miles(&self) {
        let miles = self.thing.kilometers * MILES_PER_KILOMETER;

        println!("{} Miles", miles);
    }
}
