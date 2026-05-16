use coredata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let coordinator = NSPersistentStoreCoordinator::new(&model)?;
    coordinator.add_persistent_store(store_types::IN_MEMORY, None::<&str>, None::<&str>, None)?;

    let context = NSManagedObjectContext::new_main_queue()?;
    context.set_persistent_store_coordinator(&coordinator)?;

    context.perform_and_wait(|ctx| -> Result<(), CoreDataError> {
        for (name, age) in [("Ada", 32_i32), ("Bea", 25_i32), ("Cleo", 41_i32)] {
            let object = NSManagedObject::new(&person, None)?;
            ctx.insert(&object)?;
            object.set_value("name", name)?;
            object.set_value("age", age)?;
        }

        if !ctx.has_changes() {
            return Err(CoreDataError {
                domain: COREDATA_BRIDGE_ERROR_DOMAIN.into(),
                code: -1,
                message: "context should have pending changes".into(),
            });
        }

        ctx.save()?;

        let request = NSFetchRequest::new("Person")?;
        request.set_sort_descriptors(&[SortDescriptor::ascending("age")])?;
        let people = request.execute(&ctx)?;
        if people.len() != 3 {
            return Err(CoreDataError {
                domain: COREDATA_BRIDGE_ERROR_DOMAIN.into(),
                code: -1,
                message: format!("expected 3 people, found {}", people.len()),
            });
        }

        let ages = people
            .iter()
            .map(|person| {
                person.value("age")?.as_i64().ok_or_else(|| CoreDataError {
                    domain: COREDATA_BRIDGE_ERROR_DOMAIN.into(),
                    code: -1,
                    message: "age was not numeric".into(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        if ages != vec![25, 32, 41] {
            return Err(CoreDataError {
                domain: COREDATA_BRIDGE_ERROR_DOMAIN.into(),
                code: -1,
                message: format!("unexpected age ordering: {ages:?}"),
            });
        }

        Ok(())
    })?;

    println!("✅ coredata insert + fetch OK");
    Ok(())
}
