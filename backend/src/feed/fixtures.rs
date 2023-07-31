use super::Feed;
use crate::post;

pub fn feed_new() -> Feed {
    let posts = post::fixtures::posts();

    Feed::new("new".to_string(), "New".to_string(), posts)
}

pub fn feed_popular() -> Feed {
    let posts = post::fixtures::posts();

    Feed::new("popular".to_string(), "Popular".to_string(), posts)
}

pub fn feed_0001() -> Feed {
    Feed::new(
        "b411e3b1-55ab-4dd9-884f-2cdeedf52315".to_string(),
        "Great Feed".to_string(),
        vec![],
    )
}
