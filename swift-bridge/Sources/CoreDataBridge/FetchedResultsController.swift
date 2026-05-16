import CoreData
import Foundation

@_cdecl("cd_fetched_results_controller_new")
public func cdFetchedResultsControllerNew(
    _ fetchRequestPtr: UnsafeMutableRawPointer?,
    _ managedObjectContextPtr: UnsafeMutableRawPointer?,
    _ sectionNameKeyPath: UnsafePointer<CChar>?,
    _ cacheName: UnsafePointer<CChar>?,
    _ outController: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let fetchRequestPtr, let managedObjectContextPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetched-results fetch request or managed object context")
        }
        let fetchRequest: NSFetchRequest<NSManagedObject> = cdBorrow(fetchRequestPtr)
        let managedObjectContext: NSManagedObjectContext = cdBorrow(managedObjectContextPtr)
        let controller = NSFetchedResultsController(
            fetchRequest: fetchRequest,
            managedObjectContext: managedObjectContext,
            sectionNameKeyPath: cdOptionalString(sectionNameKeyPath),
            cacheName: cdOptionalString(cacheName)
        )
        outController?.pointee = cdRetain(controller)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetched_results_controller_perform_fetch")
public func cdFetchedResultsControllerPerformFetch(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let controllerPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetched results controller")
        }
        let controller: NSFetchedResultsController<NSManagedObject> = cdBorrow(controllerPtr)
        try controller.performFetch()
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetched_results_controller_get_fetch_request")
public func cdFetchedResultsControllerGetFetchRequest(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let controllerPtr else {
        return nil
    }
    let controller: NSFetchedResultsController<NSManagedObject> = cdBorrow(controllerPtr)
    return cdRetain(controller.fetchRequest)
}

@_cdecl("cd_fetched_results_controller_get_managed_object_context")
public func cdFetchedResultsControllerGetManagedObjectContext(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let controllerPtr else {
        return nil
    }
    let controller: NSFetchedResultsController<NSManagedObject> = cdBorrow(controllerPtr)
    return cdRetain(controller.managedObjectContext)
}

@_cdecl("cd_fetched_results_controller_get_section_name_key_path")
public func cdFetchedResultsControllerGetSectionNameKeyPath(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let controllerPtr else {
        return nil
    }
    let controller: NSFetchedResultsController<NSManagedObject> = cdBorrow(controllerPtr)
    guard let value = controller.sectionNameKeyPath else {
        return nil
    }
    return cdCString(value)
}

@_cdecl("cd_fetched_results_controller_get_cache_name")
public func cdFetchedResultsControllerGetCacheName(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let controllerPtr else {
        return nil
    }
    let controller: NSFetchedResultsController<NSManagedObject> = cdBorrow(controllerPtr)
    guard let value = controller.cacheName else {
        return nil
    }
    return cdCString(value)
}

@_cdecl("cd_fetched_results_controller_get_fetched_objects")
public func cdFetchedResultsControllerGetFetchedObjects(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let controllerPtr else {
        return nil
    }
    let controller: NSFetchedResultsController<NSManagedObject> = cdBorrow(controllerPtr)
    guard let fetchedObjects = controller.fetchedObjects else {
        return nil
    }
    return cdRetain(fetchedObjects as NSArray)
}

@_cdecl("cd_fetched_results_controller_get_sections")
public func cdFetchedResultsControllerGetSections(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let controllerPtr else {
        return nil
    }
    let controller: NSFetchedResultsController<NSManagedObject> = cdBorrow(controllerPtr)
    guard let sections = controller.sections else {
        return nil
    }
    return cdRetain(sections as NSArray)
}

@_cdecl("cd_fetched_results_controller_get_section_index_titles_json")
public func cdFetchedResultsControllerGetSectionIndexTitlesJSON(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let controllerPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetched results controller")
        }
        let controller: NSFetchedResultsController<NSManagedObject> = cdBorrow(controllerPtr)
        outJSON?.pointee = cdCString(try cdEncodeJSON(controller.sectionIndexTitles))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetched_results_controller_object_at_index_path")
public func cdFetchedResultsControllerObjectAtIndexPath(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ section: Int64,
    _ item: Int64,
    _ outObject: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let controllerPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetched results controller")
        }
        let controller: NSFetchedResultsController<NSManagedObject> = cdBorrow(controllerPtr)
        guard section >= 0, item >= 0 else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Negative fetched results index path")
        }
        let sectionIndex = Int(section)
        let itemIndex = Int(item)
        guard let sections = controller.sections, sectionIndex < sections.count else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Fetched results section out of range")
        }
        guard itemIndex < sections[sectionIndex].numberOfObjects else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Fetched results item out of range")
        }
        let object = controller.object(at: IndexPath(indexes: [sectionIndex, itemIndex]))
        outObject?.pointee = cdRetain(object)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetched_results_controller_index_path_for_object")
public func cdFetchedResultsControllerIndexPathForObject(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outSection: UnsafeMutablePointer<Int64>?,
    _ outItem: UnsafeMutablePointer<Int64>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let controllerPtr, let objectPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing fetched results controller or object")
        }
        let controller: NSFetchedResultsController<NSManagedObject> = cdBorrow(controllerPtr)
        let object: NSManagedObject = cdBorrow(objectPtr)
        let indexPath = controller.indexPath(forObject: object)
        if let indexPath, indexPath.count >= 2 {
            outSection?.pointee = Int64(indexPath[0])
            outItem?.pointee = Int64(indexPath[1])
        } else {
            outSection?.pointee = -1
            outItem?.pointee = -1
        }
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_fetched_results_controller_delete_cache_with_name")
public func cdFetchedResultsControllerDeleteCacheWithName(_ name: UnsafePointer<CChar>?) {
    NSFetchedResultsController<NSManagedObject>.deleteCache(withName: cdOptionalString(name))
}

private func cdFetchedResultsSectionInfo(_ sectionInfoPtr: UnsafeMutableRawPointer) -> any NSFetchedResultsSectionInfo {
    guard let sectionInfo = cdBorrowBox(sectionInfoPtr).object as? (NSObject & NSFetchedResultsSectionInfo) else {
        fatalError("Unexpected fetched results section info type")
    }
    return sectionInfo
}

@_cdecl("cd_fetched_results_section_info_get_name")
public func cdFetchedResultsSectionInfoGetName(_ sectionInfoPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let sectionInfoPtr else {
        return nil
    }
    return cdCString(cdFetchedResultsSectionInfo(sectionInfoPtr).name)
}

@_cdecl("cd_fetched_results_section_info_get_index_title")
public func cdFetchedResultsSectionInfoGetIndexTitle(_ sectionInfoPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let sectionInfoPtr else {
        return nil
    }
    guard let value = cdFetchedResultsSectionInfo(sectionInfoPtr).indexTitle else {
        return nil
    }
    return cdCString(value)
}

@_cdecl("cd_fetched_results_section_info_get_number_of_objects")
public func cdFetchedResultsSectionInfoGetNumberOfObjects(_ sectionInfoPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let sectionInfoPtr else {
        return 0
    }
    return UInt64(cdFetchedResultsSectionInfo(sectionInfoPtr).numberOfObjects)
}

@_cdecl("cd_fetched_results_section_info_get_objects")
public func cdFetchedResultsSectionInfoGetObjects(_ sectionInfoPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let sectionInfoPtr else {
        return nil
    }
    guard let objects = cdFetchedResultsSectionInfo(sectionInfoPtr).objects else {
        return nil
    }
    return cdRetain(objects as NSArray)
}
