use anime_finished::AnimeFinishedPageMedia;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use reqwest::blocking::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/test.graphql",
    response_derives = "Debug, Deserialize, Serialize"
)]
struct AnimeFinished;

#[allow(dead_code)]
impl AnimeFinished {
    fn request(per_page: Option<i64>) -> Vec<AnimeFinishedPageMedia> {
        let client = Client::new();
        post_graphql::<Self, _>(
            &client,
            "https://graphql.anilist.co",
            anime_finished::Variables { per_page },
        )
        .unwrap()
        .data
        .expect("missing data")
        .page
        .unwrap()
        .media
        .unwrap()
        .into_iter()
        .filter_map(|opt| opt)
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let len = 20;
        let media = AnimeFinished::request(Some(len));
        assert_eq!(media.len() as i64, len);
    }
}
