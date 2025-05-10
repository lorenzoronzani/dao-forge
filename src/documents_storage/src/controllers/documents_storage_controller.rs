use common::utils::Date;
use ic_cdk::{api::time, caller};

use crate::{models::Document, services::DocumentService, types::DocumentArgs};

#[ic_cdk::update]
async fn store_document(document_args: DocumentArgs) -> Document {
    DocumentService::save(
        document_args.name,
        document_args.content_type,
        caller(),
        document_args.content,
        Date::nanoseconds_to_milliseconds(time()),
    )
}

#[ic_cdk::query]
async fn get_document(id: u32) -> Document {
    DocumentService::get(id).expect("Document not found")
}
