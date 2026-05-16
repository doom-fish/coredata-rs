#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    let container = in_memory_container("FetchedResultsExample", &fixture.model)?;
    let context = container.new_background_context()?;

    insert_person(&context, &fixture.person, "Ada", 32)?;
    insert_person(&context, &fixture.person, "Bea", 25)?;
    insert_person(&context, &fixture.person, "Cleo", 41)?;
    context.save()?;

    let request = NSFetchRequest::new("Person")?;
    request.set_sort_descriptors(&[SortDescriptor::ascending("age")])?;

    let controller = NSFetchedResultsController::new(&request, &context, None, None)?;
    controller.perform_fetch()?;

    let first = controller.object_at_index_path(FetchedResultsIndexPath::new(0, 0))?;
    assert_eq!(first.value("name")?.as_str(), Some("Bea"));
    assert_eq!(controller.sections()?[0].number_of_objects(), 3);

    println!("✅ fetched results controller example OK");
    Ok(())
}
