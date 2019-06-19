use crate::data::BaseData;

pub(crate) struct BaseDataCollection<T>
where
    T: BaseData 
{
    values: Vec<T>
}