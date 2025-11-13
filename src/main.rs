// Closure in rust
// closure in rust is basically an anonymous function , that is function that can be created without giving it a name
// one advantage of closer is that we can store them and pass them around as functions,

//  when the enviromental vairable are captured through immutable borrow, the closure is considerd to be implementing the fn trait
//  when the vairable are captured through mutable borrow the closure implements the FN mute trait
//  When tranfers of ownership is involve, the closure is suppose to implement Fn once

struct Employee {
    name: String,
    salary: u32,
    department: String
}

fn process_employees<V1, V2>(employees: Vec<Employee>, name_transform:V1, salary:V2) -> Vec<String>
where 
V1:Fn(&str)-> String,
V2:Fn(u32) -> bool

{
     let mut processed_names= Vec::new()
    for employee in employees{
        if salary(employee.salary){
            processed_names.push(name_transform(&employee.name));
        }
    }

    processed_names
}
fn main(){
    let employees= vec![
        Employee {
             name: String::from("James Victor"),
             salary: 5_000,
             department: String::from("Engineering")
        },
        Employee { name: String::from("Bob"), salary: 75000, department: String::from("Sales"), },
        Employee { name: String::from("Charlie"), salary: 50000, department: String::from("Marketing"), }
    ];

    let transform_name_to_uppercase = |name: &str| name.to_uppercase();
    let filter_salary_above_threshold = |salary:u32| salary > 6_000;

   let process= process_employees(employees, transform_name_to_uppercase, filter_salary_above_threshold);


}


