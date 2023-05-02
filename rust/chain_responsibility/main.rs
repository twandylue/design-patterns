// Chain of Responsibility
// ref:
// 1. https://refactoring.guru/design-patterns/chain-of-responsibility/rust/example
fn main() {
    let mut cashier = Cashier::default();
    let mut medical = Medical::new(cashier);
    let mut doctor = Doctor::new(medical);
    let mut reception = Reception::new(doctor);

    let mut patient = Patient {
        name: "John".to_string(),
        ..Patient::default()
    };

    reception.execute(&mut patient);

    println!("\nThe patient has been already handled:\n");

    reception.execute(&mut patient);
}

fn into_next(department: impl Department + 'static + Sized) -> Option<Box<dyn Department>> {
    Some(Box::new(department))
}

trait Department {
    fn execute(&mut self, patient: &mut Patient) {
        self.handle(patient);

        if let Some(next) = &mut self.next() {
            next.execute(patient);
        }
    }

    fn handle(&mut self, patient: &mut Patient);
    fn next(&mut self) -> &mut Option<Box<dyn Department>>;
}

/// Request
#[derive(Default)]
struct Patient {
    name: String,
    registration_done: bool,
    doctor_check_up_done: bool,
    medical_done: bool,
    payment_done: bool,
}

/// Department/Cashier
#[derive(Default)]
struct Cashier {
    next: Option<Box<dyn Department>>,
}

impl Department for Cashier {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.payment_done {
            println!("Payment done")
        } else {
            println!(
                "Cashier getting money from a patient: {name}",
                name = patient.name
            );
            patient.payment_done = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}

/// Department/Doctor
#[derive(Default)]
struct Doctor {
    next: Option<Box<dyn Department>>,
}

impl Doctor {
    fn new(next: impl Department + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Department for Doctor {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.doctor_check_up_done {
            println!("Doctor check up done")
        } else {
            println!("Doctor checking a patient: {name}", name = patient.name);
            patient.doctor_check_up_done = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}

/// Department/Medical
#[derive(Default)]
struct Medical {
    next: Option<Box<dyn Department>>,
}

impl Medical {
    fn new(next: impl Department + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Department for Medical {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.medical_done {
            println!("Medicine is already given to a patient")
        } else {
            println!(
                "Medical giving medicine to a patient: {name}",
                name = patient.name
            );
            patient.medical_done = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}

/// Department/Reception
#[derive(Default)]
struct Reception {
    next: Option<Box<dyn Department>>,
}

impl Reception {
    fn new(next: impl Department + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Department for Reception {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.registration_done {
            println!("Patient registration is already done")
        } else {
            println!(
                "Reception registering a patient: {name}",
                name = patient.name
            );
            patient.registration_done = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}
