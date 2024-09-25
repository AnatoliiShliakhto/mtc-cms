use super::*;

pub fn use_init_search_engine() -> Signal<BTreeMap<usize, SearchIdxDto>> {
    use_context_provider(UseSearchEngine::default);
    let list = consume_context::<UseSearchEngine>().list;
    let index = consume_context::<UseSearchEngine>().index;
    let mut result = consume_context::<UseSearchEngine>().result;
    let pattern = consume_context::<UseSearchEngine>().pattern;

    use_effect(move || {
        if pattern().is_empty() {
            result.write().clear();
            return;
        }

        result.write().clear();
        for idx in index().search(&pattern()).iter().take(30) {
            if let Some(item) = list().get(&idx) {
                result.write().insert(item.to_owned());
            }
        }
    });

    list
}

pub fn use_search_engine_pattern() -> Signal<Cow<'static, str>> {
    consume_context::<UseSearchEngine>().pattern
}

pub fn use_search_engine_list() -> Signal<BTreeMap<usize, SearchIdxDto>> {
    consume_context::<UseSearchEngine>().list
}

pub fn use_search_engine_index() -> Signal<simsearch::SimSearch<usize>> {
    consume_context::<UseSearchEngine>().index
}

pub fn use_search_engine() -> Signal<BTreeSet<SearchIdxDto>> {
    consume_context::<UseSearchEngine>().result
}

pub fn use_search_engine_drop() {
    let mut pattern = consume_context::<UseSearchEngine>().pattern;

    if !pattern().is_empty() {
        pattern.set("".into())
    }
}

#[derive(Default, Clone, Copy)]
pub struct UseSearchEngine {
    list: Signal<BTreeMap<usize, SearchIdxDto>>,
    index: Signal<simsearch::SimSearch<usize>>,
    result: Signal<BTreeSet<SearchIdxDto>>,
    pattern: Signal<Cow<'static, str>>,
}