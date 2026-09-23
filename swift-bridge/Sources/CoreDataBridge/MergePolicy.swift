import CoreData
import Foundation

@_cdecl("cd_merge_policy_new")
public func cdMergePolicyNew(
    _ mergeTypeRawValue: UInt64,
    _ outPolicy: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let mergeRaw = try cdKnownRawValue(mergeTypeRawValue, in: [0, 1, 2, 3, 4], "merge policy type")
        guard let mergeType = NSMergePolicyType(rawValue: UInt(mergeRaw)) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid merge policy type: \(mergeTypeRawValue)")
        }
        outPolicy?.pointee = cdRetain(NSMergePolicy(merge: mergeType))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return cdErrorStatus(error)
    }
}

@_cdecl("cd_merge_policy_error_policy")
public func cdMergePolicyErrorPolicy() -> UnsafeMutableRawPointer? {
    (NSErrorMergePolicy as? NSMergePolicy).map(cdRetain)
}

@_cdecl("cd_merge_policy_rollback_policy")
public func cdMergePolicyRollbackPolicy() -> UnsafeMutableRawPointer? {
    (NSRollbackMergePolicy as? NSMergePolicy).map(cdRetain)
}

@_cdecl("cd_merge_policy_overwrite_policy")
public func cdMergePolicyOverwritePolicy() -> UnsafeMutableRawPointer? {
    (NSOverwriteMergePolicy as? NSMergePolicy).map(cdRetain)
}

@_cdecl("cd_merge_policy_merge_by_property_object_trump_policy")
public func cdMergePolicyMergeByPropertyObjectTrumpPolicy() -> UnsafeMutableRawPointer? {
    (NSMergeByPropertyObjectTrumpMergePolicy as? NSMergePolicy).map(cdRetain)
}

@_cdecl("cd_merge_policy_merge_by_property_store_trump_policy")
public func cdMergePolicyMergeByPropertyStoreTrumpPolicy() -> UnsafeMutableRawPointer? {
    (NSMergeByPropertyStoreTrumpMergePolicy as? NSMergePolicy).map(cdRetain)
}

@_cdecl("cd_merge_policy_get_merge_type")
public func cdMergePolicyGetMergeType(_ policyPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let policyPtr else {
        return 0
    }
    let policy: NSMergePolicy = cdBorrow(policyPtr)
    return UInt64(policy.mergeType.rawValue)
}

@_cdecl("cd_managed_object_context_get_merge_policy")
public func cdManagedObjectContextGetMergePolicy(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outPolicy: CDOutObject,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let policy = try cdWithContext(contextPtr) { $0.mergePolicy as? NSMergePolicy }
        guard let policy else {
            throw cdBridgeNSError(code: CDR_FAILURE, message: "The context merge policy is not an NSMergePolicy")
        }
        outPolicy?.pointee = cdRetain(policy)
    }
}

@_cdecl("cd_managed_object_context_set_merge_policy")
public func cdManagedObjectContextSetMergePolicy(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ mergePolicyPtr: UnsafeMutableRawPointer?,
    _ outError: CDOutError
) -> Int32 {
    cdStatus(outError) {
        let mergePolicy: NSMergePolicy = try cdArg(mergePolicyPtr, "merge policy")
        try cdWithContext(contextPtr) { $0.mergePolicy = mergePolicy }
    }
}
