trait Command {
    fn new(data: String) -> SimpleCommand;
    fn execute(&self);
}

struct SimpleCommand {
    data: String,
}

impl Command for SimpleCommand {
    fn new(data: String) -> SimpleCommand {
        return SimpleCommand { data };
    }

    fn execute(&self) {
        println!("Data: {}", self.data);
    }
}
