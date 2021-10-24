// based on https://stackoverflow.com/a/63676216
struct Thing<S> {
    strategy: S,
}

impl<S> Thing<S>
where
    S: Strategy,
{
    fn do_something(&self) {
        self.strategy._do();
    }
}

trait Strategy {
    fn _do(&self);
}

struct StrategyA;

impl Strategy for StrategyA {
    fn _do(&self) {
        println!("A");
    }
}

struct StrategyB;

impl Strategy for StrategyB {
    fn _do(&self) {
        println!("B");
    }
}
