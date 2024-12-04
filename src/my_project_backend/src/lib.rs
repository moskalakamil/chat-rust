use std::cell::RefCell;

thread_local! {
    static MSG: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn set_msg(new_msg: String) {
    MSG.with(|msg| {
        msg.borrow_mut().push(new_msg);
    });
}

#[ic_cdk::query]
fn get_chat() -> Vec<String> {
    MSG.with(|msg| msg.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
