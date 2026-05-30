// https://github.com/jonhoo/rust-ibverbs

pub async fn mk_lib_network_ibverbs_discover() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = ibverbs::devices()
        .map_err(|e| format!("failed to list ibverbs devices: {e}"))?
        .iter()
        .next()
        .ok_or("no rdma device available")?
        .open()
        .map_err(|e| format!("failed to open rdma device: {e}"))?;
    let _ctx = ctx;
    Ok(())
}
