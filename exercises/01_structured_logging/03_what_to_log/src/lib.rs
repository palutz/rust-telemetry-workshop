mod logger;

pub use logger::TestLogger;

/// Given a list of order numbers, compute the total price.
///
/// # Exercise
///
/// Add log statements to `get_total` and `get_order_details`, our two units of work, to capture
/// the data points we discussed:
/// - the start and end of each unit of work
/// - the duration of each unit of work
/// - the outcome of each unit of work
///
/// Refer to the test files for the expected output format.
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    let instant = std::time::Instant::now();
    log::info!("START - process total price");
    let mut tot : u64 = 0;
    for n in order_numbers {
        tot += get_order_details(*n)?.price;
    }
    log::info!(
        "END - process total price - SUCCESS - {}ms",
        instant.elapsed().as_millis()
    );
    Ok(tot)
}

pub struct OrderDetails {
    pub order_number: u64,
    pub price: u64,
}

/// A dummy function to simulate what would normally be a database query.
fn get_order_details(order_number: u64) -> Result<OrderDetails, anyhow::Error> {
    let instant = std::time::Instant::now();
    log::info!("START - retrieve order");
    if order_number % 4 == 0 {
        log::info!(
            "END - retrieve order - ERROR - {}ms",
            instant.elapsed().as_millis()
        );
        Err(anyhow::anyhow!("Failed to talk to the database"))
    } else {
        let prices = vec![999, 1089, 1029];
        log::info!(
            "END - retrieve order - SUCCESS - {}ms",
            instant.elapsed().as_millis()
        Ok(OrderDetails {
            order_number,
            price: prices[order_number as usize % prices.len()],
        })
    }
}
