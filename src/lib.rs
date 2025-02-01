use serde::Serialize;

/// `Greeting` 구조체 정의
#[derive(Serialize, Debug, PartialEq)]
pub struct Greeting {
    pub message: String,
}

/// `greet` 함수는 주어진 이름을 받아 "Hello, {이름}!" 형태의 메시지를 반환합니다.
///
/// # 예제
///
/// ```
/// use my_rust_app::{greet, Greeting};
///
/// let result = greet("Alice");
/// assert_eq!(result, Greeting { message: String::from("Hello, Alice!") });  // 수정된 부분
/// ```
pub fn greet(name: &str) -> Greeting {
    Greeting {
        message: format!("Hello, {}!", name),
    }
}

pub async fn root_handler() -> &'static str {
    "Hello, Rust."
}