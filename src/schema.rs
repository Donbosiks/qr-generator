diesel::table! {
    qrcode (id) {
        id -> Int4,
        indeficator -> Varchar,
        link -> Text,
    }
}