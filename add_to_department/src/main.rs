use std::io;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    // let employee = get_employee();
    // let department = get_department(&employee);
    let input = get_user_input();
    let employee = input.0;
    let department = input.1;

    println!("We got {} from {}", &employee, &department);

    let mut hr : HashMap<String, String>= HashMap::new();

    hr.insert(
        employee.to_string(),
        department.to_string()
    );
    println!("{:?}", hr)
}

fn get_user_input() -> (String, String){

    let employee = get_employee();
    let department = get_department(&employee);
    
    return (employee, department);
}



fn get_employee() -> String{

    println!("So, which employee to add?");

    let mut employee = String::new();
    
    io::stdin()
    .read_line(&mut employee)
    .expect("failed to read line: employee"); //get back and do with result enum?
    
    return employee;
}


fn get_department(employee: &String) -> String{
    println!("gotcha {}... from which department again??", employee);
    
    let mut department = String::new(); 

    io::stdin()
    .read_line(&mut department)
    .expect("failed to read line: department");

    return department;
}
