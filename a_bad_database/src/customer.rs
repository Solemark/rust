#[derive(Debug)]
pub struct Customer {
    pub id: usize,
    pub name: String,
}

pub fn get_by_name(name: String) -> Vec<Customer> {
    get_data()
        .into_iter()
        .filter(|c| c.name == name)
        .collect::<Vec<Customer>>()
}

pub fn get_by_id(id: usize) -> Vec<Customer> {
    get_data()
        .into_iter()
        .filter(|c| c.id == id)
        .collect::<Vec<Customer>>()
}

pub fn new_customer(customer: Customer) -> Result<bool, std::io::Error> {
    let mut customers = get_data();
    customers.push(customer);
    write_data(customers)
}

fn get_data() -> Vec<Customer> {
    let mut customers = vec![];
    let res = std::fs::read_to_string("data/customer.csv").expect("Failed to get Customers!");
    for line in res.lines() {
        let data = line.split(',').collect::<Vec<&str>>();
        customers.push(Customer {
            id: data[0]
                .parse::<usize>()
                .expect("Customer id is not a number!"),
            name: String::from(data[1]),
        });
    }
    customers
}

fn write_data(customers: Vec<Customer>) -> Result<bool, std::io::Error> {
    let mut file =
        std::fs::File::create("data/customers.csv").expect("Failed to create customers.csv");
    let mut output = String::new();
    for customer in customers {
        output += &format!("{},{}\n", customer.id, customer.name);
    }
    match std::io::Write::write_all(&mut file, output.as_bytes()) {
        Ok(()) => Ok(true),
        Err(e) => Err(e),
    }
}
