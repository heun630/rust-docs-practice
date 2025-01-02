static mut COUNTER: i32 = 0;
static COUNTER_NONE_MUT: i32 = 0;

fn main() {
    let x = mutability_and_address();
    println!("return value of x: {}", x);
    println!("Address of x: {:p}", &x);

    static_variable();
    shadowing();
}

fn mutability_and_address() -> i64{
    // `mut`는 이 변수가 이후에 변경될 가능성이 있음을 의미한다.
    let mut x = 5;
    // println!("The value of x is {}", x);
    println!("Address of x (mutable let): {:p}", &x);

    // shadowing 변수
    x = 6;
    // println!("The value of x is {}", x);
    println!("Address of x (shadowing let): {:p}", &x);

    // `const`의 이름은 반드시 대문자로 작성하고, 언더스코어(_)로 구분하는 것이 관례다.
    // 또한, `const`는 타입을 명시해야 한다.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let const_address = &THREE_HOURS_IN_SECONDS;
    println!("Address of const THREE_HOURS_IN_SECONDS: {:p}", const_address);

    const TWO_HOURS_IN_SECONDS: u32 = 60 * 60 * 2;
    let const_two_address = &TWO_HOURS_IN_SECONDS;
    println!("Address of const TWO_HOURS_IN_SECONDS: {:p}", const_two_address);

    x
}

fn static_variable() {
    // `static mut`는 전역적으로 변경 가능한 데이터를 정의할 때 사용된다.
    // 하지만 데이터 경합(Data Race)의 가능성 때문에 Rust는 이를 안전하지 않은(unsafe) 것으로 간주한다.
    // warning: creating a shared reference to mutable static is discouraged
    // `static mut`는 반드시 `unsafe{}` 안에서만 접근할 수 있다.
    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }

    println!("Counter: {}", COUNTER_NONE_MUT);
}

fn shadowing() {
    let x = 5;
    println!("The value of x is: {x}");

    let y = x + 1;
    println!("The value of y is: {y}");

    {
        let z = x * 2;
        // {z} = {}, z
        println!("The value of z in the inner scope is: {z}");
    }
}