#![allow(unused)]

use std::{rc::Rc, borrow::{BorrowMut, Borrow}, cell::{RefCell, Cell}, ops::AddAssign};





slint::include_modules!();
fn main() {
    let counter = Counter::new();

    let mut counter_num = Rc::from(Cell::from(0i32));

    let counter_num_ref = Rc::clone(&counter_num);

    let inc_weak = counter.as_weak();

    let dec_weak = counter.as_weak();

    counter.global::<Label>().set_label(slint::SharedString::from(format!("{}", counter_num.get())));

    counter.on_inc(move || {
        counter_num.set(counter_num.get()+1);
        inc_weak.unwrap().global::<Label>().set_label(slint::SharedString::from(format!("{}", counter_num.get())));
    });

    counter.on_dec(move || {
        counter_num_ref.set(counter_num_ref.get()-1);
        dec_weak.unwrap().global::<Label>().set_label(slint::SharedString::from(format!("{}", counter_num_ref.get())));
    });

    counter.run();
}
