use crate::data::BaseData;

pub(crate) struct BaseDataCollection<B>
where
    B: BaseData 
{
    values: Vec<B>
}