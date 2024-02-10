use pbc_zk::*;

#[zk_compute(shortname = 0x61)]
pub fn run_auction() -> (Sbi32, Sbi32) {
    // Initialize state
    let mut lowest_bidder: Sbi32 = Sbi32::from(0);
    let mut lowest_amount: Sbi32 = Sbi32::from(2147483646); // Initialize with the maximum possible value

    // Determine min
    for variable_id in secret_variable_ids() {
        if load_sbi::<Sbi32>(variable_id) < lowest_amount {
            lowest_amount = load_sbi::<Sbi32>(variable_id);
            lowest_bidder = Sbi32::from(load_metadata::<i32>(variable_id));
        }
    }

    // Return lowest bidder index and lowest amount
    (lowest_bidder, lowest_amount)
}
