#import <CoreData/CoreData.h>
#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

typedef void (^CDXStoreLoadHandler)(NSPersistentStoreDescription *description, NSError *_Nullable error);

BOOL CDXSetValueForKey(id object, id _Nullable value, NSString *key, NSError *_Nullable *_Nullable error);

id _Nullable CDXCopyObject(id object, NSError *_Nullable *_Nullable error);

BOOL CDXValueForKey(
    id object,
    NSString *key,
    id _Nullable __autoreleasing *_Nonnull outValue,
    NSError *_Nullable *_Nullable error
);

NSPredicate *_Nullable CDXPredicateWithFormat(
    NSString *format,
    NSArray *_Nullable arguments,
    NSError *_Nullable *_Nullable error
);

NSPredicate *_Nullable CDXPredicateWithSubstitutionVariables(
    NSPredicate *predicate,
    NSDictionary<NSString *, id> *variables,
    NSError *_Nullable *_Nullable error
);

BOOL CDXEvaluatePredicate(
    NSPredicate *predicate,
    id _Nullable object,
    NSDictionary<NSString *, id> *_Nullable variables,
    BOOL *outResult,
    NSError *_Nullable *_Nullable error
);

BOOL CDXSetValidationPredicates(
    NSPropertyDescription *property,
    NSArray<NSPredicate *> *predicates,
    NSArray<NSString *> *warnings,
    NSError *_Nullable *_Nullable error
);

NSArray *_Nullable CDXExecuteFetchRequest(
    NSManagedObjectContext *context,
    NSFetchRequest *request,
    NSError *_Nullable *_Nullable error
);

BOOL CDXCountForFetchRequest(
    NSManagedObjectContext *context,
    NSFetchRequest *request,
    NSUInteger *outCount,
    NSError *_Nullable *_Nullable error
);

NSPersistentStoreResult *_Nullable CDXExecuteRequest(
    NSManagedObjectContext *context,
    NSPersistentStoreRequest *request,
    NSError *_Nullable *_Nullable error
);

BOOL CDXSave(NSManagedObjectContext *context, NSError *_Nullable *_Nullable error);

BOOL CDXInsertObject(NSManagedObjectContext *context, NSManagedObject *object, NSError *_Nullable *_Nullable error);

BOOL CDXDeleteObject(NSManagedObjectContext *context, NSManagedObject *object, NSError *_Nullable *_Nullable error);

BOOL CDXRefreshObject(
    NSManagedObjectContext *context,
    NSManagedObject *object,
    BOOL mergeChanges,
    NSError *_Nullable *_Nullable error
);

BOOL CDXObtainPermanentIDs(
    NSManagedObjectContext *context,
    NSArray<NSManagedObject *> *objects,
    NSError *_Nullable *_Nullable error
);

NSManagedObject *_Nullable CDXObjectWithID(
    NSManagedObjectContext *context,
    NSManagedObjectID *objectID,
    NSError *_Nullable *_Nullable error
);

NSManagedObject *_Nullable CDXExistingObjectWithID(
    NSManagedObjectContext *context,
    NSManagedObjectID *objectID,
    NSError *_Nullable *_Nullable error
);

NSManagedObject *_Nullable CDXNewManagedObject(
    NSEntityDescription *entity,
    NSManagedObjectContext *_Nullable context,
    NSError *_Nullable *_Nullable error
);

NSManagedObject *_Nullable CDXInsertNewObjectForEntityName(
    NSString *entityName,
    NSManagedObjectContext *context,
    NSError *_Nullable *_Nullable error
);

BOOL CDXHasFaultForRelationshipNamed(
    NSManagedObject *object,
    NSString *name,
    BOOL *outResult,
    NSError *_Nullable *_Nullable error
);

NSArray<NSManagedObjectID *> *_Nullable CDXObjectIDsForRelationshipNamed(
    NSManagedObject *object,
    NSString *name,
    NSError *_Nullable *_Nullable error
);

BOOL CDXMergeChangesFromContextDidSave(
    NSManagedObjectContext *context,
    NSNotification *notification,
    NSError *_Nullable *_Nullable error
);

NSFetchedResultsController *_Nullable CDXNewFetchedResultsController(
    NSFetchRequest *request,
    NSManagedObjectContext *context,
    NSString *_Nullable sectionNameKeyPath,
    NSString *_Nullable cacheName,
    NSError *_Nullable *_Nullable error
);

BOOL CDXPerformFetch(NSFetchedResultsController *controller, NSError *_Nullable *_Nullable error);

NSPersistentStore *_Nullable CDXAddPersistentStore(
    NSPersistentStoreCoordinator *coordinator,
    NSString *storeType,
    NSString *_Nullable configuration,
    NSURL *_Nullable url,
    NSDictionary *_Nullable options,
    NSError *_Nullable *_Nullable error
);

BOOL CDXAddPersistentStoreWithDescription(
    NSPersistentStoreCoordinator *coordinator,
    NSPersistentStoreDescription *description,
    CDXStoreLoadHandler handler,
    NSError *_Nullable *_Nullable error
);

BOOL CDXLoadPersistentStores(
    NSPersistentContainer *container,
    CDXStoreLoadHandler handler,
    NSError *_Nullable *_Nullable error
);

NS_ASSUME_NONNULL_END
