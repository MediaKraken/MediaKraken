use mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;
use parse_torrent_title::{ParsedTitle, Value};
use serde_json::json;
use std::error::Error;
use std::path::Path;

use crate::base::metadata_debug_log;

pub async fn metadata_guessit(
    sqlx_pool: &sqlx::PgPool,
    download_data: &DBDownloadQueueByProviderList,
    mut metadata_last_title: String,
    mut metadata_last_year: i32,
    mut metadata_last_uuid: uuid::Uuid,
) -> Result<ParsedTitle, Box<dyn Error>> {
    let mut metadata_uuid: uuid::Uuid = uuid::Uuid::nil();
    // check for dupes by name/year
    let download_path = download_data
        .mm_download_path
        .as_ref()
        .ok_or("mm_download_path missing from download queue record")?;
    let file_name = Path::new(download_path)
        .file_name()
        .ok_or("download path has no file component")?
        .to_str()
        .ok_or("download path is not valid UTF-8")?
        .to_string();
    // parse() never fails; a name with no parseable title yields an empty
    // title(), which is handled by the else branch below like the old
    // torrent_name_parser error path did.
    let guessit_data: ParsedTitle = parse_torrent_title::parse(&file_name);
    if !guessit_data.title().is_empty() {
        metadata_debug_log(json!({
            "Type": "Guessit",
            "Module": std::module_path!(),
            "DL Guid": download_data.mm_download_guid,
            "FileName": file_name,
            "Title": guessit_data.title(),
            "Year": parsed_year(&guessit_data),
        }))
        .await;
        if parsed_year(&guessit_data).is_some() {
            if guessit_data.title().to_lowercase() == metadata_last_title
                && parsed_year(&guessit_data).ok_or("year is_some but not a value")?
                    == metadata_last_year
            {
                // matches last media scanned, so set with that metadata id
                metadata_uuid = metadata_last_uuid;
            }
        } else if guessit_data.title().to_lowercase() == metadata_last_title {
            // matches last media scanned, so set with that metadata id
            metadata_uuid = metadata_last_uuid;
        }
        // allow none to be set so unmatched stuff can work for skipping
        metadata_last_uuid = metadata_uuid;
        metadata_last_title = guessit_data.title().to_lowercase();
        if parsed_year(&guessit_data).is_some() {
            metadata_last_year =
                parsed_year(&guessit_data).ok_or("year is_some but not a value")?;
        } else {
            metadata_last_year = 0;
        }
    } else {
        metadata_debug_log(json!({
            "Type": "GuessitEmpty",
            "Module": std::module_path!(),
            "DL Guid": download_data.mm_download_guid,
            "FileName": file_name,
            "Note": "no parseable title; row dead-lettered to ZZ",
        }))
        .await;
        mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_update_provider(
            sqlx_pool,
            "ZZ".to_string(),
            download_data.mm_download_guid,
        )
        .await?;
    }
    //Ok((metadata_uuid, guessit_data))
    Ok(guessit_data)
}

/// The single year of a parsed title, if any.
///
/// `ParsedTitle::year` may be a year range (e.g. "2010-2012"); only a plain
/// integer year is returned, mirroring the old `torrent_name_parser`
/// `year() -> Option<i32>` behavior.
pub(crate) fn parsed_year(parsed: &ParsedTitle) -> Option<i32> {
    parsed
        .year()
        .and_then(Value::as_int)
        .map(|year| year as i32)
}

// Cross-platform by construction: treats both '/' and '\\' as separators and
// reports no filename for directory paths (trailing separator) or empty input.
fn extract_filename(path: &str) -> Option<String> {
    if path.is_empty() || path.ends_with('/') || path.ends_with('\\') {
        return None;
    }
    path.rsplit(['/', '\\']).next().map(str::to_string)
}

fn compare_titles_and_years(
    title1: &str,
    year1: Option<i32>,
    title2: &str,
    year2: Option<i32>,
) -> bool {
    let title_match = title1.to_lowercase() == title2.to_lowercase();
    match (year1, year2) {
        (Some(y1), Some(y2)) => title_match && y1 == y2,
        _ => title_match,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_filename_from_full_path() {
        assert_eq!(
            extract_filename("/path/to/file.mp4"),
            Some("file.mp4".to_string())
        );
    }

    #[test]
    fn test_extract_filename_from_bare_name() {
        assert_eq!(extract_filename("file.mp4"), Some("file.mp4".to_string()));
    }

    #[test]
    fn test_extract_filename_from_path_with_trailing_slash() {
        assert_eq!(extract_filename("/path/to/"), None);
    }

    #[test]
    fn test_extract_filename_from_empty_path() {
        assert_eq!(extract_filename(""), None);
    }

    #[test]
    fn test_extract_filename_from_unix_path() {
        assert_eq!(
            extract_filename("Movies/Matrix, The (1999)/matrix.mp4"),
            Some("matrix.mp4".to_string())
        );
    }

    #[test]
    fn test_extract_filename_from_windows_path() {
        assert_eq!(
            extract_filename(r"C:\Movies\Matrix\matrix.mp4"),
            Some("matrix.mp4".to_string())
        );
    }

    #[test]
    fn test_compare_titles_match_same_case() {
        assert!(compare_titles_and_years(
            "The Matrix",
            None,
            "The Matrix",
            None
        ));
    }

    #[test]
    fn test_compare_titles_match_different_case() {
        assert!(compare_titles_and_years(
            "the matrix",
            None,
            "THE MATRIX",
            None
        ));
    }

    #[test]
    fn test_compare_titles_match_with_year() {
        assert!(compare_titles_and_years(
            "The Matrix",
            Some(1999),
            "the matrix",
            Some(1999)
        ));
    }

    #[test]
    fn test_compare_titles_mismatch_year() {
        assert!(!compare_titles_and_years(
            "The Matrix",
            Some(1999),
            "the matrix",
            Some(1998)
        ));
    }

    #[test]
    fn test_compare_titles_mismatch_title() {
        assert!(!compare_titles_and_years(
            "The Matrix",
            None,
            "Star Wars",
            None
        ));
    }

    #[test]
    fn test_compare_titles_one_year_some_other_none() {
        assert!(compare_titles_and_years(
            "The Matrix",
            Some(1999),
            "the matrix",
            None
        ));
    }

    #[test]
    fn test_compare_titles_both_year_none() {
        assert!(compare_titles_and_years(
            "The Matrix",
            None,
            "the matrix",
            None
        ));
    }

    #[test]
    fn test_parse_torrent_title_basic() {
        let metadata = parse_torrent_title::parse("The.Matrix.1999.1080pBluRay.x264");
        assert_eq!(metadata.title(), "The Matrix");
    }

    #[test]
    fn test_parse_torrent_title_with_year() {
        let metadata = parse_torrent_title::parse("Movie.Name.2020.720p.Webrip");
        assert_eq!(metadata.title(), "Movie Name");
        assert_eq!(parsed_year(&metadata), Some(2020));
    }

    #[test]
    fn test_parse_torrent_title_spongebob() {
        let metadata = parse_torrent_title::parse("SpongeBob.SquarePants.S01E01");
        assert_eq!(metadata.title(), "SpongeBob SquarePants");
    }

    #[test]
    fn test_parse_torrent_title_invalid() {
        // parse() never fails; an unparseable name yields an empty title
        let metadata = parse_torrent_title::parse("");
        assert!(metadata.title().is_empty());
    }
}
