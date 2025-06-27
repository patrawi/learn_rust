const ITEM_UNIT_PRICE: u32 = 10;
static mut TOTAL_STOCK: u32 = 0;
fn add_stock(quantity: u32) {
    unsafe {
        TOTAL_STOCK += quantity;
    }
}

fn calculate_total_value(quantity: u32) -> f64 {
    f64::from(quantity * ITEM_UNIT_PRICE)
}

fn main() {
    unsafe {
        TOTAL_STOCK = 100;
        add_stock(50);
        assert!(
            TOTAL_STOCK == 150u32,
            "TOTAL_STOCK should be 150 after adding stock."
        );
        let total_value = calculate_total_value(TOTAL_STOCK);
        let result = format!("{}", total_value);
        println!("Total Value of 150 items: {}", result);
    }
}
