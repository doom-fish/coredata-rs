#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

#[test]
fn inferred_mapping_models_and_migration_manager_migrate_sqlite_store(
) -> Result<(), Box<dyn std::error::Error>> {
    let fixture = migration_models()?;
    let source_artifact = SqliteStoreArtifact::new("migration-source")?;
    let destination_artifact = SqliteStoreArtifact::new("migration-destination")?;

    {
        let source_container =
            NSPersistentContainer::new("MigrationSource", &fixture.source_model)?;
        let source_description = NSPersistentStoreDescription::with_url(&source_artifact.path)?;
        source_description.set_store_type(store_types::SQLITE)?;
        source_description.set_should_add_asynchronously(false);
        source_container.set_persistent_store_descriptions(&[&source_description])?;
        source_container.load_persistent_stores()?;

        let source_context = source_container.new_background_context()?;
        insert_person(&source_context, &fixture.source_person, "Ada", 32)?;
        source_context.save()?;
    }

    let mapping_model =
        NSMappingModel::inferred(&fixture.source_model, &fixture.destination_model)?;
    assert_eq!(mapping_model.entity_mapping_names()?.len(), 1);

    let migration_manager =
        NSMigrationManager::new(&fixture.source_model, &fixture.destination_model)?;
    migration_manager.set_uses_store_specific_migration_manager(false);
    assert!(!migration_manager.uses_store_specific_migration_manager());
    migration_manager.migrate_store(
        &source_artifact.path,
        store_types::SQLITE,
        None,
        Some(&mapping_model),
        &destination_artifact.path,
        store_types::SQLITE,
        None,
    )?;
    assert!(migration_manager.migration_progress() >= 0.0);

    {
        let destination_container =
            NSPersistentContainer::new("MigrationDestination", &fixture.destination_model)?;
        let destination_description =
            NSPersistentStoreDescription::with_url(&destination_artifact.path)?;
        destination_description.set_store_type(store_types::SQLITE)?;
        destination_description.set_should_add_asynchronously(false);
        destination_container.set_persistent_store_descriptions(&[&destination_description])?;
        destination_container.load_persistent_stores()?;

        let destination_context = destination_container.new_background_context()?;
        let request = NSFetchRequest::new("Person")?;
        request.set_sort_descriptors(&[SortDescriptor::ascending("name")])?;
        let migrated = request.execute(&destination_context)?;
        assert_eq!(migrated.len(), 1);
        assert_eq!(migrated[0].value("name")?.as_str(), Some("Ada"));
        assert_eq!(migrated[0].value("age")?.as_i32(), Some(32));
    }

    source_artifact.cleanup();
    destination_artifact.cleanup();
    Ok(())
}

fn load_sqlite_container(
    name: &str,
    model: &NSManagedObjectModel,
    path: &std::path::Path,
    manager: Option<&NSStagedMigrationManager>,
) -> Result<NSPersistentContainer, CoreDataError> {
    let container = NSPersistentContainer::new(name, model)?;
    let description = NSPersistentStoreDescription::with_url(path)?;
    description.set_store_type(store_types::SQLITE)?;
    description.set_should_add_asynchronously(false);
    if let Some(manager) = manager {
        description.set_staged_migration_manager(Some(manager))?;
    }
    container.set_persistent_store_descriptions(&[&description])?;
    container.load_persistent_stores()?;
    Ok(container)
}

fn staged_stage(
    fixture: &MigrationModelFixture,
    artifact: &SqliteStoreArtifact,
) -> Result<NSCustomMigrationStage, Box<dyn std::error::Error>> {
    {
        let source =
            load_sqlite_container("StagedSource", &fixture.source_model, &artifact.path, None)?;
        let context = source.new_background_context()?;
        insert_person(&context, &fixture.source_person, "Ada", 32)?;
        context.save()?;
    }
    let source_checksum = fixture
        .source_model
        .version_checksum()
        .ok_or("source model has no version checksum")?;
    let destination_checksum = fixture
        .destination_model
        .version_checksum()
        .ok_or("destination model has no version checksum")?;
    let current =
        NSManagedObjectModelReference::new_with_model(&fixture.source_model, &source_checksum)?;
    let next = NSManagedObjectModelReference::new_with_model(
        &fixture.destination_model,
        &destination_checksum,
    )?;
    Ok(NSCustomMigrationStage::new(&current, &next)?)
}

#[test]
fn custom_migration_stage_handlers_run_during_a_staged_migration(
) -> Result<(), Box<dyn std::error::Error>> {
    let fixture = migration_models()?;
    let artifact = SqliteStoreArtifact::new("staged-migration")?;
    let stage = staged_stage(&fixture, &artifact)?;

    let calls = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let will_calls = std::sync::Arc::clone(&calls);
    let will = stage.set_will_migrate_handler(move |_manager, _stage| {
        will_calls.lock().unwrap().push("will");
        Ok(())
    })?;
    let did_calls = std::sync::Arc::clone(&calls);
    let did = stage.set_did_migrate_handler(move |manager, _stage| {
        did_calls.lock().unwrap().push("did");
        assert!(manager.container()?.is_some());
        Ok(())
    })?;
    let manager = NSStagedMigrationManager::new(&[&stage.as_migration_stage()?])?;

    let destination = load_sqlite_container(
        "StagedDestination",
        &fixture.destination_model,
        &artifact.path,
        Some(&manager),
    )?;
    assert_eq!(*calls.lock().unwrap(), vec!["will", "did"]);

    let context = destination.new_background_context()?;
    let people = context.fetch(&NSFetchRequest::new("Person")?)?;
    assert_eq!(people.len(), 1);
    assert_eq!(people[0].value("name")?.as_str(), Some("Ada"));
    assert!(matches!(people[0].value("nickname")?, Value::Null));

    drop((will, did));
    assert_eq!(std::sync::Arc::strong_count(&calls), 1);
    artifact.cleanup();
    Ok(())
}

#[test]
fn failing_custom_migration_stage_handler_fails_the_store_load(
) -> Result<(), Box<dyn std::error::Error>> {
    let fixture = migration_models()?;
    let artifact = SqliteStoreArtifact::new("staged-migration-failure")?;
    let stage = staged_stage(&fixture, &artifact)?;
    let _will = stage.set_will_migrate_handler(|_manager, _stage| {
        Err(CoreDataError {
            domain: String::from("StagedMigrationTest"),
            code: 42,
            message: String::from("refusing to migrate"),
        })
    })?;
    let manager = NSStagedMigrationManager::new(&[&stage.as_migration_stage()?])?;

    let result = load_sqlite_container(
        "StagedFailure",
        &fixture.destination_model,
        &artifact.path,
        Some(&manager),
    );
    assert!(result.is_err());
    artifact.cleanup();
    Ok(())
}
