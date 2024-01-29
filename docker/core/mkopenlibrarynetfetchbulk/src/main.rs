use mk_lib_compression;
use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let sqlx_pool = mk_lib_database::mk_lib_database::mk_lib_database_open_pool(1)
        .await
        .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool, false)
        .await
        .unwrap();

    // let (_rabbit_connection, rabbit_channel) =
    //     mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkopenlibrarynetfetchbulk")
    //         .await
    //         .unwrap();

    // let mut rabbit_consumer = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer(
    //     "mkopenlibrarynetfetchbulk",
    //     &rabbit_channel,
    // )
    // .await
    // .unwrap();

    // tokio::spawn(async move {
    //     while let Some(msg) = rabbit_consumer.recv().await {
    //         if let Some(payload) = msg.content {
    /* tab delimited
    type - type of record (/type/edition, /type/work etc.)  0
    key - unique key of the record. (/books/OL1M etc.       1
    revision - revision number of the record                2
    last_modified - last modified timestamp                 3
    JSON - the complete record in JSON format               4
    */

    // authors
    if !Path::new(&"/mediakraken/ol_dump_authors_latest.txt.gz").exists()
        && !Path::new(&"/mediakraken/ol_dump_authors_latest.txt").exists()
    {
        println!("huh1");
        let _fetch_result = mk_lib_network::mk_lib_network::mk_download_file_from_url(
            "https://openlibrary.org/data/ol_dump_authors_latest.txt.gz".to_string(),
            &"/mediakraken/ol_dump_authors_latest.txt.gz".to_string(),
        )
        .await
        .unwrap();
    }
    if !Path::new(&"/mediakraken/ol_dump_authors_latest.txt").exists() {
        println!("huh2");
        mk_lib_compression::mk_lib_compression::mk_decompress_tar_gz_file_gunzip(
            "/mediakraken/ol_dump_authors_latest.txt.gz",
        )
        .await
        .unwrap();
    }
    println!("huh3");
    let result = mk_lib_database::mk_lib_database_copy::mk_lib_database_copy(
        &sqlx_pool,
        "/mediakraken/ol_dump_authors_latest.txt",
    )
    .await;
    println!("result: {:?}", result);
    // let file = File::open("/mediakraken/ol_dump_authors_latest.txt").unwrap();
    // let reader = BufReader::new(file);
    // for line in reader.lines() {
    //     let s = line.unwrap();
    //     let record_info: Vec<&str> = s.split('\t').collect();
    //     let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_openlib::mk_lib_database_metadata_openlib_author_upsert(&sqlx_pool, record_info[1], record_info[4]).await;
    // }

    // editions
    if !Path::new(&"/mediakraken/ol_dump_editions_latest.txt.gz").exists()
        && !Path::new(&"/mediakraken/ol_dump_editions_latest.txt").exists()
    {
        println!("booger1");
        let _fetch_result = mk_lib_network::mk_lib_network::mk_download_file_from_url(
            "https://openlibrary.org/data/ol_dump_editions_latest.txt.gz".to_string(),
            &"/mediakraken/ol_dump_editions_latest.txt.gz".to_string(),
        )
        .await
        .unwrap();
    }
    if !Path::new(&"/mediakraken/ol_dump_editions_latest.txt").exists() {
        println!("booger2");
        mk_lib_compression::mk_lib_compression::mk_decompress_tar_gz_file_gunzip(
            "/mediakraken/ol_dump_editions_latest.txt.gz",
        )
        .await
        .unwrap();
    }
    println!("booger3");
    let file = File::open("/mediakraken/ol_dump_editions_latest.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let s = line.unwrap();
        let record_info: Vec<&str> = s.split('\t').collect();
        let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_openlib::mk_lib_database_metadata_openlib_edition_upsert(&sqlx_pool, record_info[1], record_info[4]).await;
    }

    // ratings
    if !Path::new(&"/mediakraken/ol_dump_ratings_latest.txt.gz").exists()
        && !Path::new(&"/mediakraken/ol_dump_ratings_latest.txt").exists()
    {
        println!("what1");
        let _fetch_result = mk_lib_network::mk_lib_network::mk_download_file_from_url(
            "https://openlibrary.org/data/ol_dump_ratings_latest.txt.gz".to_string(),
            &"/mediakraken/ol_dump_ratings_latest.txt.gz".to_string(),
        )
        .await
        .unwrap();
    }
    if !Path::new(&"/mediakraken/ol_dump_ratings_latest.txt").exists() {
        println!("what2");
        mk_lib_compression::mk_lib_compression::mk_decompress_tar_gz_file_gunzip(
            "/mediakraken/ol_dump_ratings_latest.txt.gz",
        )
        .await
        .unwrap();
    }
    println!("what3");
    let file = File::open("/mediakraken/ol_dump_ratings_latest.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let s = line.unwrap();
        let record_info: Vec<&str> = s.split('\t').collect();
        let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_openlib::mk_lib_database_metadata_openlib_rating_upsert(&sqlx_pool, record_info[1], record_info[4]).await;
    }

    // works
    if !Path::new(&"/mediakraken/ol_dump_works_latest.txt.gz").exists()
        && !Path::new(&"/mediakraken/ol_dump_works_latest.txt").exists()
    {
        println!("works1");
        let _fetch_result = mk_lib_network::mk_lib_network::mk_download_file_from_url(
            "https://openlibrary.org/data/ol_dump_works_latest.txt.gz".to_string(),
            &"/mediakraken/ol_dump_works_latest.txt.gz".to_string(),
        )
        .await
        .unwrap();
    }
    if !Path::new(&"/mediakraken/ol_dump_works_latest.txt").exists() {
        println!("works2");
        mk_lib_compression::mk_lib_compression::mk_decompress_tar_gz_file_gunzip(
            "/mediakraken/ol_dump_works_latest.txt.gz",
        )
        .await
        .unwrap();
    }
    println!("works2");
    let file = File::open("/mediakraken/ol_dump_works_latest.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let s = line.unwrap();
        let record_info: Vec<&str> = s.split('\t').collect();
        let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_openlib::mk_lib_database_metadata_openlib_work_upsert(&sqlx_pool, record_info[1], record_info[4]).await;
    }

    // match record_info[0] {
    //     "/type/about" => {}
    //     "/type/author" => {
    //     }
    //     "/type/backreference" => {}
    //     "/type/collection" => {}
    //     "/type/content" => {}
    //     "/type/edition" => {
    //     }
    //     "/type/delete" => {}
    //     "/type/doc" => {}
    //     "/type/home" => {}
    //     "/type/i18n" => {}
    //     "/type/i18n_page" => {}
    //     "/type/language" => {}
    //     "/type/library" => {}
    //     "/type/list" => {}
    //     "/type/local_id" => {}
    //     "/type/macro" => {}
    //     "/type/object" => {}
    //     "/type/page" => {}
    //     "/type/permission" => {}
    //     "/type/place" => {}
    //     "/type/property" => {}
    //     "/type/rawtext" => {}
    //     "/type/redirect" => {}
    //     "/type/scan_location" => {}
    //     "/type/scan_record" => {}
    //     "/type/series" => {}
    //     "/type/subject" => {}
    //     "/type/template" => {}
    //     "/type/type" => {}
    //     "/type/uri" => {}
    //     "/type/user" => {}
    //     "/type/usergroup" => {}
    //     "/type/volume" => {}
    //     "/type/work" => {
    //     }
    //     _ => println!("Missing Type {}", record_info[0]),
    // }

    //             let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
    //                 &rabbit_channel,
    //                 msg.deliver.unwrap().delivery_tag(),
    //             )
    //             .await;
    //         }
    //     }
    // });

    // let guard = Notify::new();
    // guard.notified().await;
    Ok(())
}
