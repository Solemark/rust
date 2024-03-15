mod employee;
use employee::Employee;
use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use crate::employee::EmpFunctions;

fn display(employees: &Vec<Employee>) {
    println!("{:?}", employees);
}

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush!");
    stdin().read_line(input).expect("Failed to read!");
}

fn get_data(name: &mut String, phone: &mut String, email: &mut String, rate: &mut String) {
    println!("Enter new employee name: ");
    read(name);

    println!("Enter new employee phone: ");
    read(phone);

    println!("Enter new employee email: ");
    read(email);

    println!("Enter new employee rate: ");
    read(rate);
}

fn create_employee(employees: &mut Vec<Employee>) {
    let mut name: String = String::new();
    let mut phone: String = String::new();
    let mut email: String = String::new();
    let mut rate: String = String::new();

    println!("Create a new employee");
    get_data(&mut name, &mut phone, &mut email, &mut rate);

    employees.push(Employee {
        name: name.trim().parse().unwrap(),
        phone: phone.trim().parse().unwrap(),
        email: email.trim().parse().unwrap(),
        rate: rate.trim().parse().unwrap(),
    });
    display(employees);
}

fn update_employee(employees: &mut Vec<Employee>) {
    let mut input: String = String::new();
    let mut name: String = String::new();
    let mut phone: String = String::new();
    let mut email: String = String::new();
    let mut rate: String = String::new();
    let mut flag: bool = false;

    println!("Enter employee details");
    read(&mut input);

    let value: String = input.trim().parse().unwrap();
    for employee in &mut *employees {
        if value == employee.get_name() {
            flag = true;
            get_data(&mut name, &mut phone, &mut email, &mut rate);

            employee.set_name(name.trim().parse().unwrap());
            employee.set_phone(name.trim().parse().unwrap());
            employee.set_email(name.trim().parse().unwrap());
            employee.set_rate(name.trim().parse().unwrap());
        }
    }
    if flag {
        display(employees);
    } else {
        println!("Error! Couldn't find employee")
    }
}

fn search_employee(employees: &mut Vec<Employee>) {
    let mut input: String = String::new();

    println!("Enter employee details to search");
    read(&mut input);

    let value: String = input.trim().parse().unwrap();
    for employee in employees {
        if value == employee.get_name() {
            println!("found employee: {:?}", employee);
            return;
        }
    }
    println!("No employee found!");
}

fn remove_employee(employees: &mut Vec<Employee>) {
    let mut input: String = String::new();

    println!("Enter details of employee to be deleted");
    read(&mut input);

    let value: String = input.trim().parse().unwrap();
    employees.retain(|employee| employee.get_name() != value);
    display(&employees);
}

fn cli() {
    println!("Employee CLI 1.0");
    let mut employees: Vec<Employee> = vec![];
    let options: String =
        String::from("Do you want to 1. CREATE, 2. UPDATE, 3. SEARCH, 4. REMOVE or 0. EXIT");
    loop {
        let mut input: String = String::new();
        println!("{}", &options);

        read(&mut input);
        let input: i8 = input.trim().parse().unwrap();

        match input {
            1 => create_employee(&mut employees),
            2 => update_employee(&mut employees),
            3 => search_employee(&mut employees),
            4 => remove_employee(&mut employees),
            0 => exit(0),
            _ => println!("{}", &options),
        }
    }
}

fn main() {
    cli();
}
