use std::error::Error;
use brother_ql::{connection::{PrinterConnection, UsbConnection, UsbConnectionInfo}, media::Media, printjob::PrintJob};
use image::open;

// DK1209
// 62mm x 29mm
fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = UsbConnection::open(UsbConnectionInfo::discover()?.unwrap())?;
    let job = PrintJob::from_image(image::open("label.png")?, Media::D62x29)?;
    conn.print(job)?;
    Ok(())
}
