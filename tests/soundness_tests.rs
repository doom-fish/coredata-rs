#![allow(clippy::wildcard_imports)]

mod support;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use coredata::prelude::*;
use support::*;

use coredata::bridge_error_codes::{INVALIDATED, INVALID_ARGUMENT, OBJC_EXCEPTION, WRONG_THREAD};

type TestResult = Result<(), Box<dyn std::error::Error>>;

const fn assert_send_sync<T: Send + Sync>() {}

fn bridge_code<T: std::fmt::Debug>(result: Result<T, CoreDataError>) -> i64 {
    let error = result.expect_err("expected a Core Data error");
    assert_eq!(error.domain, COREDATA_BRIDGE_ERROR_DOMAIN, "{error}");
    error.code
}

fn person_count(context: &NSManagedObjectContext) -> Result<usize, CoreDataError> {
    context.count(&NSFetchRequest::new("Person")?)
}

#[test]
fn queue_confined_wrappers_are_send_and_sync() {
    assert_send_sync::<NSManagedObjectContext>();
    assert_send_sync::<NSManagedObject>();
    assert_send_sync::<NSFetchedResultsController>();
    assert_send_sync::<NSFetchedResultsSectionInfo>();
    assert_send_sync::<NSManagedObjectID>();
    assert_send_sync::<NSPredicate>();
}

#[test]
fn private_context_serializes_concurrent_use_from_many_threads() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("ConcurrentUse", &fixture.model)?;
    let context = container.new_background_context()?;

    std::thread::scope(|scope| -> Result<(), CoreDataError> {
        let workers = (0..4)
            .map(|thread| {
                let context = &context;
                scope.spawn(move || -> Result<(), CoreDataError> {
                    context.perform_and_wait(move |ctx| -> Result<(), CoreDataError> {
                        for index in 0..25_i32 {
                            let object =
                                NSEntityDescription::insert_new_object_for_name("Person", &ctx)?;
                            object.set_value("name", format!("t{thread}-{index}"))?;
                            object.set_value("age", index)?;
                            assert_eq!(object.value("age")?.as_i64(), Some(i64::from(index)));
                        }
                        ctx.save()
                    })?
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            worker.join().expect("worker thread panicked")?;
        }
        Ok(())
    })?;

    assert_eq!(person_count(&context)?, 100);
    Ok(())
}

#[test]
fn main_queue_context_and_its_objects_refuse_use_off_the_main_thread() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("MainQueueOffMain", &fixture.model)?;
    let view_context = container.view_context()?;

    assert_eq!(bridge_code(view_context.has_changes()), WRONG_THREAD);
    assert_eq!(bridge_code(view_context.save()), WRONG_THREAD);
    assert_eq!(bridge_code(view_context.name()), WRONG_THREAD);
    assert_eq!(
        bridge_code(NSManagedObject::new(&fixture.person, Some(&view_context))),
        WRONG_THREAD
    );
    assert_eq!(
        bridge_code(view_context.fetch(&NSFetchRequest::new("Person")?)),
        WRONG_THREAD
    );

    let unowned = NSManagedObject::new(&fixture.person, None)?;
    assert_eq!(bridge_code(view_context.insert(&unowned)), WRONG_THREAD);
    assert!(unowned.managed_object_context()?.is_none());
    Ok(())
}

#[test]
fn unsupported_concurrency_types_are_rejected() {
    for raw in [0, 3, 7] {
        let result =
            NSManagedObjectContext::new(NSManagedObjectContextConcurrencyType::Unknown(raw));
        assert_eq!(bridge_code(result), INVALID_ARGUMENT);
    }
}

#[test]
fn perform_and_wait_returns_the_closure_value_and_propagates_panics() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("PerformAndWait", &fixture.model)?;
    let context = container.new_background_context()?;

    let count = context.perform_and_wait(|ctx| -> Result<usize, CoreDataError> {
        insert_named(&ctx, "Ada")?;
        ctx.save()?;
        person_count(&ctx)
    })??;
    assert_eq!(count, 1);

    let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = context.perform_and_wait(|_| -> usize { panic!("closure panic") });
    }));
    assert!(panicked.is_err());
    assert_eq!(person_count(&context)?, 1);
    Ok(())
}

#[test]
fn perform_runs_the_closure_on_the_private_queue() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("PerformAsync", &fixture.model)?;
    let context = container.new_background_context()?;
    let (sender, receiver) = std::sync::mpsc::channel();

    context.perform(move |ctx| {
        let outcome = insert_named(&ctx, "Bea")
            .and_then(|_| ctx.save())
            .and_then(|()| person_count(&ctx));
        let _ = sender.send(outcome);
    })?;

    let count = receiver.recv_timeout(Duration::from_secs(10))??;
    assert_eq!(count, 1);
    Ok(())
}

fn insert_named(
    context: &NSManagedObjectContext,
    name: &str,
) -> Result<NSManagedObject, CoreDataError> {
    let object = NSEntityDescription::insert_new_object_for_name("Person", context)?;
    object.set_value("name", name)?;
    object.set_value("age", 30_i32)?;
    Ok(object)
}

#[test]
fn objects_are_invalidated_by_reset_rollback_and_deletion() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("Invalidation", &fixture.model)?;
    let context = container.new_background_context()?;

    let saved = insert_named(&context, "Ada")?;
    context.save()?;
    context.reset()?;
    assert_eq!(bridge_code(saved.value("name")), INVALIDATED);
    assert_eq!(bridge_code(saved.set_value("name", "Eve")), INVALIDATED);
    assert_eq!(bridge_code(saved.is_fault()), INVALIDATED);
    assert!(saved.managed_object_context()?.is_none());
    assert!(!saved.object_id()?.is_temporary());
    assert_eq!(bridge_code(context.delete(&saved)), INVALIDATED);

    let fetched = context.fetch(&NSFetchRequest::new("Person")?)?;
    assert_eq!(fetched.len(), 1);
    assert_eq!(fetched[0].value("name")?.as_str(), Some("Ada"));

    let rolled_back = insert_named(&context, "Bea")?;
    context.rollback()?;
    assert_eq!(bridge_code(rolled_back.value("name")), INVALIDATED);

    context.delete(&fetched[0])?;
    assert!(fetched[0].is_deleted()?);
    context.save()?;
    assert_eq!(bridge_code(fetched[0].value("name")), INVALIDATED);
    assert_eq!(person_count(&context)?, 0);
    Ok(())
}

#[test]
fn managed_objects_keep_their_context_alive() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("KeepAlive", &fixture.model)?;
    let object = {
        let context = container.new_background_context()?;
        insert_named(&context, "Ada")?
    };

    let context = object
        .managed_object_context()?
        .expect("object should still be registered with its context");
    assert!(context.has_changes()?);
    assert_eq!(object.value("name")?.as_str(), Some("Ada"));
    context.save()?;
    assert_eq!(person_count(&context)?, 1);
    Ok(())
}

#[test]
fn cross_context_operations_return_errors() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("CrossContext", &fixture.model)?;
    let first = container.new_background_context()?;
    let second = container.new_background_context()?;
    let object = insert_named(&first, "Ada")?;

    assert_eq!(bridge_code(second.insert(&object)), INVALID_ARGUMENT);
    assert_eq!(bridge_code(second.delete(&object)), INVALID_ARGUMENT);
    assert_eq!(
        bridge_code(second.refresh_object(&object, false)),
        INVALID_ARGUMENT
    );
    assert_eq!(
        bridge_code(second.obtain_permanent_ids(&[&object])),
        INVALID_ARGUMENT
    );

    let unowned = NSManagedObject::new(&fixture.person, None)?;
    assert_eq!(bridge_code(first.delete(&unowned)), INVALID_ARGUMENT);
    first.insert(&unowned)?;
    assert_eq!(bridge_code(second.insert(&unowned)), INVALID_ARGUMENT);
    assert!(first.inserted_objects()?.len() >= 2);
    Ok(())
}

#[test]
fn unknown_entities_and_attributes_return_errors() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("UnknownNames", &fixture.model)?;
    let context = container.new_background_context()?;

    let unknown = NSFetchRequest::new("Nope")?;
    assert_eq!(bridge_code(context.fetch(&unknown)), INVALID_ARGUMENT);
    assert_eq!(bridge_code(context.count(&unknown)), INVALID_ARGUMENT);
    assert_eq!(
        bridge_code(unknown.execute_object_ids(&context)),
        INVALID_ARGUMENT
    );
    assert_eq!(
        bridge_code(NSEntityDescription::insert_new_object_for_name(
            "Nope", &context
        )),
        INVALID_ARGUMENT
    );
    assert!(NSEntityDescription::entity_for_name("Nope", &context)?.is_none());

    let object = insert_named(&context, "Ada")?;
    assert_eq!(bridge_code(object.value("bogus")), INVALID_ARGUMENT);
    assert_eq!(
        bridge_code(object.set_value("bogus", 1_i32)),
        INVALID_ARGUMENT
    );
    assert_eq!(
        bridge_code(object.set_value("age", "not a number")),
        OBJC_EXCEPTION
    );
    assert_eq!(
        bridge_code(object.has_fault_for_relationship_named("name")),
        INVALID_ARGUMENT
    );
    assert_eq!(
        bridge_code(object.object_ids_for_relationship_named("bogus")),
        INVALID_ARGUMENT
    );

    let bad_key_path = NSFetchRequest::new("Person")?;
    bad_key_path.set_predicate(Some(&NSPredicate::from_format("bogus == 1", &[])?));
    assert_eq!(bridge_code(context.fetch(&bad_key_path)), OBJC_EXCEPTION);
    Ok(())
}

#[test]
fn saving_without_stores_or_coordinator_returns_errors() -> TestResult {
    let fixture = basic_model()?;
    let coordinator = NSPersistentStoreCoordinator::new(&fixture.model)?;
    let no_stores = NSManagedObjectContext::new_private_queue()?;
    no_stores.set_persistent_store_coordinator(&coordinator)?;
    no_stores.save()?;
    insert_named(&no_stores, "Ada")?;
    assert_eq!(bridge_code(no_stores.save()), INVALID_ARGUMENT);

    let no_coordinator = NSManagedObjectContext::new_private_queue()?;
    assert_eq!(
        bridge_code(NSEntityDescription::insert_new_object_for_name(
            "Person",
            &no_coordinator
        )),
        INVALID_ARGUMENT
    );
    assert_eq!(
        bridge_code(no_coordinator.fetch(&NSFetchRequest::new("Person")?)),
        INVALID_ARGUMENT
    );
    Ok(())
}

#[test]
fn context_parenting_errors_do_not_abort() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("Parenting", &fixture.model)?;
    let root = container.new_background_context()?;
    let child = NSManagedObjectContext::new_private_queue()?;

    assert_eq!(
        bridge_code(child.set_parent_context(Some(&child))),
        INVALID_ARGUMENT
    );
    child.set_parent_context(Some(&root))?;
    assert_eq!(
        bridge_code(root.set_parent_context(Some(&child))),
        INVALID_ARGUMENT
    );
    let coordinator = in_memory_coordinator(&fixture.model)?;
    assert_eq!(
        bridge_code(child.set_persistent_store_coordinator(&coordinator)),
        OBJC_EXCEPTION
    );
    Ok(())
}

#[test]
fn malformed_predicates_return_errors() -> TestResult {
    assert_eq!(
        bridge_code(NSPredicate::from_format("name ==", &[])),
        OBJC_EXCEPTION
    );
    assert_eq!(
        bridge_code(NSPredicate::from_format("name == %@", &[])),
        OBJC_EXCEPTION
    );

    let contains = NSPredicate::from_format("name CONTAINS 'a'", &[])?;
    let numeric_name = BTreeMap::from([(String::from("name"), Value::from(5_i32))]);
    assert_eq!(
        bridge_code(contains.evaluate_with_object(Some(&numeric_name), None)),
        OBJC_EXCEPTION
    );

    let variable = NSPredicate::from_format("age >= $MIN_AGE", &[])?;
    let adult = BTreeMap::from([(String::from("age"), Value::from(30_i32))]);
    assert_eq!(
        bridge_code(variable.evaluate_with_object(Some(&adult), Some(&BTreeMap::new()))),
        OBJC_EXCEPTION
    );
    Ok(())
}

#[test]
fn argument_based_predicates_keep_values_out_of_the_format() -> TestResult {
    let hostile = "x' OR TRUEPREDICATE OR name == 'y";
    let equals = NSPredicate::comparison(
        "name",
        PredicateOperator::EqualTo,
        hostile,
        PredicateOptions::NONE,
    )?;
    let bea = BTreeMap::from([(String::from("name"), Value::from("Bea"))]);
    let hostile_row = BTreeMap::from([(String::from("name"), Value::from(hostile))]);
    assert!(!equals.evaluate_with_object(Some(&bea), None)?);
    assert!(equals.evaluate_with_object(Some(&hostile_row), None)?);

    let prefix = NSPredicate::comparison(
        "name",
        PredicateOperator::BeginsWith,
        "b",
        PredicateOptions::CASE_INSENSITIVE,
    )?;
    let old = NSPredicate::comparison(
        "age",
        PredicateOperator::GreaterThanOrEqualTo,
        40_i32,
        PredicateOptions::NONE,
    )?;
    let row = BTreeMap::from([
        (String::from("name"), Value::from("Bea")),
        (String::from("age"), Value::from(25_i32)),
    ]);
    assert!(prefix.evaluate_with_object(Some(&row), None)?);
    assert!(!old.evaluate_with_object(Some(&row), None)?);
    assert!(!NSPredicate::and(&[&prefix, &old])?.evaluate_with_object(Some(&row), None)?);
    assert!(NSPredicate::or(&[&prefix, &old])?.evaluate_with_object(Some(&row), None)?);
    assert!(NSPredicate::not(&old)?.evaluate_with_object(Some(&row), None)?);

    let fixture = basic_model()?;
    let container = in_memory_container("ArgumentPredicates", &fixture.model)?;
    let context = container.new_background_context()?;
    insert_named(&context, "Bea")?;
    insert_named(&context, hostile)?;
    context.save()?;
    let request = NSFetchRequest::new("Person")?;
    request.set_predicate(Some(&equals));
    let matches = context.fetch(&request)?;
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].value("name")?.as_str(), Some(hostile));
    Ok(())
}

#[test]
fn invalid_validation_rules_return_errors() -> TestResult {
    let fixture = basic_model()?;
    assert_eq!(
        bridge_code(
            fixture
                .name
                .set_validation_rules(&[ValidationRule::new("SELF ==", "broken")])
        ),
        OBJC_EXCEPTION
    );
    assert_eq!(
        bridge_code(
            fixture
                .name
                .set_validation_rules(&[ValidationRule::new("SELF == %@", "missing argument")])
        ),
        OBJC_EXCEPTION
    );
    fixture
        .name
        .set_validation_rules(&[ValidationRule::new("length > 1", "Name too short")])?;
    assert_eq!(fixture.name.validation_rules()?.len(), 1);
    Ok(())
}

#[test]
fn editing_a_model_in_use_returns_errors() -> TestResult {
    let fixture = basic_model()?;
    let _container = in_memory_container("ImmutableModel", &fixture.model)?;
    assert_eq!(
        bridge_code(fixture.person.set_name("Renamed")),
        OBJC_EXCEPTION
    );
    assert_eq!(bridge_code(fixture.age.set_optional(true)), OBJC_EXCEPTION);
    let extra = NSAttributeDescription::new("extra", AttributeType::String)?;
    assert_eq!(
        bridge_code(fixture.person.add_attribute(&extra)),
        OBJC_EXCEPTION
    );
    assert_eq!(fixture.person.name()?, "Person");
    Ok(())
}

#[test]
fn unknown_enum_values_are_rejected() -> TestResult {
    let attribute = NSAttributeDescription::new("value", AttributeType::String)?;
    assert_eq!(
        bridge_code(attribute.set_attribute_type(AttributeType::Unknown(12_345))),
        INVALID_ARGUMENT
    );
    let relationship = NSRelationshipDescription::new("link")?;
    assert_eq!(
        bridge_code(relationship.set_delete_rule(DeleteRule::Unknown(9))),
        INVALID_ARGUMENT
    );
    assert_eq!(
        bridge_code(relationship.set_max_count(usize::MAX)),
        INVALID_ARGUMENT
    );
    assert_eq!(
        bridge_code(NSMergePolicy::new(MergePolicyType::Unknown(99))),
        INVALID_ARGUMENT
    );
    let request = NSFetchRequest::new("Person")?;
    assert_eq!(
        bridge_code(request.set_result_type(FetchRequestResultType::Unknown(99))),
        INVALID_ARGUMENT
    );
    let history =
        NSPersistentHistoryChangeRequest::fetch_history_after_date(std::time::UNIX_EPOCH)?;
    assert_eq!(
        bridge_code(history.set_result_type(PersistentHistoryResultType::Unknown(99))),
        INVALID_ARGUMENT
    );
    let batch = NSBatchUpdateRequest::new("Person")?;
    assert_eq!(
        bridge_code(batch.set_result_type(BatchUpdateRequestResultType::Unknown(99))),
        INVALID_ARGUMENT
    );
    Ok(())
}

#[test]
fn oversized_fetch_limits_are_clamped_instead_of_trapping() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("FetchLimits", &fixture.model)?;
    let context = container.new_background_context()?;
    insert_named(&context, "Ada")?;
    insert_named(&context, "Bea")?;
    context.save()?;

    let request = NSFetchRequest::new("Person")?;
    request.set_fetch_limit(usize::MAX)?;
    assert_eq!(request.fetch_limit(), u32::MAX as usize);
    assert_eq!(context.fetch(&request)?.len(), 2);

    request.set_fetch_limit((1_usize << 32) + 1)?;
    assert_eq!(request.fetch_limit(), u32::MAX as usize);
    assert_eq!(context.fetch(&request)?.len(), 2);

    request.set_fetch_limit(1)?;
    assert_eq!(context.fetch(&request)?.len(), 1);

    request.set_fetch_limit(0)?;
    request.set_fetch_offset(usize::MAX)?;
    assert_eq!(request.fetch_offset(), usize::try_from(i64::MAX)?);
    assert!(context.fetch(&request)?.is_empty());

    request.set_fetch_offset(0)?;
    request.set_fetch_batch_size(usize::MAX)?;
    assert_eq!(request.fetch_batch_size(), usize::try_from(i64::MAX)?);
    assert_eq!(context.fetch(&request)?.len(), 2);
    Ok(())
}

#[test]
fn non_object_result_types_return_errors_from_fetch() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("ResultTypes", &fixture.model)?;
    let context = container.new_background_context()?;
    insert_named(&context, "Ada")?;
    context.save()?;

    let request = NSFetchRequest::new("Person")?;
    request.set_result_type(FetchRequestResultType::Dictionary)?;
    assert_eq!(bridge_code(context.fetch(&request)), INVALID_ARGUMENT);
    request.set_result_type(FetchRequestResultType::ManagedObjectId)?;
    assert_eq!(bridge_code(context.fetch(&request)), INVALID_ARGUMENT);
    assert_eq!(request.execute_object_ids(&context)?.len(), 1);
    assert!(matches!(
        request.result_type(),
        FetchRequestResultType::ManagedObjectId
    ));
    Ok(())
}

#[test]
fn store_and_request_misuse_returns_errors() -> TestResult {
    let fixture = basic_model()?;
    let coordinator = NSPersistentStoreCoordinator::new(&fixture.model)?;
    assert_eq!(
        bridge_code(coordinator.add_persistent_store("Bogus", None::<&str>, None::<&str>, None)),
        INVALID_ARGUMENT
    );

    let container = in_memory_container("RequestMisuse", &fixture.model)?;
    let context = container.new_background_context()?;
    insert_named(&context, "Ada")?;
    context.save()?;

    let delete = NSBatchDeleteRequest::from_fetch_request(&NSFetchRequest::new("Person")?)?;
    assert_eq!(bridge_code(delete.execute(&context)), INVALID_ARGUMENT);

    let unsorted = NSFetchRequest::new("Person")?;
    assert_eq!(
        bridge_code(NSFetchedResultsController::new(
            &unsorted, &context, None, None
        )),
        INVALID_ARGUMENT
    );
    let unknown = NSFetchRequest::new("Nope")?;
    unknown.set_sort_descriptors(&[SortDescriptor::ascending("name")])?;
    assert_eq!(
        bridge_code(NSFetchedResultsController::new(
            &unknown, &context, None, None
        )),
        INVALID_ARGUMENT
    );

    let sorted = NSFetchRequest::new("Person")?;
    sorted.set_sort_descriptors(&[SortDescriptor::ascending("name")])?;
    let sectioned = NSFetchedResultsController::new(&sorted, &context, Some("bogus"), None)?;
    assert_eq!(bridge_code(sectioned.perform_fetch()), OBJC_EXCEPTION);
    Ok(())
}

#[test]
fn loading_a_container_without_store_descriptions_completes_immediately() -> TestResult {
    let fixture = basic_model()?;
    let container = NSPersistentContainer::new("NoDescriptions", &fixture.model)?;
    container.set_persistent_store_descriptions(&[])?;
    let started = Instant::now();
    container.load_persistent_stores()?;
    assert!(started.elapsed() < Duration::from_secs(5));
    Ok(())
}

#[test]
fn unsupported_store_types_fail_to_load_without_aborting() -> TestResult {
    let fixture = basic_model()?;
    let container = NSPersistentContainer::new("BogusStore", &fixture.model)?;
    let description = NSPersistentStoreDescription::new()?;
    description.set_store_type("Bogus")?;
    description.set_should_add_asynchronously(false);
    container.set_persistent_store_descriptions(&[&description])?;
    assert_eq!(
        bridge_code(container.load_persistent_stores()),
        INVALID_ARGUMENT
    );
    Ok(())
}

#[test]
fn save_changes_requests_return_the_original_handles() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("SaveChanges", &fixture.model)?;
    let context = container.new_background_context()?;
    let object = insert_named(&context, "Ada")?;
    let request = NSSaveChangesRequest::new(&[&object, &object], &[], &[], &[])?;
    let inserted = request.inserted_objects()?;
    assert_eq!(inserted.len(), 1);
    assert_eq!(inserted[0].value("name")?.as_str(), Some("Ada"));
    assert!(request.updated_objects()?.is_empty());
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FrcEvent {
    WillChange,
    Object(
        NSFetchedResultsChangeType,
        Option<FetchedResultsIndexPath>,
        Option<String>,
    ),
    DidChange,
}

struct RecordingDelegate {
    events: Arc<Mutex<Vec<FrcEvent>>>,
}

impl NSFetchedResultsControllerDelegate for RecordingDelegate {
    fn controller_will_change_content(&self, _controller: &NSFetchedResultsController) {
        self.events.lock().unwrap().push(FrcEvent::WillChange);
    }

    fn controller_did_change_object(
        &self,
        _controller: &NSFetchedResultsController,
        object: &NSManagedObject,
        change: NSFetchedResultsChangeType,
        _index_path: Option<FetchedResultsIndexPath>,
        new_index_path: Option<FetchedResultsIndexPath>,
    ) {
        let name = object
            .value("name")
            .ok()
            .and_then(|value| value.as_str().map(str::to_owned));
        self.events
            .lock()
            .unwrap()
            .push(FrcEvent::Object(change, new_index_path, name));
    }

    fn controller_did_change_content(&self, _controller: &NSFetchedResultsController) {
        self.events.lock().unwrap().push(FrcEvent::DidChange);
    }
}

struct PanickingDelegate;

impl NSFetchedResultsControllerDelegate for PanickingDelegate {
    fn controller_will_change_content(&self, _controller: &NSFetchedResultsController) {
        panic!("delegate panic");
    }
}

fn sorted_controller(
    context: &NSManagedObjectContext,
) -> Result<NSFetchedResultsController, CoreDataError> {
    let request = NSFetchRequest::new("Person")?;
    request.set_sort_descriptors(&[SortDescriptor::ascending("name")])?;
    let controller = NSFetchedResultsController::new(&request, context, None, None)?;
    controller.perform_fetch()?;
    Ok(controller)
}

#[test]
fn fetched_results_delegate_reports_changes_until_the_registration_drops() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("FrcDelegate", &fixture.model)?;
    let context = container.new_background_context()?;
    insert_named(&context, "Bea")?;
    context.save()?;

    let controller = sorted_controller(&context)?;
    let events = Arc::new(Mutex::new(Vec::new()));
    let registration = controller.set_delegate(RecordingDelegate {
        events: Arc::clone(&events),
    })?;

    insert_named(&context, "Ada")?;
    context.save()?;
    assert_eq!(
        *events.lock().unwrap(),
        vec![
            FrcEvent::WillChange,
            FrcEvent::Object(
                NSFetchedResultsChangeType::Insert,
                Some(FetchedResultsIndexPath::new(0, 0)),
                Some(String::from("Ada")),
            ),
            FrcEvent::DidChange,
        ]
    );
    assert_eq!(controller.fetched_objects()?.len(), 2);

    drop(registration);
    events.lock().unwrap().clear();
    insert_named(&context, "Cleo")?;
    context.save()?;
    assert!(events.lock().unwrap().is_empty());
    assert_eq!(Arc::strong_count(&events), 1);
    Ok(())
}

#[test]
fn fetched_results_delegate_panics_are_contained() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("FrcPanic", &fixture.model)?;
    let context = container.new_background_context()?;
    let controller = sorted_controller(&context)?;
    let _registration = controller.set_delegate(PanickingDelegate)?;

    insert_named(&context, "Ada")?;
    context.save()?;
    assert_eq!(controller.fetched_objects()?.len(), 1);
    assert_eq!(controller.sections()?[0].number_of_objects()?, 1);
    Ok(())
}

#[test]
fn fetched_results_sections_are_usable_from_other_threads() -> TestResult {
    let fixture = basic_model()?;
    let container = in_memory_container("FrcThreads", &fixture.model)?;
    let context = container.new_background_context()?;
    insert_named(&context, "Ada")?;
    insert_named(&context, "Bea")?;
    context.save()?;
    let controller = sorted_controller(&context)?;

    let names = std::thread::scope(|scope| {
        scope
            .spawn(|| -> Result<Vec<String>, CoreDataError> {
                let section = controller.sections()?.remove(0);
                section
                    .objects()?
                    .iter()
                    .map(|object| {
                        object
                            .value("name")
                            .map(|value| value.as_str().unwrap_or_default().to_owned())
                    })
                    .collect()
            })
            .join()
            .expect("thread panicked")
    })?;
    assert_eq!(names, vec![String::from("Ada"), String::from("Bea")]);
    Ok(())
}
