#![allow(clippy::wildcard_imports)]

mod support;

use coredata::prelude::*;
use support::*;

#[test]
fn fetched_results_controller_fetches_sorted_objects_and_sections(
) -> Result<(), Box<dyn std::error::Error>> {
    let fixture = basic_model()?;
    let container = in_memory_container("FetchedResultsTests", &fixture.model)?;
    let context = container.new_background_context()?;

    insert_person(&context, &fixture.person, "Ada", 32)?;
    insert_person(&context, &fixture.person, "Bea", 25)?;
    insert_person(&context, &fixture.person, "Cleo", 41)?;
    context.save()?;

    let request = NSFetchRequest::new("Person")?;
    request.set_sort_descriptors(&[SortDescriptor::ascending("age")])?;

    let controller = NSFetchedResultsController::new(&request, &context, None, None)?;
    controller.perform_fetch()?;

    let objects = controller.fetched_objects()?;
    assert_eq!(objects.len(), 3);
    assert_eq!(objects[0].value("name")?.as_str(), Some("Bea"));
    assert_eq!(objects[2].value("name")?.as_str(), Some("Cleo"));

    let sections = controller.sections()?;
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].number_of_objects(), 3);
    assert_eq!(sections[0].objects()?.len(), 3);

    let second = controller.object_at_index_path(FetchedResultsIndexPath::new(0, 1))?;
    assert_eq!(second.value("name")?.as_str(), Some("Ada"));

    let index_path = controller.index_path_for_object(&objects[2])?;
    assert_eq!(index_path, Some(FetchedResultsIndexPath::new(0, 2)));
    assert!(controller.section_index_titles()?.is_empty());
    Ok(())
}
