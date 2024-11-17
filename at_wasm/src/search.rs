/// This is how we do search.
///
/// 1. Get all of the data from each of Events.
/// 2. Remove html tags.
/// 3. Remove all common words. 
/// 4. Remove punctuation
/// 5. Words into a hashset with a vec of indexes that contain each event.
/// 6. Store the result, no recompute.
/// 7. use lev distance pick the indices of events we want to render.
/// 8. 

use std::collections::{HashMap, HashSet};
use crate::Event;
use crate::CustomStringTrait;

pub type DocumentTokenMap = HashMap<String, HashSet<usize>>;

// This should be pre-computed.
pub fn construct_document_token_map(events: &[Event]) -> DocumentTokenMap {

    let mut rv: DocumentTokenMap = DocumentTokenMap::new();

    for (i_event, event) in events.iter().enumerate() {
        let mut details_string = event.details.as_str().to_string().to_ascii_lowercase();
        details_string = details_string.replace("<h3>", " ")
                                       .replace("</h3>", " ")
                                       .replace("<h2>", " ")
                                       .replace("<p>", " ")
                                       .replace("</p>", " ")
                                       .replace("\n", " ")
                                       .replace("\t", " ")
                                       .replace(".", " ")
                                       .replace(",", " ");

        for it in details_string.split(" ") {

            match rv.get_mut(it) {
                Some(v) => {
                    v.insert(i_event);
                },
                None => {
                    rv.insert(it.to_string(), HashSet::from([i_event]));
                }
            }
        }

        let mut summary_string = event.summary.as_str().to_string().to_ascii_lowercase();
        summary_string = summary_string.replace("<h3>", " ")
                                       .replace("</h3>", " ")
                                       .replace("<h2>", " ")
                                       .replace("<p>", " ")
                                       .replace("</p>", " ")
                                       .replace("\n", " ")
                                       .replace("\t", " ")
                                       .replace(".", " ")
                                       .replace(",", " ");


        for it in summary_string.split(" ") {
            match rv.get_mut(it) {
                Some(v) => {
                    v.insert(i_event);
                },
                None => {
                    rv.insert(it.to_string(), HashSet::from([i_event]));
                }
            }
        }
    }

    for it in ["a", "be", "are", "the",  "for", "in", "an", "that", "with"] {
        rv.remove(it);
    }
    return rv;
}

pub fn query_document_token_map(query: &str, map: &DocumentTokenMap) -> HashSet<usize> {
    // Simple - Replace with l-distance
    let mut words: Vec<&str> = query.split(" ").collect();
    words.dedup();
    // TODO use levenshtein distance instead of a direct compare.
    // let l = levenshtein_distance::levenshtein_dist_word(jt, it) as f32;

    let mut indices = HashSet::new();
    for it in words.iter() {
        for jt in map.keys() {
            if it == jt {
                let l = map.get(jt).unwrap();
                for lt in l.iter() {
                    indices.insert(*lt);
                }
            }
        }
    }
    return indices;
}
