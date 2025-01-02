fn main() {
    let result = if_expression(3);
    println!("result boolean: {result}");

    else_if_expression(3);

    let condition = false;
    let_with_if(condition);
    loop_expression();
    loop_with_label();
    while_expression();
    for_collection();
}

fn if_expression(number: i32) -> bool {
    if number < 5 {
        println!("condition was true");
        true
    } else {
        println!("condition was false");
        false
    }
}

fn else_if_expression(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn let_with_if(condition: bool) {
    // let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {number}");
}

fn loop_expression() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {result}");
}

fn loop_with_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End Count = {count}");
}

fn while_expression() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}