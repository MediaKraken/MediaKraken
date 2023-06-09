// https://isbndb.com/apidocs/v2

use mk_lib_network;

const ISBN_API_URL: &str = "http://isbndb.com/api/v2/json/";

pub async fn mk_provider_isbndb_author_detail_by_name(
    api_key: String,
    lookup_name: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}/author/{}",
        ISBN_API_URL,
        api_key,
        lookup_name.replace(" ", "_")
    );
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(url)
        .await
        .unwrap();
    Ok(json_data)
}

pub async fn mk_provider_isbndb_author_search_by_name(
    api_key: String,
    lookup_name: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}/authors/{}",
        ISBN_API_URL,
        api_key,
        lookup_name.replace(" ", "_")
    );
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(url)
        .await
        .unwrap();
    Ok(json_data)
}

pub async fn mk_provider_isbndb_book_search_by_name(
    api_key: String,
    lookup_isbn: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}/books/{}",
        ISBN_API_URL,
        api_key,
        lookup_isbn.replace("-", "")
    );
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(url)
        .await
        .unwrap();
    Ok(json_data)
}

pub async fn mk_provider_isbndb_book_detail_by_isbn(
    api_key: String,
    lookup_name: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}/book/{}",
        ISBN_API_URL,
        api_key,
        lookup_name.replace(" ", "_")
    );
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(url)
        .await
        .unwrap();
    Ok(json_data)
}

pub async fn mk_provider_isbndb_publisher_detail_by_name(
    api_key: String,
    lookup_name: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}/publisher/{}",
        ISBN_API_URL,
        api_key,
        lookup_name.replace(" ", "_")
    );
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(url)
        .await
        .unwrap();
    Ok(json_data)
}

pub async fn mk_provider_isbndb_publisher_search_by_name(
    api_key: String,
    lookup_name: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}/publishers/{}",
        ISBN_API_URL,
        api_key,
        lookup_name.replace(" ", "_")
    );
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(url)
        .await
        .unwrap();
    Ok(json_data)
}
