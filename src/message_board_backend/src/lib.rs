use ic_cdk::{export_candid, query, update, storage};
use std::cell::RefCell;

thread_local! {
    static MESSAGES: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[query]
fn greet(name: String) -> String { 
    format!("hello {}!", name)
}

#[update]
fn add_message(message: String) {
    MESSAGES.with(|messages| {
        messages.borrow_mut().push(message);
        storage::stable_save((messages.borrow().clone(),)).expect("Failed to save messages to stable storage");
    });
}

#[query]
fn get_messages() -> Vec<String> {
    MESSAGES.with(|messages| messages.borrow().clone())
}

#[update]
fn modify_message(index: usize, new_message: String) -> Result<(), &'static str> {
    MESSAGES.with(|messages| {
        let mut messages = messages.borrow_mut();
        if index < messages.len() {
            messages[index] = new_message;
            storage::stable_save((messages.clone(),)).expect("Failed to save messages to stable storage");
            Ok(())
        } else {
            Err("Message not found")
        }
    })
}

#[update]
fn delete_message(index: usize) -> Result<(), &'static str> {
    MESSAGES.with(|messages| {
        let mut messages = messages.borrow_mut();
        if index < messages.len() {
            messages.remove(index);
            storage::stable_save((messages.clone(),)).expect("Failed to save messages to stable storage");
            Ok(())
        } else {
            Err("Message not found")
        }
    })
}

export_candid!();
