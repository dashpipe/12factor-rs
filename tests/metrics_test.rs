extern crate twelve_factor;
#[macro_use]
extern crate prometheus;

use twelve_factor::ApplicationBuilder;

#[test]
fn testmetrics(){
    let app = ApplicationBuilder::default().build();
    let test_counter = register_int_counter!(opts!(
        "test_couter",
        "Test counter",
        labels! {"label" => "one", "label_two" => "two", }))
        .unwrap();
    test_counter.inc();
    test_counter.inc();
    std::thread::sleep_ms(6000);
}