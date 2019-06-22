use std::collections::HashMap;
use crate::data::EpochTime;

pub(crate) trait MapFileProvider {
    
}

pub(crate) struct MapFileResolver<'a, 'b> {
    map_files: HashMap<&'a str, MapFile<'b>>,
    by_symbol: HashMap<&'a str, HashMap<EpochTime, &'b MapFileRowEntry>>,
}

pub(crate) struct MapFile<'a> {
    permtick: &'a str,
    delisting_date: EpochTime,
    first_date_time: EpochTime,
    first_ticker_time: EpochTime,
}

pub(crate) struct MapFileRowEntry {
    
}