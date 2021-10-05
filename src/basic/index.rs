use std::sync::mpsc::channel;
use std::fmt::Display;
use std::time::Duration;
use std::thread;

pub fn hs_print() {
    println!("hihi");
}

pub fn hs_variables() {
    // ## 상수
    // mut 을 사용할 수 없으며, 전체 영역에 선언가능
    const _CONST_I8: u32 = 100_000; // 상수의 변수 이름은 대문자로 선언!
    const _CONST_STR: &str = "hello world!";

    // ## 불변 변수
    // signed 는 i unsigned 는 u
    let _v_i8: i8 = 0;
    let _v_i16: i16 = 0;
    let _v_i32: i32 = 100_000; // 숫자 구분을 위해 _ 사용 가능!
    let _v_i64: i64 = 0;
    let _v_i128: i128 = 0; // 오 크네?
    let _v_isize: isize = 0; // 32비트 운영체제에서 4바이트, 64비트 운영체제에서 8바이트
    let _v_f32: f32 = 2.0; // 부동 소수점 타입
    let _v_f64: f64 = 3.0; //
    let _v_bool: bool = true;
    let _v_c = 'c'; // 문자 타입
    let _v_tup: (i32, f64, u8) = (500, 6.4, 1); // 튜플
    let _v_arr_i32 = [1, 2, 3, 4, 5]; // 배열
    let _v_arr_str = ["aa", "bb"];

    // ## 가변 변수
    let mut _v_i8: i8 = 12; // 위에서 선언한 불변 변수를 가변변수로 바꿀 수 있음.
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

    let arr = [10, 20, 30];
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
    println!("{}", hello);
    println!("{}", world);
}

pub fn hs_etc_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn hs_struct() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("another@example.com"), String::from("anotherusername567"));
    let user3 = User {
        email: String::from("ano1@example.com"),
        username: String::from("anot123"),
        ..user2 // 나머지 필드를 user2 값으로 채운다. 이 구문은 맨 마지막에서만 유효하다.
    };

    let black = Color(0, 0, 0); // 튜플 구조체 사용방법!

    // 구조체의 인스턴스가 모든 데이터를 소유하기를 원하기 때문에 전체 구조가 유효한 데이터이어야 한다.
    // 때문에 구조체에서 &str을 사용하려면 수명을 지정해야 한다.
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // 구조체 변수랑 파라미터 이름이 같으면 그냥 넣으면 된다.
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);

pub fn hs_struct_fn() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let rect2 = Rectangle::square(3);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectange 구조체의 구현
impl Rectangle {
    // &self로 구조체 필드를 사용할 수 있다. (읽기 권한!)
    // 쓰기 권한을 원하면 &mut self를 사용!
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // &self가 없으면 Rectangle::square(3) <-- 이런식으로 사용한다.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn hs_lifetime() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    let lv = i.level();
    /*
    컴파일러는 명시적 주석이 없을때, 세가지 규칙을 사용하여 언제 라이프타임 참조를 갖는지 알아낸다.
    첫번째 규칙은 입력 수명에 적용되며, 두번째 및 세번째 규칙은 출력 수명에 적용된다.

    첫번째 규칙은 참조인 각 매개변수가 고유한 수명을 갖는다. fn foo<'a>(x: &'a i32)
    매개 변수가 두개인 함수는 fn foo<'a, 'b>(x: &'a i32, y: &'b i32) ---> 출력 수명 매개변수가 존재할 경우 명시적으로 지정해야 한다.

    두번째 규칙은 정확히 하나의 입력 수명 매개변수가 있는 경우, 모든 출력 수명 매개변수에 할당된다.
    fn foo<'a>(x: &'a i32) -> &'a i32

    세번째 규칙은 출력 수명은. &self 또는 &mut self 이다! ( 구조체 메소드일때는 &self 수명을 따라간다! )
    */
}

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    // 'a는 수명 매개변수! 여러개일 경우, <'a, 'b> 이런식으로 작성!
    // 반환되는 값은 x의 라이프 타임을 따라간다!
    x
    // 정적 수명은 let s: &'static str = "I have a static lifetime."; 으로 지정할 수 있다.
}

// 참조를 보유하기 위한 구조체는 수명을 추가해야 한다!
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 수명이 있는 구조체에 메서드를 구현하려면 아래 처럼 해야 한다!! impl 옆에 수명 이름이 있다.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// 구현을 이런식으로 분할할 수도 있으나 권장하지는 않음.
impl<'a> ImportantExcerpt<'a> {
    // announce 에 라이프타임을 생략가능하다! &self를 따라가기 때문에 announcement의 라이프타임은 'a 이 된다.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 컴파일러는 세 가지 규칙을 사용하여 언제 라이프타임 참조를 갖는지 알아낸다.
// 명시적 주석이 없다.
// 첫 번째 규칙은 입력 수명에 적용되며,
pub fn hs_generic() {
    let result = longest_with_an_announcement("hihiX", "hihiYY", "Ann??");
    println!("{}", result);
}

// 제네릭 유형 매개변수, 특성 경계 및 수명을 함께 사용!
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 클로저 핥아보기
pub fn hs_closures() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

// 이것도 <T>가 있는 구조체를 구현하기 위해서는 앞에 붙여주어야 하나보다!!
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 캐셔 구조체에 익명함수를 넘긴다!
    let mut expensive_result = Cacher::new( |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity)); //
        println!("Next, do {} situps!", expensive_result.value(intensity + 1)); // 이미 위의 intensity 값으로 초기화 되었다. 클로저가 다시 호출되지 않는다.
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

// 그나마 클로저 같다.
pub fn hs_closures1() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
// Iterator 를 Counter 구조체에 구현한다!!
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn hs_index() {
    hs_print();
    hs_variables();
    hs_if();
    hs_loop();
    hs_references_borrowing();
    hs_no_dangle();
    hs_string_slice();
    hs_etc_slice();
    hs_struct();
    hs_struct_fn();
    hs_lifetime();
    hs_generic();
    hs_closures();
    hs_closures1();
}