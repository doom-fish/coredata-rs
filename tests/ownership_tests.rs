#![allow(clippy::wildcard_imports)]

mod support;

use std::collections::BTreeMap;
use std::sync::Arc;
use std::thread;
use std::time::UNIX_EPOCH;

use coredata::bridge_error_codes::{INVALID_ARGUMENT, OBJC_EXCEPTION};
use coredata::prelude::*;
use coredata::COREDATA_BRIDGE_ERROR_DOMAIN;
use support::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const fn assert_send<T: Send>() {}
const fn assert_send_sync<T: Send + Sync>() {}

trait AmbiguousIfSync<A> {
    fn check() {}
}
impl<T: ?Sized> AmbiguousIfSync<()> for T {}
impl<T: ?Sized + Sync> AmbiguousIfSync<u8> for T {}

trait AmbiguousIfSend<A> {
    fn check() {}
}
impl<T: ?Sized> AmbiguousIfSend<()> for T {}
impl<T: ?Sized + Send> AmbiguousIfSend<u8> for T {}

trait AmbiguousIfClone<A> {
    fn check() {}
}
impl<T: ?Sized> AmbiguousIfClone<()> for T {}
impl<T: Clone> AmbiguousIfClone<u8> for T {}

macro_rules! assert_owned_config {
    ($($name:ty),+ $(,)?) => {
        $(
            assert_send::<$name>();
            <$name as AmbiguousIfSync<_>>::check();
        )+
    };
}

macro_rules! assert_not_clone {
    ($($name:ty),+ $(,)?) => {
        $(<$name as AmbiguousIfClone<_>>::check();)+
    };
}

fn bridge_code<T: std::fmt::Debug>(result: Result<T, CoreDataError>) -> i64 {
    let error = result.expect_err("expected a bridge error");
    assert_eq!(error.domain, COREDATA_BRIDGE_ERROR_DOMAIN, "{error}");
    error.code
}

#[test]
fn configuration_wrappers_are_send_but_not_sync() {
    assert_owned_config!(
        NSManagedObjectModel,
        NSEntityDescription,
        NSAttributeDescription,
        NSRelationshipDescription,
        NSPropertyDescription,
        NSFetchedPropertyDescription,
        NSExpressionDescription,
        NSDerivedAttributeDescription,
        NSCompositeAttributeDescription,
        NSFetchIndexDescription,
        NSFetchIndexElementDescription,
        NSFetchRequest,
        NSAsynchronousFetchRequest,
        NSSaveChangesRequest,
        NSBatchDeleteRequest,
        NSBatchInsertRequest,
        NSBatchUpdateRequest,
        NSPersistentHistoryChangeRequest,
        NSPersistentCloudKitContainerEventRequest,
        NSPersistentStoreDescription,
        NSPersistentCloudKitContainerOptions,
        NSPersistentContainer,
        NSPersistentCloudKitContainer,
        NSMappingModel,
        NSMigrationManager,
        NSEntityMapping,
        NSPropertyMapping,
        NSManagedObjectModelReference,
        NSMigrationStage,
        NSLightweightMigrationStage,
        NSCustomMigrationStage,
        NSStagedMigrationManager,
        NSAtomicStoreCacheNode,
        NSIncrementalStoreNode,
    );
    assert_send_sync::<NSPersistentStoreCoordinator>();
    assert_send_sync::<NSPersistentStore>();
}

#[test]
fn borrowed_views_cannot_leave_their_thread() {
    <Borrowed<'static, NSEntityDescription> as AmbiguousIfSend<_>>::check();
    <Borrowed<'static, NSEntityDescription> as AmbiguousIfSync<_>>::check();
    <Borrowed<'static, NSManagedObjectModel> as AmbiguousIfSend<_>>::check();
    <Borrowed<'static, NSAttributeDescription> as AmbiguousIfSend<_>>::check();
}

#[test]
fn types_without_a_faithful_copy_are_not_clone() {
    assert_not_clone!(
        NSPersistentContainer,
        NSPersistentCloudKitContainer,
        NSPersistentCloudKitContainerOptions,
        NSMappingModel,
        NSMigrationManager,
        NSEntityMapping,
        NSPropertyMapping,
        NSManagedObjectModelReference,
        NSMigrationStage,
        NSLightweightMigrationStage,
        NSCustomMigrationStage,
        NSStagedMigrationManager,
        NSPersistentStoreRequest,
        NSAtomicStoreCacheNode,
        NSIncrementalStoreNode,
    );
}

#[test]
fn cloned_model_graph_objects_are_independent() -> TestResult {
    let fixture = relationship_model()?;

    let mut model = fixture.model.clone();
    model.add_entity(&NSEntityDescription::named("Extra")?)?;
    assert_eq!(model.entities()?.len(), 3);
    assert_eq!(fixture.model.entities()?.len(), 2);

    let mut person = fixture.person.clone();
    person.set_name("Renamed")?;
    assert_eq!(fixture.person.name()?, "Person");
    let relationships = person.relationships()?;
    assert_eq!(relationships.len(), 1);
    assert_eq!(
        relationships[0].destination_entity_name().as_deref(),
        Some("Task")
    );
    assert_eq!(
        relationships[0].inverse_relationship_name().as_deref(),
        Some("owner")
    );

    let mut children = fixture.children.clone();
    children.set_destination_entity_name(Some("Elsewhere"))?;
    children.set_max_count(3)?;
    assert_eq!(
        fixture.children.destination_entity_name().as_deref(),
        Some("Task")
    );
    assert_eq!(fixture.children.max_count(), 0);

    let base = BTreeMap::from([(String::from("k"), String::from("v"))]);
    let mut attribute = NSAttributeDescription::new("flag", AttributeType::Boolean)?;
    attribute.set_optional(false)?;
    let mut attribute_copy = attribute.clone();
    attribute_copy.set_optional(true)?;
    attribute_copy.set_name("other")?;
    assert!(!attribute.is_optional());
    assert_eq!(attribute.name()?, "flag");

    let mut entity = NSEntityDescription::named("Meta")?;
    entity.set_user_info(&base)?;
    let mut entity_copy = entity.clone();
    entity_copy.set_user_info(&BTreeMap::new())?;
    assert_eq!(entity.user_info()?, base);
    Ok(())
}

#[test]
fn cloned_model_views_are_editable_copies() -> TestResult {
    let fixture = relationship_model()?;
    let view = fixture
        .model
        .entity_named("Person")?
        .expect("Person entity");
    let mut copy = view.clone();
    copy.set_name("Copied")?;
    copy.add_attribute(&NSAttributeDescription::new(
        "extra",
        AttributeType::String,
    )?)?;
    assert!(copy.managed_object_model()?.is_none());
    assert_eq!(copy.relationships()?.len(), 1);
    assert!(copy.relationships()?[0].destination_entity()?.is_none());
    assert_eq!(
        copy.relationships()?[0]
            .destination_entity_name()
            .as_deref(),
        Some("Task")
    );
    assert_eq!(view.name()?, "Person");
    assert_eq!(view.attributes()?.len(), 1);
    Ok(())
}

#[test]
fn adding_to_an_entity_or_model_copies_the_argument() -> TestResult {
    let mut model = NSManagedObjectModel::new()?;
    let mut entity = NSEntityDescription::named("Person")?;
    let mut attribute = NSAttributeDescription::new("name", AttributeType::String)?;
    entity.add_attribute(&attribute)?;
    attribute.set_name("renamed")?;
    model.add_entity(&entity)?;
    entity.set_name("Other")?;
    entity.add_attribute(&NSAttributeDescription::new("late", AttributeType::String)?)?;

    let stored = model.entity_named("Person")?.expect("Person entity");
    let attributes = stored.attributes()?;
    assert_eq!(attributes.len(), 1);
    assert_eq!(attributes[0].name()?, "name");
    assert!(model.entity_named("Other")?.is_none());
    Ok(())
}

#[test]
fn relationships_are_linked_by_name_in_either_order() -> TestResult {
    for task_first in [false, true] {
        let mut person = NSEntityDescription::named("Person")?;
        person.set_managed_object_class_name("NSManagedObject")?;
        let mut task = NSEntityDescription::named("Task")?;
        task.set_managed_object_class_name("NSManagedObject")?;
        let mut tasks = NSRelationshipDescription::new("tasks")?;
        tasks.set_destination_entity_name(Some("Task"))?;
        tasks.set_inverse_relationship_name(Some("owner"))?;
        tasks.set_max_count(0)?;
        tasks.set_optional(true)?;
        let mut owner = NSRelationshipDescription::new("owner")?;
        owner.set_destination_entity_name(Some("Person"))?;
        owner.set_inverse_relationship_name(Some("tasks"))?;
        owner.set_max_count(1)?;
        owner.set_optional(true)?;
        person.add_relationship(&tasks)?;
        task.add_relationship(&owner)?;
        drop((tasks, owner));

        let mut model = NSManagedObjectModel::new()?;
        if task_first {
            model.add_entity(&task)?;
            model.add_entity(&person)?;
        } else {
            model.add_entity(&person)?;
            model.add_entity(&task)?;
        }
        drop((person, task));

        let stored = model.entity_named("Person")?.expect("Person entity");
        let relationships = stored.relationships()?;
        let link = &relationships[0];
        let destination = link.destination_entity()?.expect("destination entity");
        assert_eq!(destination.name()?, "Task");
        let inverse = link.inverse_relationship()?.expect("inverse relationship");
        assert_eq!(inverse.name()?, "owner");
        assert_eq!(
            inverse
                .destination_entity()?
                .expect("inverse destination")
                .name()?,
            "Person"
        );

        let container = in_memory_container("LinkedByName", &model)?;
        let context = container.new_background_context()?;
        let owner_object = NSEntityDescription::insert_new_object_for_name("Person", &context)?;
        let task_object = NSEntityDescription::insert_new_object_for_name("Task", &context)?;
        context.save()?;
        assert!(owner_object
            .object_ids_for_relationship_named("tasks")?
            .is_empty());
        assert!(task_object
            .object_ids_for_relationship_named("owner")?
            .is_empty());
    }
    Ok(())
}

#[test]
fn unresolved_destination_names_are_rejected_at_handoff() -> TestResult {
    let mut person = NSEntityDescription::named("Person")?;
    let mut link = NSRelationshipDescription::new("missing")?;
    link.set_destination_entity_name(Some("Nowhere"))?;
    person.add_relationship(&link)?;
    let mut model = NSManagedObjectModel::new()?;
    model.add_entity(&person)?;
    assert_eq!(
        bridge_code(NSPersistentContainer::new("Unresolved", &model)),
        INVALID_ARGUMENT
    );
    assert_eq!(
        bridge_code(NSPersistentStoreCoordinator::new(&model)),
        INVALID_ARGUMENT
    );
    Ok(())
}

#[test]
fn a_container_keeps_its_own_model_and_descriptions() -> TestResult {
    let mut fixture = basic_model()?;
    let mut container = NSPersistentContainer::new("OwnCopies", &fixture.model)?;

    let mut renamed = fixture.person.clone();
    renamed.set_name("Renamed")?;
    fixture.model.add_entity(&renamed)?;
    assert_eq!(fixture.model.entities()?.len(), 2);
    let container_model = container.managed_object_model()?;
    assert_eq!(container_model.entities()?.len(), 1);
    assert!(container_model.entity_named("Renamed")?.is_none());
    drop(container_model);

    let mut description = NSPersistentStoreDescription::new()?;
    description.set_store_type(store_types::IN_MEMORY)?;
    description.set_should_add_asynchronously(false);
    description.set_timeout(3.0)?;
    container.set_persistent_store_descriptions(&[&description])?;
    description.set_timeout(9.0)?;
    description.set_store_type(store_types::SQLITE)?;

    let mut copies = container.persistent_store_descriptions()?;
    assert!((copies[0].timeout() - 3.0).abs() < f64::EPSILON);
    assert_eq!(copies[0].store_type()?, store_types::IN_MEMORY);
    copies[0].set_timeout(1.0)?;
    let again = container.persistent_store_descriptions()?;
    assert!((again[0].timeout() - 3.0).abs() < f64::EPSILON);

    container.load_persistent_stores()?;
    let context = container.new_background_context()?;
    let object = NSManagedObject::new(&fixture.person, &context)?;
    object.set_value("name", "Ada")?;
    object.set_value("age", 36_i32)?;
    context.save()?;
    let entity = object.entity()?;
    assert_eq!(entity.name()?, "Person");
    assert!(entity.managed_object_model()?.is_some());
    Ok(())
}

#[test]
fn cloned_store_descriptions_are_independent() -> TestResult {
    let mut description = NSPersistentStoreDescription::new()?;
    description.set_store_type(store_types::SQLITE)?;
    description.set_sqlite_pragma("journal_mode", Some("WAL"))?;
    let mut options = NSPersistentCloudKitContainerOptions::new("iCloud.com.example.coredata-rs")?;
    options.set_database_scope(CloudKitDatabaseScope::Private)?;
    description.set_cloudkit_container_options(Some(options))?;

    let mut copy = description.clone();
    copy.set_sqlite_pragma("journal_mode", Some("DELETE"))?;
    copy.set_store_type(store_types::IN_MEMORY)?;
    assert_eq!(
        description
            .sqlite_pragmas()?
            .get("journal_mode")
            .map(String::as_str),
        Some("WAL")
    );
    assert_eq!(description.store_type()?, store_types::SQLITE);
    let shared = copy
        .cloudkit_container_options()?
        .expect("copied CloudKit options");
    assert!(matches!(
        shared.database_scope(),
        CloudKitDatabaseScope::Private
    ));
    Ok(())
}

#[test]
fn cloned_requests_are_faithful_and_independent() -> TestResult {
    let mut fetch = NSFetchRequest::new("Person")?;
    fetch.set_fetch_limit(7)?;
    fetch.set_sort_descriptors(&[SortDescriptor::ascending("name")])?;
    let mut fetch_copy = fetch.clone();
    fetch_copy.set_fetch_limit(1)?;
    assert_eq!(fetch.fetch_limit(), 7);
    assert_eq!(fetch_copy.entity_name().as_deref(), Some("Person"));

    let mut asynchronous = NSAsynchronousFetchRequest::new(&fetch)?;
    fetch.set_fetch_limit(2)?;
    asynchronous.set_estimated_result_count(5);
    let asynchronous_copy = asynchronous.clone();
    assert_eq!(asynchronous_copy.estimated_result_count(), 5);
    assert_eq!(asynchronous_copy.fetch_request()?.fetch_limit(), 7);
    asynchronous.set_estimated_result_count(9);
    assert_eq!(asynchronous_copy.estimated_result_count(), 5);

    let mut delete = NSBatchDeleteRequest::from_fetch_request(&fetch)?;
    delete.set_result_type(BatchDeleteRequestResultType::Count)?;
    let delete_copy = delete.clone();
    assert!(matches!(
        delete_copy.result_type(),
        BatchDeleteRequestResultType::Count
    ));
    assert_eq!(
        delete_copy.fetch_request()?.entity_name().as_deref(),
        Some("Person")
    );

    let mut insert = NSBatchInsertRequest::new("Person", &[person_row("Ada", 36)])?;
    insert.set_result_type(BatchInsertRequestResultType::Count)?;
    let insert_copy = insert.clone();
    assert_eq!(insert_copy.entity_name()?, "Person");
    assert!(matches!(
        insert_copy.result_type(),
        BatchInsertRequestResultType::Count
    ));

    let mut update = NSBatchUpdateRequest::new("Person")?;
    update.set_result_type(BatchUpdateRequestResultType::Count)?;
    update.set_includes_subentities(false);
    let update_copy = update.clone();
    assert_eq!(update_copy.entity_name()?, "Person");
    assert!(!update_copy.includes_subentities());
    assert!(matches!(
        update_copy.result_type(),
        BatchUpdateRequestResultType::Count
    ));

    let mut history = NSPersistentHistoryChangeRequest::fetch_history_after_date(UNIX_EPOCH)?;
    history.set_result_type(PersistentHistoryResultType::Count)?;
    let history_copy = history.clone();
    history.set_result_type(PersistentHistoryResultType::ObjectIds)?;
    assert!(matches!(
        history_copy.result_type(),
        PersistentHistoryResultType::Count
    ));

    let mut events =
        NSPersistentCloudKitContainerEventRequest::fetch_events_after_date(UNIX_EPOCH)?;
    events.set_result_type(NSPersistentCloudKitContainerEventResultType::CountEvents)?;
    let events_copy = events.clone();
    assert!(matches!(
        events_copy.result_type(),
        NSPersistentCloudKitContainerEventResultType::CountEvents
    ));
    Ok(())
}

#[test]
fn cloned_save_changes_requests_keep_their_objects() -> TestResult {
    let fixture = basic_model()?;
    let (_coordinator, context) = in_memory_context(&fixture.model)?;
    let object = insert_person(&context, &fixture.person, "Ada", 36)?;
    let request = NSSaveChangesRequest::new(&[&object], &[], &[], &[])?;
    let copy = request.clone();
    assert_eq!(copy.inserted_objects()?.len(), 1);
    assert_eq!(
        copy.inserted_objects()?[0].value("name")?.as_str(),
        Some("Ada")
    );
    assert_eq!(request.inserted_objects()?.len(), 1);
    Ok(())
}

#[test]
fn fetched_properties_copy_without_touching_name_based_requests() -> TestResult {
    let mut property = NSFetchedPropertyDescription::new()?;
    let request = NSFetchRequest::new("Person")?;
    assert_eq!(
        bridge_code(property.set_fetch_request(Some(&request))),
        OBJC_EXCEPTION
    );
    assert!(property.fetch_request()?.is_none());
    let copy = property.clone();
    assert!(copy.fetch_request()?.is_none());
    assert_eq!(
        copy.as_property_description().is_optional(),
        property.as_property_description().is_optional()
    );
    Ok(())
}

#[test]
fn cloned_fetch_index_descriptions_are_independent() -> TestResult {
    let property = NSAttributeDescription::new("name", AttributeType::String)?;
    let mut element =
        NSFetchIndexElementDescription::new(&property, NSFetchIndexElementType::Binary)?;
    element.set_ascending(true)?;
    let mut index = NSFetchIndexDescription::new("byName", &[&element])?;
    element.set_ascending(false)?;
    assert!(index.elements()?[0].is_ascending());

    let mut copy = index.clone();
    copy.set_name("renamed")?;
    index.set_elements(&[])?;
    assert_eq!(copy.name()?, "renamed");
    assert_eq!(copy.elements()?.len(), 1);
    assert_eq!(copy.elements()?[0].property_name().as_deref(), Some("name"));
    Ok(())
}

#[test]
fn requests_executed_against_a_dropped_store_can_still_be_copied_and_reused() -> TestResult {
    let fixture = basic_model()?;
    let mut request = NSFetchRequest::new("Person")?;
    request.set_relationship_key_paths_for_prefetching(&["name"])?;
    let batch = NSBatchDeleteRequest::from_fetch_request(&request)?;
    {
        let (container, artifact) =
            sqlite_container("DroppedStore", &fixture.model, "dropped", false)?;
        let context = container.new_background_context()?;
        insert_person(&context, &fixture.person, "Ada", 36)?;
        context.save()?;
        assert_eq!(context.fetch(&request)?.len(), 1);
        assert_eq!(context.count(&request)?, 1);
        batch.execute(&context)?;
        drop(context);
        drop(container);
        artifact.cleanup();
    }
    let copy = request.clone();
    let batch_copy = batch.clone();
    assert_eq!(
        batch_copy.fetch_request()?.entity_name().as_deref(),
        Some("Person")
    );
    assert_eq!(
        batch.fetch_request()?.entity_name().as_deref(),
        Some("Person")
    );
    let container = in_memory_container("ReusedRequest", &fixture.model)?;
    let context = container.new_background_context()?;
    insert_person(&context, &fixture.person, "Bea", 25)?;
    context.save()?;
    assert_eq!(context.fetch(&copy)?.len(), 1);
    assert_eq!(context.fetch(&request)?.len(), 1);
    assert_eq!(request.execute_object_ids(&context)?.len(), 1);
    Ok(())
}

#[test]
fn object_ids_report_their_entity_name_after_the_store_is_gone() -> TestResult {
    let fixture = basic_model()?;
    let object_id = {
        let container = in_memory_container("DroppedIds", &fixture.model)?;
        let context = container.new_background_context()?;
        let object = insert_person(&context, &fixture.person, "Ada", 36)?;
        context.save()?;
        object.object_id()?
    };
    assert_eq!(object_id.entity_name()?, "Person");
    Ok(())
}

#[test]
fn objects_use_the_entity_from_the_contexts_model() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("ContextEntity", &fixture.model)?;
    let context = container.new_background_context()?;
    let builder = NSEntityDescription::named("Person")?;
    let object = NSManagedObject::new(&builder, &context)?;
    object.set_value("name", "Ada")?;
    object.set_value("age", 36_i32)?;
    context.save()?;
    assert_eq!(context.count(&NSFetchRequest::new("Person")?)?, 1);

    let unknown = NSEntityDescription::named("Unknown")?;
    assert_eq!(
        bridge_code(NSManagedObject::new(&unknown, &context)),
        INVALID_ARGUMENT
    );
    let entity = NSEntityDescription::entity_for_name("Person", &context)?.expect("Person entity");
    assert_eq!(entity.attributes()?.len(), 2);
    Ok(())
}

#[test]
fn coordinator_and_store_handles_can_be_shared_between_threads() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("SharedCoordinator", &fixture.model)?;
    let coordinator = Arc::new(container.persistent_store_coordinator()?);
    let workers = (0..8)
        .map(|index| {
            let coordinator = Arc::clone(&coordinator);
            thread::spawn(move || -> Result<(), CoreDataError> {
                for round in 0..50 {
                    coordinator.set_name(Some(&format!("worker-{index}-{round}")))?;
                    let _ = coordinator.name();
                    for store in coordinator.persistent_stores()? {
                        store.set_identifier(Some(&format!("store-{index}-{round}")))?;
                        let _ = store.identifier()?;
                        let _ = store.is_read_only();
                    }
                    let model = coordinator.managed_object_model()?;
                    assert_eq!(model.entities()?.len(), 1);
                }
                Ok(())
            })
        })
        .collect::<Vec<_>>();
    for worker in workers {
        worker.join().expect("worker panicked")?;
    }
    assert!(coordinator.name().is_some());
    Ok(())
}
