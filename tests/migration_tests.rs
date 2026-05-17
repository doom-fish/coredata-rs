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

        let source_context = source_container.view_context()?;
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

        let destination_context = destination_container.view_context()?;
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
