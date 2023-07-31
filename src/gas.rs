use megalodon::Megalodon;
use megalodon::pleroma::pleroma::Pleroma;
use crate::etherscan::etherscan::Etherscan;

pub async fn track_gas(
	pleroma: Pleroma,
	etherscan: Etherscan,
) -> Result<(), Box<dyn std::error::Error>> {
	// Vec keeping track of the last 50 measurements
	let mut gas_vec: Vec<u32> = Vec::new();
	let mut gas_price_sum: u32 = 0;

	loop {
		let results = etherscan.clone().get_gas().await?;
		// Get how many measurements we have in the vec, and if 50, remove the oldest one
		if gas_vec.len() == 50 {
			gas_vec.remove(0);
		}
		// Push the new measurement
		let fast_gas_price = results.fast_gas_price.parse::<u32>().unwrap();
		gas_vec.push(fast_gas_price);
		gas_price_sum = gas_price_sum + fast_gas_price;
		// Get the average
		let gas_price_average = gas_vec.iter().sum::<u32>() as f32 / gas_vec.len() as f32;

		// Get all the stats we're going to use
		let last_block = results.last_block.parse::<u32>().unwrap();
		let safe_gas_price = results.safe_gas_price.parse::<u32>().unwrap();
		let recommended_gas_price = results.propose_gas_price.parse::<u32>().unwrap();

		// Send the toot
		pleroma.post_status(format!("Stats for block {}:\n\nSafe gas price: {} gwei\nRecommended gas price: {} gwei\nFast gas price: {} gwei\nAverage fast over the last 50 blocks: {}\n", last_block, safe_gas_price, recommended_gas_price, fast_gas_price, gas_price_average), None).await?;
		// Wait for 10 mins
		std::thread::sleep(std::time::Duration::from_secs(600));
	}
}