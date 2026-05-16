#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use coredata::prelude::*;
use coredata::COREDATA_BRIDGE_ERROR_DOMAIN;

pub struct BasicModelFixture {
    pub model: NSManagedObjectModel,
    pub person: NSEntityDescription,
    pub name: NSAttributeDescription,
    pub age: NSAttributeDescription,
}

pub struct RelationshipModelFixture {
    pub model: NSManagedObjectModel,
    pub person: NSEntityDescription,
    pub task: NSEntityDescription,
    pub children: NSRelationshipDescription,
    pub parent: NSRelationshipDescription,
}

pub struct MigrationModelFixture {
    pub source_model: NSManagedObjectModel,
    pub destination_model: NSManagedObjectModel,
    pub source_person: NSEntityDescription,
    pub destination_person: NSEntityDescription,
}

#[derive(Debug, Clone)]
pub struct SqliteStoreArtifact {
    pub path: PathBuf,
}

impl SqliteStoreArtifact {
    pub fn new(prefix: &str) -> Result<Self, CoreDataError> {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/coredata-artifacts");
        fs::create_dir_all(&root).map_err(|error| CoreDataError {
            domain: COREDATA_BRIDGE_ERROR_DOMAIN.into(),
            code: -1,
            message: format!("failed to create artifact directory: {error}"),
        })?;
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| CoreDataError {
                domain: COREDATA_BRIDGE_ERROR_DOMAIN.into(),
                code: -1,
                message: format!("clock drift: {error}"),
            })?
            .as_nanos();
        Ok(Self {
            path: root.join(format!("{prefix}-{}-{nanos}.sqlite", process::id())),
        })
    }

    pub fn cleanup(&self) {
        for suffix in ["", "-shm", "-wal"] {
            let target = if suffix.is_empty() {
                self.path.clone()
            } else {
                PathBuf::from(format!("{}{}", self.path.display(), suffix))
            };
            if let Err(error) = fs::remove_file(&target) {
                if error.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("cleanup warning for {}: {error}", target.display());
                }
            }
        }
    }
}

pub fn basic_model() -> Result<BasicModelFixture, CoreDataError> {
    let model = NSManagedObjectModel::new()?;
    let person = NSEntityDescription::named("Person")?;
    person.set_managed_object_class_name("NSManagedObject")?;

    let name = NSAttributeDescription::new("name", AttributeType::String)?;
    name.set_optional(false)?;

    let age = NSAttributeDescription::new("age", AttributeType::Integer32)?;
    age.set_optional(false)?;

    person.add_attribute(&name)?;
    person.add_attribute(&age)?;
    model.add_entity(&person)?;

    Ok(BasicModelFixture {
        model,
        person,
        name,
        age,
    })
}

pub fn relationship_model() -> Result<RelationshipModelFixture, CoreDataError> {
    let model = NSManagedObjectModel::new()?;

    let person = NSEntityDescription::named("Person")?;
    person.set_managed_object_class_name("NSManagedObject")?;
    let person_name = NSAttributeDescription::new("name", AttributeType::String)?;
    person_name.set_optional(false)?;
    person.add_attribute(&person_name)?;

    let task = NSEntityDescription::named("Task")?;
    task.set_managed_object_class_name("NSManagedObject")?;
    let title = NSAttributeDescription::new("title", AttributeType::String)?;
    title.set_optional(false)?;
    task.add_attribute(&title)?;

    let children = NSRelationshipDescription::new("tasks")?;
    children.set_destination_entity(Some(&task))?;
    children.set_optional(true)?;
    children.set_min_count(0)?;
    children.set_max_count(0)?;
    children.set_delete_rule(DeleteRule::Cascade)?;

    let parent = NSRelationshipDescription::new("owner")?;
    parent.set_destination_entity(Some(&person))?;
    parent.set_optional(true)?;
    parent.set_min_count(0)?;
    parent.set_max_count(1)?;
    parent.set_delete_rule(DeleteRule::Nullify)?;

    children.set_inverse_relationship(Some(&parent))?;
    parent.set_inverse_relationship(Some(&children))?;

    person.add_relationship(&children)?;
    task.add_relationship(&parent)?;

    model.add_entity(&person)?;
    model.add_entity(&task)?;

    Ok(RelationshipModelFixture {
        model,
        person,
        task,
        children,
        parent,
    })
}

pub fn migration_models() -> Result<MigrationModelFixture, CoreDataError> {
    let source_model = NSManagedObjectModel::new()?;
    let source_person = NSEntityDescription::named("Person")?;
    source_person.set_managed_object_class_name("NSManagedObject")?;

    let source_name = NSAttributeDescription::new("name", AttributeType::String)?;
    source_name.set_optional(false)?;
    let source_age = NSAttributeDescription::new("age", AttributeType::Integer32)?;
    source_age.set_optional(false)?;
    source_person.add_attribute(&source_name)?;
    source_person.add_attribute(&source_age)?;
    source_model.add_entity(&source_person)?;

    let destination_model = NSManagedObjectModel::new()?;
    let destination_person = NSEntityDescription::named("Person")?;
    destination_person.set_managed_object_class_name("NSManagedObject")?;

    let destination_name = NSAttributeDescription::new("name", AttributeType::String)?;
    destination_name.set_optional(false)?;
    let destination_age = NSAttributeDescription::new("age", AttributeType::Integer32)?;
    destination_age.set_optional(false)?;
    let nickname = NSAttributeDescription::new("nickname", AttributeType::String)?;
    nickname.set_optional(true)?;
    destination_person.add_attribute(&destination_name)?;
    destination_person.add_attribute(&destination_age)?;
    destination_person.add_attribute(&nickname)?;
    destination_model.add_entity(&destination_person)?;

    Ok(MigrationModelFixture {
        source_model,
        destination_model,
        source_person,
        destination_person,
    })
}

pub fn in_memory_coordinator(
    model: &NSManagedObjectModel,
) -> Result<NSPersistentStoreCoordinator, CoreDataError> {
    let coordinator = NSPersistentStoreCoordinator::new(model)?;
    coordinator.add_persistent_store(store_types::IN_MEMORY, None::<&str>, None::<&str>, None)?;
    Ok(coordinator)
}

pub fn in_memory_context(
    model: &NSManagedObjectModel,
) -> Result<(NSPersistentStoreCoordinator, NSManagedObjectContext), CoreDataError> {
    let coordinator = in_memory_coordinator(model)?;
    let context = NSManagedObjectContext::new_main_queue()?;
    context.set_persistent_store_coordinator(&coordinator)?;
    Ok((coordinator, context))
}

pub fn in_memory_container(
    name: &str,
    model: &NSManagedObjectModel,
) -> Result<NSPersistentContainer, CoreDataError> {
    let container = NSPersistentContainer::new(name, model)?;
    let description = NSPersistentStoreDescription::new()?;
    description.set_store_type(store_types::IN_MEMORY)?;
    description.set_should_add_asynchronously(false);
    container.set_persistent_store_descriptions(&[&description])?;
    container.load_persistent_stores()?;
    Ok(container)
}

pub fn sqlite_container(
    name: &str,
    model: &NSManagedObjectModel,
    prefix: &str,
    history_tracking: bool,
) -> Result<(NSPersistentContainer, SqliteStoreArtifact), CoreDataError> {
    let artifact = SqliteStoreArtifact::new(prefix)?;
    let container = NSPersistentContainer::new(name, model)?;
    let description = NSPersistentStoreDescription::with_url(&artifact.path)?;
    description.set_store_type(store_types::SQLITE)?;
    description.set_should_add_asynchronously(false);
    description.set_should_migrate_automatically(true);
    description.set_should_infer_mapping_model_automatically(true);
    if history_tracking {
        description.set_option(option_keys::PERSISTENT_HISTORY_TRACKING, Some(true.into()))?;
        description.set_option(
            option_keys::REMOTE_CHANGE_NOTIFICATION_POST,
            Some(true.into()),
        )?;
    }
    container.set_persistent_store_descriptions(&[&description])?;
    container.load_persistent_stores()?;
    Ok((container, artifact))
}

pub fn insert_person(
    context: &NSManagedObjectContext,
    entity: &NSEntityDescription,
    name: &str,
    age: i32,
) -> Result<NSManagedObject, CoreDataError> {
    let object = NSManagedObject::new(entity, None)?;
    context.insert(&object)?;
    object.set_value("name", name)?;
    object.set_value("age", age)?;
    Ok(object)
}

pub fn person_row(name: &str, age: i32) -> BTreeMap<String, Value> {
    BTreeMap::from([
        (String::from("name"), Value::from(name)),
        (String::from("age"), Value::from(age)),
    ])
}
