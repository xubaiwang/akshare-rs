use arrow::{array::RecordBatch, util::pretty::pretty_format_batches};
use try_fetchpypi::AKShare;

fn main() -> anyhow::Result<()> {
    let ak = AKShare::new()?;

    let df = ak.stock_szse_summary().preserve_index(true).call()?;

    print_record_batch(&df);

    Ok(())
}

fn print_record_batch(record: &RecordBatch) {
    println!(
        "{}",
        pretty_format_batches(std::slice::from_ref(record)).expect("Fail to format record batch")
    );
}
