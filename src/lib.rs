#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::cargo_common_metadata,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::redundant_pub_crate,
    clippy::ref_option,
    clippy::return_self_not_must_use,
    clippy::struct_field_names,
    clippy::type_complexity,
    clippy::use_self
)]

pub mod context;
pub mod error;
pub mod ffi;
pub mod model;
mod private;
pub mod query;
pub mod schema;
pub mod store;
pub mod value;

pub use context::{
    NSManagedObject, NSManagedObjectContext, NSManagedObjectContextConcurrencyType,
};
pub use error::{CoreDataError, COREDATA_BRIDGE_ERROR_DOMAIN};
pub use model::NSManagedObjectModel;
pub use query::{NSFetchRequest, NSPredicate, SortDescriptor};
pub use schema::{
    AttributeType, DeleteRule, NSAttributeDescription, NSEntityDescription,
    NSRelationshipDescription,
};
pub use store::{
    store_types, NSPersistentContainer, NSPersistentStoreCoordinator, PersistentStoreOptions,
};
pub use value::Value;

/// Common imports.
pub mod prelude {
    pub use crate::context::{
        NSManagedObject, NSManagedObjectContext, NSManagedObjectContextConcurrencyType,
    };
    pub use crate::error::{CoreDataError, COREDATA_BRIDGE_ERROR_DOMAIN};
    pub use crate::model::NSManagedObjectModel;
    pub use crate::query::{NSFetchRequest, NSPredicate, SortDescriptor};
    pub use crate::schema::{
        AttributeType, DeleteRule, NSAttributeDescription, NSEntityDescription,
        NSRelationshipDescription,
    };
    pub use crate::store::{
        store_types, NSPersistentContainer, NSPersistentStoreCoordinator, PersistentStoreOptions,
    };
    pub use crate::value::Value;
}
