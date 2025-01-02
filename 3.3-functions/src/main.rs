fn main() {
    another_function();
    function_with_parameter(2);
    print_labeled_measurement(5,'h');
    expression_and_statement();

    let x = return_function();
    println!("x: {x}");
}

// 함수명은 스네이크 케이스(snake case) 방식을 사용한다.
// 소문자 + 밑줄(underscore)로 단어를 구분하는 방식이다.
fn another_function() {
    println!("Another function.");
}

fn function_with_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// `Rust`는 표현식 기반 프로그래밍 언어다.
fn expression_and_statement() {
    // 함수를 정의하는 것도 구문이다.
    // 구문은 값을 반환하지 않는다.
    let y = {
        let x = 3;
        x + 1 // 표현식은 세미 콜론을 사용하지 않는다.
    };

    println!("The value of y is: {y}");
}

fn return_function() -> i32 {
    5
}