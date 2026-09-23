import CoreData
import Foundation

@_cdecl("cd_query_generation_token_current")
public func cdQueryGenerationTokenCurrent() -> UnsafeMutableRawPointer? {
    cdRetain(NSQueryGenerationToken.current)
}

@_cdecl("cd_managed_object_context_get_query_generation_token")
public func cdManagedObjectContextGetQueryGenerationToken(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outToken: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let token = try cdWithContext(contextPtr) { $0.queryGenerationToken }
        outToken?.pointee = token.map(cdRetain)
    }
}

@_cdecl("cd_managed_object_context_set_query_generation_from_token")
public func cdManagedObjectContextSetQueryGenerationFromToken(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let token: NSQueryGenerationToken? = try tokenPtr.map { try cdArg($0, "query generation token") }
        try cdWithContext(contextPtr) { try $0.setQueryGenerationFrom(token) }
    }
}
