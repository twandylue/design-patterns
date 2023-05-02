// Mediator
// ref:
// 1. https://refactoring.guru/design-patterns/mediator/rust/example
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let train1 = PassengerTrain::new("train1".to_string());
    let train2 = FreightTrain::new("train2".to_string());

    let mut station = TrainStation::default();

    station.accept(train1);
    station.accept(train2);

    station.depart("train1");
    station.depart("train2");
    station.depart("test train");
}

trait Mediator {
    fn notify_about_arrival(&mut self, train_name: &str) -> bool;
    fn notify_about_departure(&mut self, train_name: &str);
}

#[derive(Default)]
struct TrainStation {
    trains: HashMap<String, Box<dyn Train>>,
    train_queue: VecDeque<String>,
    train_on_platform: Option<String>,
}

impl TrainStation {
    fn accept(&mut self, mut train: impl Train + 'static) {
        if self.trains.contains_key(train.name()) {
            println!("'{name}' has already arrived", name = train.name());

            return;
        }

        train.arrive(self);
        self.trains
            .insert(train.name().to_string(), Box::new(train));
    }

    fn depart(&mut self, name: &'static str) {
        let train = self.trains.remove(name);
        if let Some(mut train) = train {
            train.depart(self);
        } else {
            println!("'{name}' is not on the station");
        }
    }
}

impl Mediator for TrainStation {
    fn notify_about_arrival(&mut self, train_name: &str) -> bool {
        match self.train_on_platform {
            Some(_) => {
                self.train_queue.push_back(train_name.to_string());
                false
            }
            None => {
                self.train_on_platform.replace(train_name.to_string());
                true
            }
        }
    }

    fn notify_about_departure(&mut self, _train_name: &str) {
        match &self.train_on_platform {
            Some(_) => {
                self.train_on_platform = None;
                if let Some(next_train_name) = self.train_queue.pop_front() {
                    // NOTE:
                    let mut next_train = self.trains.remove(&next_train_name).unwrap();
                    next_train.arrive(self);
                    self.trains.insert(next_train_name.clone(), next_train);

                    self.train_on_platform = Some(next_train_name);
                }
            }
            None => (),
        }
    }
}

trait Train {
    fn name(&self) -> &str;
    fn arrive(&mut self, mediator: &mut dyn Mediator);
    fn depart(&mut self, mediator: &mut dyn Mediator);
}

struct FreightTrain {
    name: String,
}

impl FreightTrain {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl Train for FreightTrain {
    fn name(&self) -> &str {
        &self.name
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_about_arrival(&self.name) {
            println!(
                "Freight tain '{name}': Arrival blocked, waiting...",
                name = self.name
            );
            return;
        }

        println!("Freight train '{name}': Arrived", name = self.name)
    }

    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("Freight tain '{name}': Leaving", name = self.name);
        mediator.notify_about_departure(&self.name);
    }
}

struct PassengerTrain {
    name: String,
}

impl PassengerTrain {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl Train for PassengerTrain {
    fn name(&self) -> &str {
        &self.name
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_about_arrival(&self.name) {
            println!(
                "Passenger tain '{name}': Arrival blocked, waiting...",
                name = self.name
            );
            return;
        }

        println!("Passenger train '{name}': Arrived", name = self.name)
    }

    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("Passenger tain '{name}': Leaving", name = self.name);
        mediator.notify_about_departure(&self.name);
    }
}
