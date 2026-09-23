import CoreData
import CoreDataObjCBridge
import Foundation

let cdUnboundObjectLock = NSRecursiveLock()

final class CDManagedObjectBox: CDObjectBox {
    let managedObject: NSManagedObject
    private var bound: NSManagedObjectContext?

    init(_ managedObject: NSManagedObject, context: NSManagedObjectContext?) {
        self.managedObject = managedObject
        self.bound = context
        super.init(managedObject)
    }

    var boundContextLocked: NSManagedObjectContext? {
        bound
    }

    func bindLocked(_ context: NSManagedObjectContext) {
        bound = context
    }
}

final class CDContextBoundBox: CDObjectBox {
    let context: NSManagedObjectContext

    init(_ object: AnyObject, context: NSManagedObjectContext) {
        self.context = context
        super.init(object)
    }
}

func cdWrongThreadError() -> NSError {
    cdBridgeNSError(
        code: CDR_WRONG_THREAD,
        message: "A main-queue managed object context and its objects can only be used on the main thread"
    )
}

func cdInvalidatedError() -> NSError {
    cdBridgeNSError(
        code: CDR_INVALIDATED,
        message: "The managed object is no longer registered with its context (reset, rolled back, or deleted and saved)"
    )
}

func cdEnsureQueueContext(_ context: NSManagedObjectContext) throws {
    switch context.concurrencyType {
    case .privateQueueConcurrencyType, .mainQueueConcurrencyType:
        return
    default:
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "The managed object context was not created with a private or main queue concurrency type"
        )
    }
}

func cdOnContextQueue<T>(_ context: NSManagedObjectContext, _ body: () throws -> T) throws -> T {
    try cdEnsureQueueContext(context)
    if context.concurrencyType == .mainQueueConcurrencyType && !Thread.isMainThread {
        throw cdWrongThreadError()
    }
    return try context.performAndWait(body)
}

func cdWithContext<T>(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ body: (NSManagedObjectContext) throws -> T
) throws -> T {
    let context: NSManagedObjectContext = try cdArg(contextPtr, "managed object context")
    return try cdOnContextQueue(context) { try body(context) }
}

func cdManagedObjectBox(_ objectPtr: UnsafeMutableRawPointer?) throws -> CDManagedObjectBox {
    guard let objectPtr else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object")
    }
    guard let box = cdBorrowBox(objectPtr) as? CDManagedObjectBox else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Expected a managed object handle")
    }
    return box
}

func cdEnsureRegistered(_ object: NSManagedObject, in context: NSManagedObjectContext) throws {
    guard object.managedObjectContext === context else {
        throw cdInvalidatedError()
    }
}

func cdWithManagedObject<T>(
    _ objectPtr: UnsafeMutableRawPointer?,
    requireRegistered: Bool = true,
    _ body: (NSManagedObject, NSManagedObjectContext?) throws -> T
) throws -> T {
    let box = try cdManagedObjectBox(objectPtr)
    cdUnboundObjectLock.lock()
    if let context = box.boundContextLocked {
        cdUnboundObjectLock.unlock()
        return try cdOnContextQueue(context) {
            if requireRegistered {
                try cdEnsureRegistered(box.managedObject, in: context)
            }
            return try body(box.managedObject, context)
        }
    }
    defer { cdUnboundObjectLock.unlock() }
    return try body(box.managedObject, nil)
}

func cdObjectRegistered(
    _ objectPtr: UnsafeMutableRawPointer?,
    in context: NSManagedObjectContext
) throws -> NSManagedObject {
    let box = try cdManagedObjectBox(objectPtr)
    cdUnboundObjectLock.lock()
    let bound = box.boundContextLocked
    cdUnboundObjectLock.unlock()
    guard let bound else {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "The managed object has not been inserted into this managed object context"
        )
    }
    guard bound === context else {
        throw cdBridgeNSError(
            code: CDR_INVALID_ARGUMENT,
            message: "The managed object belongs to a different managed object context"
        )
    }
    try cdEnsureRegistered(box.managedObject, in: context)
    return box.managedObject
}

func cdRetainManagedObject(_ object: NSManagedObject, in context: NSManagedObjectContext) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(CDManagedObjectBox(object, context: context)).toOpaque()
}

func cdManagedObjectBoxes(_ objects: [Any], in context: NSManagedObjectContext) throws -> [CDManagedObjectBox] {
    try objects.map { element in
        guard let object = element as? NSManagedObject else {
            throw cdBridgeNSError(
                code: CDR_INVALID_ARGUMENT,
                message: "Expected managed objects but the result contains \(type(of: element)); use a managed-object result type"
            )
        }
        return CDManagedObjectBox(object, context: context)
    }
}

func cdRetainManagedObjects(_ objects: [Any], in context: NSManagedObjectContext) throws -> UnsafeMutableRawPointer {
    cdRetain(try cdManagedObjectBoxes(objects, in: context) as NSArray)
}

func cdContextBoundArg<T: AnyObject>(
    _ ptr: UnsafeMutableRawPointer?,
    _ what: String
) throws -> (T, NSManagedObjectContext) {
    guard let ptr else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing \(what)")
    }
    guard let box = cdBorrowBox(ptr) as? CDContextBoundBox, let object = box.object as? T else {
        throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Expected \(what)")
    }
    return (object, box.context)
}

func cdRetainContextBound(_ object: AnyObject, in context: NSManagedObjectContext) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(CDContextBoundBox(object, context: context)).toOpaque()
}
