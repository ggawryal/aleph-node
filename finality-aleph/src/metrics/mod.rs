mod all_block;
mod chain_state;
mod timing;
pub mod transaction_pool;

pub use all_block::AllBlockMetrics;
pub use chain_state::run_chain_state_metrics;
use substrate_prometheus_endpoint::{exponential_buckets, prometheus};
pub use timing::{Checkpoint, DefaultClock, TimingBlockMetrics};
pub use transaction_pool::TransactionPoolInfoProvider;

const LOG_TARGET: &str = "aleph-metrics";

/// Create `count_below` + 1 + `count_above` buckets, where (`count_below` + 1)th bucket
/// has an upper bound `start`. The buckets are exponentially distributed with a factor `factor`.
pub fn exponential_buckets_two_sided(
    start: f64,
    factor: f64,
    count_below: usize,
    count_above: usize,
) -> prometheus::Result<Vec<f64>> {
    let mut strictly_smaller =
        exponential_buckets(start / factor.powi(count_below as i32), factor, count_below)?;
    let mut greater_than_or_equal = exponential_buckets(start, factor, 1 + count_above)?;
    if strictly_smaller.last().is_some()
        && strictly_smaller.last().unwrap()
            >= greater_than_or_equal
                .first()
                .expect("There is at least one checkpoint")
    {
        return Err(prometheus::Error::Msg(
            "Floating point arithmetic error causing incorrect buckets, try larger factor or smaller count_below"
                .to_string(),
        ));
    }
    strictly_smaller.append(&mut greater_than_or_equal);
    Ok(strictly_smaller)
}
