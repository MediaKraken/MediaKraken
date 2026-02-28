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

/*
    generate a 62x29mm label with the following checkboxes: MK, UHD, Bray, DVD, Disc, Box, Case, Ripd, UPC, Seen, Fav, Good, Bad, Trash
    no headers and nothing checked
    need it for a ql800 printer
    without the rating, date and botton upc line
 
Die-cut labels, WxH 62x29mm (696x318px)
 */