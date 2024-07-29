use std::{cell::{Ref, RefCell}, collections::HashMap};

use candid::Principal;
use ic_cdk::caller;

thread_local! {
    static NOTES: RefCell<HashMap<Principal, Vec<String>>> = RefCell::default();
}

#[ic_cdk::query]
fn getNotes(user: Principal) -> Option<Vec<String>> {
    NOTES.with_borrow(|notes| notes.get(&user).cloned())
}

#[ic_cdk::update]
fn addNote(note: String) {
    let user: Principal = caller();

    if user == Principal::anonymous() {
        panic!("Anonymous Principal!")
    }
    
    NOTES.with_borrow_mut(|notes|{
        let mut_notes = notes.get_mut(&user);

        if let Some(notes_vec) = mut_notes{
            notes_vec.push(note)
        }
        else{
            notes.insert(user, vec![note]);
        }
    })
}