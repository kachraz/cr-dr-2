use ytrusttest3::add;

mod common;

#[test]
fn integration_it_works() {
    common::setup();
    let result = add(2, 2);
    assert_eq!(result, 4);
}
