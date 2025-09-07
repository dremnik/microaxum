pub trait IntoNewRecord {
    type Record;

    fn into_new_record(self) -> Self::Record;
}
