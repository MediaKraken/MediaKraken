// https://docs.google.com/spreadsheets/d/1po70GCN9JUwrWgycMueNfxpEvBjLd7DQkiMRUQFFsL8/export?format=tsv&gid=0
// https://my-upc.com/# uses the above URL to get data from google sheets

// upc "master list"
// https://docs.google.com/spreadsheets/d/1IgK7tIEKngP59PUOs_lsF4P1hbSRIG71tgxncpu1Mws/edit?gid=402351217#gid=402351217

use mk_lib_network;

pub async fn provider_google_sheets_fetch(
    sheet_id: String,
    export_type: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let sheet_data = mk_lib_network::mk_lib_network::mk_data_from_url(format!(
        "https://docs.google.com/spreadsheets/d/{}/export?format={}&gid=0",
        sheet_id, export_type
    ))
    .await
    .unwrap();
    Ok(sheet_data)
}
