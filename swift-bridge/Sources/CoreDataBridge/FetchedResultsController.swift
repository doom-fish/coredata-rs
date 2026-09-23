import CoreData
import CoreDataObjCBridge
import Foundation

public typealias CDFetchedResultsCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    Int32,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UInt64,
    Int64,
    Int64,
    Int64,
    Int64
) -> Void

let CDFRC_EVENT_WILL_CHANGE_CONTENT: Int32 = 0
let CDFRC_EVENT_DID_CHANGE_OBJECT: Int32 = 1
let CDFRC_EVENT_DID_CHANGE_SECTION: Int32 = 2
let CDFRC_EVENT_DID_CHANGE_CONTENT: Int32 = 3

private let cdFetchedResultsDelegateKey = UnsafeMutableRawPointer.allocate(byteCount: 1, alignment: 1)

final class CDFetchedResultsDelegate: NSObject, NSFetchedResultsControllerDelegate {
    let token: UnsafeMutableRawPointer
    private let callback: CDFetchedResultsCallback
    private let release: CDVoidCallback

    init(token: UnsafeMutableRawPointer, callback: @escaping CDFetchedResultsCallback, release: @escaping CDVoidCallback) {
        self.token = token
        self.callback = callback
        self.release = release
    }

    deinit {
        release(token)
    }

    private func indexComponents(_ indexPath: IndexPath?) -> (Int64, Int64) {
        guard let indexPath, indexPath.count >= 2 else {
            return (-1, -1)
        }
        return (Int64(indexPath[0]), Int64(indexPath[1]))
    }

    func controllerWillChangeContent(_ controller: NSFetchedResultsController<NSFetchRequestResult>) {
        callback(token, CDFRC_EVENT_WILL_CHANGE_CONTENT, cdRetain(controller), nil, 0, -1, -1, -1, -1)
    }

    func controller(
        _ controller: NSFetchedResultsController<NSFetchRequestResult>,
        didChange anObject: Any,
        at indexPath: IndexPath?,
        for type: NSFetchedResultsChangeType,
        newIndexPath: IndexPath?
    ) {
        guard let object = anObject as? NSManagedObject else {
            return
        }
        let (fromSection, fromItem) = indexComponents(indexPath)
        let (toSection, toItem) = indexComponents(newIndexPath)
        callback(
            token,
            CDFRC_EVENT_DID_CHANGE_OBJECT,
            cdRetain(controller),
            cdRetainManagedObject(object, in: controller.managedObjectContext),
            UInt64(type.rawValue),
            fromSection,
            fromItem,
            toSection,
            toItem
        )
    }

    func controller(
        _ controller: NSFetchedResultsController<NSFetchRequestResult>,
        didChange sectionInfo: NSFetchedResultsSectionInfo,
        atSectionIndex sectionIndex: Int,
        for type: NSFetchedResultsChangeType
    ) {
        callback(
            token,
            CDFRC_EVENT_DID_CHANGE_SECTION,
            cdRetain(controller),
            cdRetainContextBound(sectionInfo as AnyObject, in: controller.managedObjectContext),
            UInt64(type.rawValue),
            Int64(sectionIndex),
            -1,
            -1,
            -1
        )
    }

    func controllerDidChangeContent(_ controller: NSFetchedResultsController<NSFetchRequestResult>) {
        callback(token, CDFRC_EVENT_DID_CHANGE_CONTENT, cdRetain(controller), nil, 0, -1, -1, -1, -1)
    }
}

func cdController(_ controllerPtr: UnsafeMutableRawPointer?) throws -> NSFetchedResultsController<NSFetchRequestResult> {
    try cdArg(controllerPtr, "fetched results controller")
}

func cdWithController<T>(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ body: (NSFetchedResultsController<NSFetchRequestResult>, NSManagedObjectContext) throws -> T
) throws -> T {
    let controller = try cdController(controllerPtr)
    let context = controller.managedObjectContext
    return try cdOnContextQueue(context) { try body(controller, context) }
}

@_cdecl("cd_fetched_results_controller_new")
public func cdFetchedResultsControllerNew(
    _ fetchRequestPtr: UnsafeMutableRawPointer?,
    _ managedObjectContextPtr: UnsafeMutableRawPointer?,
    _ sectionNameKeyPath: UnsafePointer<CChar>?,
    _ cacheName: UnsafePointer<CChar>?,
    _ outController: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let fetchRequest: NSFetchRequest<NSFetchRequestResult> = try cdArg(fetchRequestPtr, "fetch request")
        let sectionKeyPath = cdOptionalString(sectionNameKeyPath)
        let cache = cdOptionalString(cacheName)
        let controller = try cdWithContext(managedObjectContextPtr) { context in
            guard let sortDescriptors = fetchRequest.sortDescriptors, !sortDescriptors.isEmpty else {
                throw cdBridgeNSError(
                    code: CDR_INVALID_ARGUMENT,
                    message: "A fetched results controller requires a fetch request with sort descriptors"
                )
            }
            try cdEnsureFetchable(fetchRequest, in: context)
            guard let snapshot = try cdObjCValue({ CDXCopyObject(fetchRequest, $0) }) as? NSFetchRequest<NSFetchRequestResult> else {
                throw cdBridgeNSError(code: CDR_FAILURE, message: "Could not copy the fetch request")
            }
            return try cdObjCValue { CDXNewFetchedResultsController(snapshot, context, sectionKeyPath, cache, $0) }
        }
        outController?.pointee = cdRetain(controller)
    }
}

@_cdecl("cd_fetched_results_controller_perform_fetch")
public func cdFetchedResultsControllerPerformFetch(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        try cdWithController(controllerPtr) { controller, _ in
            try cdObjCCall { CDXPerformFetch(controller, $0) }
        }
    }
}

@_cdecl("cd_fetched_results_controller_set_delegate")
public func cdFetchedResultsControllerSetDelegate(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ token: UnsafeMutableRawPointer?,
    _ callback: @escaping CDFetchedResultsCallback,
    _ release: @escaping CDVoidCallback,
    _ outError: CDOutError
) -> Int32 {
    guard let token else {
        cdWriteError(cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing delegate context"), to: outError)
        return CDR_INVALID_ARGUMENT
    }
    let delegate = CDFetchedResultsDelegate(token: token, callback: callback, release: release)
    return cdStatus(outError) {
        try cdWithController(controllerPtr) { controller, _ in
            objc_setAssociatedObject(controller, cdFetchedResultsDelegateKey, delegate, .OBJC_ASSOCIATION_RETAIN)
            controller.delegate = delegate
        }
    }
}

@_cdecl("cd_fetched_results_controller_remove_delegate")
public func cdFetchedResultsControllerRemoveDelegate(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ token: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let controller = try cdController(controllerPtr)
        let detach = {
            guard let current = objc_getAssociatedObject(controller, cdFetchedResultsDelegateKey) as? CDFetchedResultsDelegate,
                  current.token == token
            else {
                return
            }
            controller.delegate = nil
            objc_setAssociatedObject(controller, cdFetchedResultsDelegateKey, nil, .OBJC_ASSOCIATION_RETAIN)
        }
        let context = controller.managedObjectContext
        if context.concurrencyType == .mainQueueConcurrencyType && !Thread.isMainThread {
            context.perform(detach)
        } else {
            try cdOnContextQueue(context, detach)
        }
    }
}

@_cdecl("cd_fetched_results_controller_get_fetch_request")
public func cdFetchedResultsControllerGetFetchRequest(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let controller = try? cdController(controllerPtr),
          let snapshot = (try? cdObjCValue { CDXCopyObject(controller.fetchRequest, $0) }) as? NSFetchRequest<NSFetchRequestResult>
    else {
        return nil
    }
    return cdRetain(snapshot)
}

@_cdecl("cd_fetched_results_controller_get_managed_object_context")
public func cdFetchedResultsControllerGetManagedObjectContext(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let controller = try? cdController(controllerPtr) else {
        return nil
    }
    return cdRetain(controller.managedObjectContext)
}

@_cdecl("cd_fetched_results_controller_get_section_name_key_path")
public func cdFetchedResultsControllerGetSectionNameKeyPath(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let controller = try? cdController(controllerPtr), let value = controller.sectionNameKeyPath else {
        return nil
    }
    return cdCString(value)
}

@_cdecl("cd_fetched_results_controller_get_cache_name")
public func cdFetchedResultsControllerGetCacheName(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let controller = try? cdController(controllerPtr), let value = controller.cacheName else {
        return nil
    }
    return cdCString(value)
}

@_cdecl("cd_fetched_results_controller_get_fetched_objects")
public func cdFetchedResultsControllerGetFetchedObjects(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ outArray: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        outArray?.pointee = try cdWithController(controllerPtr) { controller, context in
            try controller.fetchedObjects.map { try cdRetainManagedObjects($0, in: context) }
        }
    }
}

@_cdecl("cd_fetched_results_controller_get_sections")
public func cdFetchedResultsControllerGetSections(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ outArray: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        outArray?.pointee = try cdWithController(controllerPtr) { controller, context in
            controller.sections.map { sections in
                cdRetain(sections.map { CDContextBoundBox($0 as AnyObject, context: context) } as NSArray)
            }
        }
    }
}

@_cdecl("cd_fetched_results_controller_get_section_index_titles_json")
public func cdFetchedResultsControllerGetSectionIndexTitlesJSON(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ outJSON: CDOutString,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let titles = try cdWithController(controllerPtr) { controller, _ in controller.sectionIndexTitles }
        outJSON?.pointee = cdCString(try cdEncodeJSON(titles))
    }
}

@_cdecl("cd_fetched_results_controller_object_at_index_path")
public func cdFetchedResultsControllerObjectAtIndexPath(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ section: Int64,
    _ item: Int64,
    _ outObject: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        guard let sectionIndex = Int(exactly: section), let itemIndex = Int(exactly: item),
              sectionIndex >= 0, itemIndex >= 0
        else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Negative fetched results index path")
        }
        outObject?.pointee = try cdWithController(controllerPtr) { controller, context in
            guard let sections = controller.sections, sectionIndex < sections.count else {
                throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Fetched results section out of range")
            }
            guard itemIndex < sections[sectionIndex].numberOfObjects else {
                throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Fetched results item out of range")
            }
            let result = controller.object(at: IndexPath(indexes: [sectionIndex, itemIndex]))
            guard let object = result as? NSManagedObject else {
                throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "The fetched result is not a managed object")
            }
            return cdRetainManagedObject(object, in: context)
        }
    }
}

@_cdecl("cd_fetched_results_controller_index_path_for_object")
public func cdFetchedResultsControllerIndexPathForObject(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ objectPtr: UnsafeMutableRawPointer?,
    _ outSection: UnsafeMutablePointer<Int64>?,
    _ outItem: UnsafeMutablePointer<Int64>?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let box = try cdManagedObjectBox(objectPtr)
        let (section, item) = try cdWithController(controllerPtr) { controller, context -> (Int64, Int64) in
            cdUnboundObjectLock.lock()
            let bound = box.boundContextLocked
            cdUnboundObjectLock.unlock()
            guard bound === context, box.managedObject.managedObjectContext === context,
                  let indexPath = controller.indexPath(forObject: box.managedObject), indexPath.count >= 2
            else {
                return (-1, -1)
            }
            return (Int64(indexPath[0]), Int64(indexPath[1]))
        }
        outSection?.pointee = section
        outItem?.pointee = item
    }
}

@_cdecl("cd_fetched_results_controller_delete_cache_with_name")
public func cdFetchedResultsControllerDeleteCacheWithName(_ name: UnsafePointer<CChar>?) {
    NSFetchedResultsController<NSFetchRequestResult>.deleteCache(withName: cdOptionalString(name))
}

func cdWithSectionInfo<T>(
    _ sectionInfoPtr: UnsafeMutableRawPointer?,
    _ body: (NSFetchedResultsSectionInfo, NSManagedObjectContext) throws -> T
) throws -> T {
    let (object, context): (NSObject, NSManagedObjectContext) = try cdContextBoundArg(sectionInfoPtr, "fetched results section")
    guard let sectionInfo = object as? NSFetchedResultsSectionInfo else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Expected fetched results section")
    }
    return try cdOnContextQueue(context) { try body(sectionInfo, context) }
}

@_cdecl("cd_fetched_results_section_info_get_name")
public func cdFetchedResultsSectionInfoGetName(
    _ sectionInfoPtr: UnsafeMutableRawPointer?,
    _ outName: CDOutString,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let name = try cdWithSectionInfo(sectionInfoPtr) { section, _ in section.name }
        outName?.pointee = cdCString(name)
    }
}

@_cdecl("cd_fetched_results_section_info_get_index_title")
public func cdFetchedResultsSectionInfoGetIndexTitle(
    _ sectionInfoPtr: UnsafeMutableRawPointer?,
    _ outTitle: CDOutString,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let title = try cdWithSectionInfo(sectionInfoPtr) { section, _ in section.indexTitle }
        outTitle?.pointee = title.flatMap { cdCString($0) }
    }
}

@_cdecl("cd_fetched_results_section_info_get_number_of_objects")
public func cdFetchedResultsSectionInfoGetNumberOfObjects(
    _ sectionInfoPtr: UnsafeMutableRawPointer?,
    _ outCount: UnsafeMutablePointer<UInt64>?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let count = try cdWithSectionInfo(sectionInfoPtr) { section, _ in section.numberOfObjects }
        outCount?.pointee = UInt64(clamping: count)
    }
}

@_cdecl("cd_fetched_results_section_info_get_objects")
public func cdFetchedResultsSectionInfoGetObjects(
    _ sectionInfoPtr: UnsafeMutableRawPointer?,
    _ outArray: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        outArray?.pointee = try cdWithSectionInfo(sectionInfoPtr) { section, context in
            try section.objects.map { try cdRetainManagedObjects($0, in: context) }
        }
    }
}
