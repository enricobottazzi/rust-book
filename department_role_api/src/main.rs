use std::io;
use std::collections::HashMap;

fn main() {

    // create a hash map with department as key and vector of employees as value
    let mut db : HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please enter the name of the employee");
        let name = read_input();
        println!("Please enter the department of the employee");
        let department = read_input();

        add_to_db(&mut db, name, department);

        println!("enter department name to retrieve all its employees");
        let department = read_input();
        fetch_employee_by_department(&db, department);

        println!("enter all to retrieve all employees");
        let input = read_input();
        if input.trim() == "all" {
            fetch_all_employees(&db);
        }
    }

}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
    input
}

fn add_to_db(db: &mut HashMap<String, Vec<String>>, name: String, department: String) {
    // handle the case where the department is not found
    let employees = db.entry(department).or_insert(Vec::new());
    employees.push(name);
}

fn fetch_employee_by_department(db: &HashMap<String, Vec<String>>, department: String) {

    match db.get(&department) {
        Some(employees) => {
            for employee in employees {
                println!("{}", employee);
            }
        },
        None => println!("No employees found in the department"),
    }
}

fn fetch_all_employees(db: &HashMap<String, Vec<String>>) {

    for (department, employees) in db {
        println!("Department: {}", department);
        for employee in employees {
            println!("{}", employee);
        }
    }

}

