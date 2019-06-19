use std::collections::HashMap;

pub(crate) trait MapFileProvider {
    
}

pub(crate) struct MapFileResolver<'a, 'b> {
    map_files: HashMap<&'a str, MapFile<'b>>,
    by_symbol: HashMap<&'a str, HashMap<u128, &'b MapFileRowEntry>>,
}

pub(crate) struct MapFile<'a> {
    permtick: &'a str,
    delisting_date: u128,
    first_date: u128,
    first_ticker: u128,
}

pub(crate) struct MapFileRowEntry {
    
}