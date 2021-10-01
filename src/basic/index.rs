use std::sync::mpsc::channel;

pub fn hs_print() {
    println!("hihi");
}

pub fn hs_variables() {
    // ## 상수
    // mut 을 사용할 수 없으며, 전체 영역에 선언가능
    const _CONST_I8 :u32 = 100_000; // 상수의 변수 이름은 대문자로 선언!
    const _CONST_STR:&str = "hello world!";

    // ## 불변 변수
    // signed 는 i unsigned 는 u
    let _v_i8:i8 = 0;
    let _v_i16:i16 = 0;
    let _v_i32:i32 = 100_000; // 숫자 구분을 위해 _ 사용 가능!
    let _v_i64:i64 = 0;
    let _v_i128:i128 = 0; // 오 크네?
    let _v_isize:isize = 0; // 32비트 운영체제에서 4바이트, 64비트 운영체제에서 8바이트
    let _v_f32:f32 = 2.0; // 부동 소수점 타입
    let _v_f64:f64 = 3.0; //
    let _v_bool:bool = true;
    let _v_c = 'c'; // 문자 타입
    let _v_tup: (i32, f64, u8) = (500, 6.4, 1); // 튜플
    let _v_arr_i32 = [1,2,3,4,5]; // 배열
    let _v_arr_str = ["aa", "bb"];

    // ## 가변 변수
    let mut _v_i8:i8 = 12; // 위에서 선언한 불변 변수를 가변변수로 바꿀 수 있음.
    println!("{}", _v_i8);
}
// 파라미터
pub fn hs_print_x(x: i32) {
    println!("{}", x);
}
// 표현식 반환
pub fn hs_return_five() -> i32 {
    5
}
// 제어문
pub fn hs_if() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let num = if condition {
        5
    } else {
        6
    };
   println!("{}", num);
}
// 반복문
pub fn hs_loop() {
    let mut loopBreak = 2;
    loop {
        println!("loop num:{}", loopBreak);
        if loopBreak == 0 {
            break;
        }
        loopBreak = loopBreak - 1;
    }

    let mut whileBreak = 2;
    while whileBreak >= 0 {
        println!("while num:{}", whileBreak);
        whileBreak = whileBreak - 1;
    }

    let arr = [10,20,30];
    for element in arr.iter() {
        println!("for iter:{}", element);
    }

    for num in (1..4).rev() {
        println!("for range.rev:{}", num);
    }
}

// 참조자와 빌림
pub fn hs_references_borrowing() {
    // # 불변 참조자
    let s1 = String::from("hello"); // 힙한 hello!
    let len = hs_ref_borrow_calc_len(&s1); // 소유권을 넘기는 대신에 참조권한을 넘김! ( & 기호가 참조자 )
    // # 가변 참조자
    // **** 가변 참조자는 딱 한가지 큰 제한이 있다. 특정한 스코프 내에서 특정한 데이터 조각에 대한 가변 참조자를 딱 하나만 만들 수 있다. ***
    // **** 불변 참조자를 가지고 있을 동안에도 가변 참조자를 만들 수 없다! ****
    let mut s2 = String::from("hihi"); // 힙한 hihi
    hs_ref_borrow_change(&mut s2); // &mut
    // 새로운 스코프를 만들어 아래처럼 가변 참조자를 활용 할 수 있다.
    {
        let r1 = &mut s2;
    } // r1은 스코프 밖으로 벗어났으므로, 새로운 참조자를 만들 수 있다.
    let r2 = &mut s2;

}
fn hs_ref_borrow_calc_len(s: &String) -> usize {
    s.len()
}
fn hs_ref_borrow_change(s: &mut String) {
    s.push_str(", world");
}
pub fn hs_no_dangle() -> String {
    // # 댕글링 포인터
    // 댕글링 포인터란 어떤 메모리를 가리키는 포인터를 보존하는 동안, 그 메모리를 해제함으로써 다른 개체에게 사용하도록
    // 줘버렸을 지도 모를 메모리를 참조하고 있는 포인터를 말한다.
    // 러스트는 컴파일러가 모든 참조자들이 댕글링 참조자가 되지 않도록 보장해준다.
    let s = String::from("hello"); // 힙한 hello
    // s는 구문을 빠져나가면 제거가 되는데, &s (참조자)를 넘기면 댕글링 포인터가 된다.
    s
    // 그냥 s를 넘겨서 소유권을 줘버리면 문제가 없다!
}
pub fn hs_string_slice() {
    // slice는 컬렉션 전체가 아닌 컬렉션의 연속된 일련의 요소들을 참조할 수 있게 한다.
    let s = String::from("hello world!");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}",hello);
    println!("{}",world);
}