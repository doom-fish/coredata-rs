import CoreData
import CoreDataObjCBridge
import Foundation
import ObjectiveC

private let cdDestinationEntityNameKey = UnsafeMutableRawPointer.allocate(byteCount: 1, alignment: 1)
private let cdInverseRelationshipNameKey = UnsafeMutableRawPointer.allocate(byteCount: 1, alignment: 1)
private let cdEventRequestSourceKey = UnsafeMutableRawPointer.allocate(byteCount: 1, alignment: 1)

func cdDestinationEntityName(_ relationship: NSRelationshipDescription) -> String? {
    if let name = objc_getAssociatedObject(relationship, cdDestinationEntityNameKey) as? String {
        return name
    }
    return relationship.destinationEntity?.name
}

func cdInverseRelationshipName(_ relationship: NSRelationshipDescription) -> String? {
    if let name = objc_getAssociatedObject(relationship, cdInverseRelationshipNameKey) as? String {
        return name
    }
    return relationship.inverseRelationship?.name
}

func cdSetRelationshipNames(_ relationship: NSRelationshipDescription, destination: String?, inverse: String?) {
    objc_setAssociatedObject(relationship, cdDestinationEntityNameKey, destination.map { $0 as NSString }, .OBJC_ASSOCIATION_COPY)
    objc_setAssociatedObject(relationship, cdInverseRelationshipNameKey, inverse.map { $0 as NSString }, .OBJC_ASSOCIATION_COPY)
}

func cdOwnRelationships(_ entity: NSEntityDescription) -> [NSRelationshipDescription] {
    entity.properties.compactMap { $0 as? NSRelationshipDescription }
}

func cdTransferRelationshipNames(from source: NSEntityDescription, to copy: NSEntityDescription) {
    var copied: [String: NSRelationshipDescription] = [:]
    for relationship in cdOwnRelationships(copy) {
        copied[relationship.name] = relationship
    }
    for relationship in cdOwnRelationships(source) {
        guard let target = copied[relationship.name] else {
            continue
        }
        cdSetRelationshipNames(
            target,
            destination: cdDestinationEntityName(relationship),
            inverse: cdInverseRelationshipName(relationship)
        )
    }
    var copiedSubentities: [String: NSEntityDescription] = [:]
    for subentity in copy.subentities {
        if let name = subentity.name {
            copiedSubentities[name] = subentity
        }
    }
    for subentity in source.subentities {
        if let name = subentity.name, let target = copiedSubentities[name] {
            cdTransferRelationshipNames(from: subentity, to: target)
        }
    }
}

func cdCopyEntity(_ entity: NSEntityDescription) throws -> NSEntityDescription {
    guard let copy = try cdObjCValue({ CDXCopyObject(entity, $0) }) as? NSEntityDescription else {
        throw cdBridgeNSError(code: CDR_FAILURE, message: "Could not copy the entity description")
    }
    cdTransferRelationshipNames(from: entity, to: copy)
    return copy
}

func cdCopyModel(_ model: NSManagedObjectModel) throws -> NSManagedObjectModel {
    guard let copy = try cdObjCValue({ CDXCopyObject(model, $0) }) as? NSManagedObjectModel else {
        throw cdBridgeNSError(code: CDR_FAILURE, message: "Could not copy the managed object model")
    }
    let copiedEntities = copy.entitiesByName
    for entity in model.entities {
        if let name = entity.name, let target = copiedEntities[name] {
            cdTransferRelationshipNames(from: entity, to: target)
        }
    }
    return copy
}

func cdCopyFetchedProperty(_ property: NSFetchedPropertyDescription) throws -> NSFetchedPropertyDescription {
    let copy = NSFetchedPropertyDescription()
    copy.name = property.name
    copy.isOptional = property.isOptional
    copy.isTransient = property.isTransient
    copy.userInfo = property.userInfo
    copy.versionHashModifier = property.versionHashModifier
    copy.renamingIdentifier = property.renamingIdentifier
    copy.isIndexedBySpotlight = property.isIndexedBySpotlight
    let warnings = property.validationWarnings.compactMap { $0 as? String }
    if !property.validationPredicates.isEmpty {
        try cdObjCCall { CDXSetValidationPredicates(copy, property.validationPredicates, warnings, $0) }
    }
    copy.fetchRequest = property.fetchRequest.map { cdCopyFetchRequest($0) }
    return copy
}

func cdCopyProperty(_ property: NSPropertyDescription) throws -> NSPropertyDescription {
    if let fetched = property as? NSFetchedPropertyDescription {
        return try cdCopyFetchedProperty(fetched)
    }
    guard let copy = try cdObjCValue({ CDXCopyObject(property, $0) }) as? NSPropertyDescription else {
        throw cdBridgeNSError(code: CDR_FAILURE, message: "Could not copy the property description")
    }
    if let relationship = property as? NSRelationshipDescription, let target = copy as? NSRelationshipDescription {
        cdSetRelationshipNames(
            target,
            destination: cdDestinationEntityName(relationship),
            inverse: cdInverseRelationshipName(relationship)
        )
    }
    return copy
}

func cdCopyIndexElements(_ elements: [NSFetchIndexElementDescription]) throws -> [NSFetchIndexElementDescription] {
    try elements.map { element in
        guard let copy = try cdObjCValue({ CDXCopyObject(element, $0) }) as? NSFetchIndexElementDescription else {
            throw cdBridgeNSError(code: CDR_FAILURE, message: "Could not copy the fetch index element")
        }
        return copy
    }
}

func cdResolveRelationships(in model: NSManagedObjectModel) throws {
    let entitiesByName = model.entitiesByName
    for entity in model.entities {
        for relationship in cdOwnRelationships(entity) {
            let destination = cdDestinationEntityName(relationship).flatMap { entitiesByName[$0] }
            try cdTrySetValue(destination, forKey: "destinationEntity", on: relationship)
        }
    }
    for entity in model.entities {
        for relationship in cdOwnRelationships(entity) {
            var inverse: NSRelationshipDescription?
            if let destination = relationship.destinationEntity, let name = cdInverseRelationshipName(relationship) {
                inverse = destination.relationshipsByName[name]
            }
            try cdTrySetValue(inverse, forKey: "inverseRelationship", on: relationship)
        }
    }
}

func cdAddEntity(_ entity: NSEntityDescription, to model: NSManagedObjectModel) throws {
    guard let name = entity.name, !name.isEmpty else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "An entity needs a name before it can be added to a model")
    }
    if let existing = model.entitiesByName[name], existing.superentity != nil || !existing.subentities.isEmpty {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "Entity '\(name)' is part of an entity hierarchy and cannot be replaced"
        )
    }
    let copy = try cdCopyEntity(entity)
    for existing in model.entities {
        for relationship in cdOwnRelationships(existing) {
            cdSetRelationshipNames(
                relationship,
                destination: cdDestinationEntityName(relationship),
                inverse: cdInverseRelationshipName(relationship)
            )
        }
    }
    for existing in model.entities {
        for relationship in cdOwnRelationships(existing) {
            try cdTrySetValue(nil, forKey: "inverseRelationship", on: relationship)
            try cdTrySetValue(nil, forKey: "destinationEntity", on: relationship)
        }
    }
    var entities = model.entities.filter { $0.name != name }
    entities.append(copy)
    try cdTrySetValue(entities, forKey: "entities", on: model)
    try cdResolveRelationships(in: model)
}

func cdHandoffModel(_ model: NSManagedObjectModel) throws -> NSManagedObjectModel {
    for entity in model.entities {
        for relationship in cdOwnRelationships(entity) where relationship.destinationEntity == nil {
            if let destination = cdDestinationEntityName(relationship) {
                throw cdBridgeNSError(
                    code: CDR_INVALID_ARGUMENT,
                    message: "Relationship '\(entity.name ?? "").\(relationship.name)' names destination entity '\(destination)', which is not in the model"
                )
            }
        }
    }
    return try cdCopyModel(model)
}

func cdCopyFetchRequest<T>(_ request: NSFetchRequest<T>) -> NSFetchRequest<NSFetchRequestResult> {
    let copy: NSFetchRequest<NSFetchRequestResult>
    if let name = request.entityName {
        copy = NSFetchRequest(entityName: name)
    } else {
        copy = NSFetchRequest()
    }
    copy.predicate = request.predicate
    copy.sortDescriptors = request.sortDescriptors
    copy.fetchLimit = request.fetchLimit
    copy.fetchOffset = request.fetchOffset
    copy.fetchBatchSize = request.fetchBatchSize
    copy.resultType = request.resultType
    copy.includesSubentities = request.includesSubentities
    copy.includesPropertyValues = request.includesPropertyValues
    copy.returnsObjectsAsFaults = request.returnsObjectsAsFaults
    copy.relationshipKeyPathsForPrefetching = request.relationshipKeyPathsForPrefetching
    copy.includesPendingChanges = request.includesPendingChanges
    copy.returnsDistinctResults = request.returnsDistinctResults
    copy.shouldRefreshRefetchedObjects = request.shouldRefreshRefetchedObjects
    copy.havingPredicate = request.havingPredicate
    copy.affectedStores = request.affectedStores
    return copy
}

func cdCopyAsynchronousFetchRequest(
    _ request: NSAsynchronousFetchRequest<NSFetchRequestResult>
) -> NSAsynchronousFetchRequest<NSFetchRequestResult> {
    let copy = NSAsynchronousFetchRequest(fetchRequest: cdCopyFetchRequest(request.fetchRequest), completionBlock: nil)
    copy.estimatedResultCount = request.estimatedResultCount
    copy.affectedStores = request.affectedStores
    return copy
}

func cdSetEventRequestSource(_ request: NSPersistentCloudKitContainerEventRequest, _ source: AnyObject) {
    objc_setAssociatedObject(request, cdEventRequestSourceKey, source, .OBJC_ASSOCIATION_RETAIN)
}

func cdCopyEventRequest(_ request: NSPersistentCloudKitContainerEventRequest) throws -> NSPersistentCloudKitContainerEventRequest {
    let source = objc_getAssociatedObject(request, cdEventRequestSourceKey)
    let copy: NSPersistentCloudKitContainerEventRequest
    if let date = source as? Date {
        copy = NSPersistentCloudKitContainerEventRequest.fetchEvents(after: date)
    } else if let event = source as? NSPersistentCloudKitContainer.Event {
        copy = NSPersistentCloudKitContainerEventRequest.fetchEvents(after: event)
    } else if source is NSNull {
        copy = NSPersistentCloudKitContainerEventRequest.fetchEvents(after: nil as NSPersistentCloudKitContainer.Event?)
    } else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "This CloudKit event request cannot be copied")
    }
    if let source {
        cdSetEventRequestSource(copy, source as AnyObject)
    }
    copy.resultType = request.resultType
    copy.affectedStores = request.affectedStores
    return copy
}

func cdCopyRequest(_ request: NSPersistentStoreRequest) throws -> NSPersistentStoreRequest {
    let copy: NSPersistentStoreRequest
    switch request {
    case let fetch as NSFetchRequest<NSFetchRequestResult>:
        return cdCopyFetchRequest(fetch)
    case let asynchronous as NSAsynchronousFetchRequest<NSFetchRequestResult>:
        return cdCopyAsynchronousFetchRequest(asynchronous)
    case let delete as NSBatchDeleteRequest:
        let snapshot = NSBatchDeleteRequest(fetchRequest: cdCopyFetchRequest(delete.fetchRequest))
        snapshot.resultType = delete.resultType
        copy = snapshot
    case let insert as NSBatchInsertRequest:
        let snapshot = NSBatchInsertRequest(entityName: insert.entityName, objects: insert.objectsToInsert ?? [])
        snapshot.resultType = insert.resultType
        copy = snapshot
    case let update as NSBatchUpdateRequest:
        let snapshot = NSBatchUpdateRequest(entityName: update.entityName)
        snapshot.predicate = update.predicate
        snapshot.includesSubentities = update.includesSubentities
        snapshot.resultType = update.resultType
        snapshot.propertiesToUpdate = update.propertiesToUpdate
        copy = snapshot
    case let history as NSPersistentHistoryChangeRequest:
        guard let snapshot = try cdObjCValue({ CDXCopyObject(history, $0) }) as? NSPersistentHistoryChangeRequest else {
            throw cdBridgeNSError(code: CDR_FAILURE, message: "Could not copy the persistent history request")
        }
        copy = snapshot
    case let event as NSPersistentCloudKitContainerEventRequest:
        return try cdCopyEventRequest(event)
    case let save as NSSaveChangesRequest:
        return cdCopySaveChangesRequest(save)
    default:
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Unsupported persistent store request \(type(of: request))")
    }
    copy.affectedStores = request.affectedStores
    return copy
}

func cdCopyStoreDescription(_ description: NSPersistentStoreDescription) throws -> NSPersistentStoreDescription {
    guard let copy = try cdObjCValue({ CDXCopyObject(description, $0) }) as? NSPersistentStoreDescription else {
        throw cdBridgeNSError(code: CDR_FAILURE, message: "Could not copy the persistent store description")
    }
    if #available(macOS 14.0, *),
       let manager = description.options[NSPersistentStoreStagedMigrationManagerOptionKey] as? NSStagedMigrationManager {
        copy.setOption(NSStagedMigrationManager(manager.stages), forKey: NSPersistentStoreStagedMigrationManagerOptionKey)
    }
    return copy
}

func cdCopyObject(_ object: AnyObject) throws -> AnyObject {
    switch object {
    case let model as NSManagedObjectModel:
        return try cdCopyModel(model)
    case let entity as NSEntityDescription:
        return try cdCopyEntity(entity)
    case let property as NSPropertyDescription:
        return try cdCopyProperty(property)
    case let request as NSPersistentStoreRequest:
        return try cdCopyRequest(request)
    case let description as NSPersistentStoreDescription:
        return try cdCopyStoreDescription(description)
    case is NSFetchIndexDescription, is NSFetchIndexElementDescription:
        return try cdObjCValue { CDXCopyObject(object, $0) } as AnyObject
    default:
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "\(type(of: object)) cannot be copied")
    }
}

@_cdecl("cd_object_copy")
public func cdObjectCopy(_ objectPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let objectPtr, let copy = try? cdCopyObject(cdBorrowBox(objectPtr).object) else {
        return nil
    }
    return cdRetain(copy)
}
