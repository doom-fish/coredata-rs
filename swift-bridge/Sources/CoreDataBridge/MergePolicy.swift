import CoreData
import Foundation

@_cdecl("cd_merge_policy_new")
public func cdMergePolicyNew(
    _ mergeTypeRawValue: UInt64,
    _ outPolicy: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let mergeType = NSMergePolicyType(rawValue: UInt(mergeTypeRawValue)) else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Invalid merge policy type: \(mergeTypeRawValue)")
        }
        outPolicy?.pointee = cdRetain(NSMergePolicy(merge: mergeType))
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}

@_cdecl("cd_merge_policy_error_policy")
public func cdMergePolicyErrorPolicy() -> UnsafeMutableRawPointer? {
    cdRetain(NSErrorMergePolicy as! NSMergePolicy)
}

@_cdecl("cd_merge_policy_rollback_policy")
public func cdMergePolicyRollbackPolicy() -> UnsafeMutableRawPointer? {
    cdRetain(NSRollbackMergePolicy as! NSMergePolicy)
}

@_cdecl("cd_merge_policy_overwrite_policy")
public func cdMergePolicyOverwritePolicy() -> UnsafeMutableRawPointer? {
    cdRetain(NSOverwriteMergePolicy as! NSMergePolicy)
}

@_cdecl("cd_merge_policy_merge_by_property_object_trump_policy")
public func cdMergePolicyMergeByPropertyObjectTrumpPolicy() -> UnsafeMutableRawPointer? {
    cdRetain(NSMergeByPropertyObjectTrumpMergePolicy as! NSMergePolicy)
}

@_cdecl("cd_merge_policy_merge_by_property_store_trump_policy")
public func cdMergePolicyMergeByPropertyStoreTrumpPolicy() -> UnsafeMutableRawPointer? {
    cdRetain(NSMergeByPropertyStoreTrumpMergePolicy as! NSMergePolicy)
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
public func cdManagedObjectContextGetMergePolicy(_ contextPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let contextPtr else {
        return nil
    }
    let context: NSManagedObjectContext = cdBorrow(contextPtr)
    return cdRetain(context.mergePolicy as! NSMergePolicy)
}

@_cdecl("cd_managed_object_context_set_merge_policy")
public func cdManagedObjectContextSetMergePolicy(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ mergePolicyPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let contextPtr, let mergePolicyPtr else {
            throw cdBridgeNSError(code: CDR_INVALID_ARGUMENT, message: "Missing managed object context or merge policy")
        }
        let context: NSManagedObjectContext = cdBorrow(contextPtr)
        let mergePolicy: NSMergePolicy = cdBorrow(mergePolicyPtr)
        context.mergePolicy = mergePolicy
        return CDR_OK
    } catch let error as NSError {
        cdWriteError(error, to: outError)
        return Int32(error.code)
    }
}
