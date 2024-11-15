use voca_injector::VocaInjector;

mod voca_injector;
mod voca_dao;

fn main() {
    let mut injector = VocaInjector::new();
    injector.insert("hand","손");
    injector.insert_tag("debris","잔해","Toeic");

    println!("{:?}",injector.get_voca_eng("hand"));
}
