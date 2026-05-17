import CoreData
import Foundation

@_cdecl("cd_coredata_version_number")
public func cdCoreDataVersionNumber() -> Double {
    NSCoreDataVersionNumber
}
