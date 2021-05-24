fn main() {
    let a_number = 10;
    let a_boolean = true;

    // !는 함수를 일반적인 메서드가 아닌 매크로로 사용하고 있음을 Rust에 알립니다.
    // 가변 개수의 인수를 포함하는 함수로 간주할 수 있습니다.
    println!("The number is {}.", a_number);
    println!("The boolean is {}.", a_boolean);

    // 가변성
    // Rust에서는 기본적으로 변수 바인딩을 변경할 수 없습니다.
    // a_number = 15; // ==> 에러

    // 값을 변경하려면 mut 키워드를 사용하여 변수 바인딩을 변경할 수 있도록 설정해야 합니다.
    let mut b_number = 10;
    println!("The number is {}.", b_number);
    b_number = 15;
    println!("Now the number is {}.", b_number);
    
    // 섀도잉
    // 이전 변수와 이름이 같은 새 변수를 선언하여 새 바인딩을 만들 수도 있습니다.
    // 새 변수는 이전 변수를 섀도잉합니다. 이전 변수는 여전히 존재하지만 이 범위에서는 더 이상 참조할 수 없습니다.
    
    let number = 5;
    let number = number + 5;
    let number = number * 2;
    println!("The number is {}.", number); // 20

    // str과 String
    // str은 문자열 데이터의 뷰. 대부분의 경우 &str 형식을 사용.
    // &str을 변경할 수 없는 문자열에 대한 포인터로 생각할 수 있다.
    // String은 변경될 수 있는 문자열 데이터. 힙에 할당된다.

    // Create a String from a string literal
    let mut hello = String::from("Hello, ");  

    // Push a character into our String
    hello.push('w');
    
    // Push a string literal into our String       
    hello.push_str("orld!");
             
    println!("{}", hello)
}
