import CoreData
import Foundation

@_cdecl("cd_query_generation_token_current")
public func cdQueryGenerationTokenCurrent() -> UnsafeMutableRawPointer? {
    cdRetain(NSQueryGenerationToken.current)
}

@_cdecl("cd_managed_object_context_get_query_generation_token")
public func cdManagedObjectContextGetQueryGenerationToken(_ contextPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let contextPtr else {
        return nil
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    return context.queryGenerationToken.map(cdRetain)
}

@_cdecl("cd_managed_object_context_set_query_generation_from_token")
public func cdManagedObjectContextSetQueryGenerationFromToken(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let token = tokenPtr.map { cdBorrow($0) as NSQueryGenerationToken }
        try context.setQueryGenerationFrom(token)
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
