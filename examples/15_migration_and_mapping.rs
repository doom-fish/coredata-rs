#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = migration_models()?;
    let source_artifact = SqliteStoreArtifact::new("migration-example-source")?;
    let destination_artifact = SqliteStoreArtifact::new("migration-example-destination")?;

    {
        let source_container =
            NSPersistentContainer::new("MigrationExampleSource", &fixture.source_model)?;
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
    let migration_manager =
        NSMigrationManager::new(&fixture.source_model, &fixture.destination_model)?;
    migration_manager.set_uses_store_specific_migration_manager(false);
    migration_manager.migrate_store(
        &source_artifact.path,
        store_types::SQLITE,
        None,
        Some(&mapping_model),
        &destination_artifact.path,
        store_types::SQLITE,
        None,
    )?;

    {
        let destination_container =
            NSPersistentContainer::new("MigrationExampleDestination", &fixture.destination_model)?;
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
    }

    source_artifact.cleanup();
    destination_artifact.cleanup();
    println!("✅ migration and mapping example OK");
    Ok(())
}
