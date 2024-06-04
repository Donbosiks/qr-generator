diesel::table! {
    qrcode (id) {
        id -> Int4,
        identifier -> Varchar,
        link -> Text,
    }
}