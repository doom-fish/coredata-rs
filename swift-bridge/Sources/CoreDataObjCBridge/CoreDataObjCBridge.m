#import "CoreDataObjCBridge.h"

static NSString *const CDXErrorDomain = @"CoreDataBridge";
static const NSInteger CDXExceptionCode = -5;

static void CDXAssignException(NSException *exception, NSError *_Nullable *_Nullable error) {
    if (error == NULL) {
        return;
    }
    NSString *reason = exception.reason;
    NSString *message = reason.length > 0
        ? [NSString stringWithFormat:@"%@: %@", exception.name, reason]
        : exception.name;
    *error = [NSError errorWithDomain:CDXErrorDomain
                                 code:CDXExceptionCode
                             userInfo:@{
                                 NSLocalizedDescriptionKey: message,
                                 @"NSExceptionName": exception.name,
                             }];
}

BOOL CDXSetValueForKey(id object, id _Nullable value, NSString *key, NSError *_Nullable *_Nullable error) {
    @try {
        [object setValue:value forKey:key];
        return YES;
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

id _Nullable CDXCopyObject(id object, NSError *_Nullable *_Nullable error) {
    @try {
        return [object copy];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return nil;
    }
}

BOOL CDXValueForKey(
    id object,
    NSString *key,
    id _Nullable __autoreleasing *_Nonnull outValue,
    NSError *_Nullable *_Nullable error
) {
    @try {
        *outValue = [object valueForKey:key];
        return YES;
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

NSPredicate *_Nullable CDXPredicateWithFormat(
    NSString *format,
    NSArray *_Nullable arguments,
    NSError *_Nullable *_Nullable error
) {
    @try {
        return [NSPredicate predicateWithFormat:format argumentArray:arguments];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return nil;
    }
}

NSPredicate *_Nullable CDXPredicateWithSubstitutionVariables(
    NSPredicate *predicate,
    NSDictionary<NSString *, id> *variables,
    NSError *_Nullable *_Nullable error
) {
    @try {
        return [predicate predicateWithSubstitutionVariables:variables];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return nil;
    }
}

BOOL CDXEvaluatePredicate(
    NSPredicate *predicate,
    id _Nullable object,
    NSDictionary<NSString *, id> *_Nullable variables,
    BOOL *outResult,
    NSError *_Nullable *_Nullable error
) {
    @try {
        *outResult = variables != nil
            ? [predicate evaluateWithObject:object substitutionVariables:variables]
            : [predicate evaluateWithObject:object];
        return YES;
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

BOOL CDXSetValidationPredicates(
    NSPropertyDescription *property,
    NSArray<NSPredicate *> *predicates,
    NSArray<NSString *> *warnings,
    NSError *_Nullable *_Nullable error
) {
    @try {
        [property setValidationPredicates:predicates withValidationWarnings:warnings];
        return YES;
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

NSArray *_Nullable CDXExecuteFetchRequest(
    NSManagedObjectContext *context,
    NSFetchRequest *request,
    NSError *_Nullable *_Nullable error
) {
    @try {
        return [context executeFetchRequest:request error:error];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return nil;
    }
}

BOOL CDXCountForFetchRequest(
    NSManagedObjectContext *context,
    NSFetchRequest *request,
    NSUInteger *outCount,
    NSError *_Nullable *_Nullable error
) {
    @try {
        NSUInteger count = [context countForFetchRequest:request error:error];
        if (count == NSNotFound) {
            return NO;
        }
        *outCount = count;
        return YES;
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

NSPersistentStoreResult *_Nullable CDXExecuteRequest(
    NSManagedObjectContext *context,
    NSPersistentStoreRequest *request,
    NSError *_Nullable *_Nullable error
) {
    @try {
        return [context executeRequest:request error:error];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return nil;
    }
}

BOOL CDXSave(NSManagedObjectContext *context, NSError *_Nullable *_Nullable error) {
    @try {
        return [context save:error];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

BOOL CDXInsertObject(NSManagedObjectContext *context, NSManagedObject *object, NSError *_Nullable *_Nullable error) {
    @try {
        [context insertObject:object];
        return YES;
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

BOOL CDXDeleteObject(NSManagedObjectContext *context, NSManagedObject *object, NSError *_Nullable *_Nullable error) {
    @try {
        [context deleteObject:object];
        return YES;
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

BOOL CDXRefreshObject(
    NSManagedObjectContext *context,
    NSManagedObject *object,
    BOOL mergeChanges,
    NSError *_Nullable *_Nullable error
) {
    @try {
        [context refreshObject:object mergeChanges:mergeChanges];
        return YES;
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

BOOL CDXObtainPermanentIDs(
    NSManagedObjectContext *context,
    NSArray<NSManagedObject *> *objects,
    NSError *_Nullable *_Nullable error
) {
    @try {
        return [context obtainPermanentIDsForObjects:objects error:error];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

NSManagedObject *_Nullable CDXObjectWithID(
    NSManagedObjectContext *context,
    NSManagedObjectID *objectID,
    NSError *_Nullable *_Nullable error
) {
    @try {
        return [context objectWithID:objectID];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return nil;
    }
}

NSManagedObject *_Nullable CDXExistingObjectWithID(
    NSManagedObjectContext *context,
    NSManagedObjectID *objectID,
    NSError *_Nullable *_Nullable error
) {
    @try {
        return [context existingObjectWithID:objectID error:error];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return nil;
    }
}

NSManagedObject *_Nullable CDXNewManagedObject(
    NSEntityDescription *entity,
    NSManagedObjectContext *_Nullable context,
    NSError *_Nullable *_Nullable error
) {
    @try {
        return [[NSManagedObject alloc] initWithEntity:entity insertIntoManagedObjectContext:context];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return nil;
    }
}

NSManagedObject *_Nullable CDXInsertNewObjectForEntityName(
    NSString *entityName,
    NSManagedObjectContext *context,
    NSError *_Nullable *_Nullable error
) {
    @try {
        return [NSEntityDescription insertNewObjectForEntityForName:entityName inManagedObjectContext:context];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return nil;
    }
}

BOOL CDXHasFaultForRelationshipNamed(
    NSManagedObject *object,
    NSString *name,
    BOOL *outResult,
    NSError *_Nullable *_Nullable error
) {
    @try {
        *outResult = [object hasFaultForRelationshipNamed:name];
        return YES;
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

NSArray<NSManagedObjectID *> *_Nullable CDXObjectIDsForRelationshipNamed(
    NSManagedObject *object,
    NSString *name,
    NSError *_Nullable *_Nullable error
) {
    @try {
        return [object objectIDsForRelationshipNamed:name];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return nil;
    }
}

BOOL CDXMergeChangesFromContextDidSave(
    NSManagedObjectContext *context,
    NSNotification *notification,
    NSError *_Nullable *_Nullable error
) {
    @try {
        [context mergeChangesFromContextDidSaveNotification:notification];
        return YES;
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

NSFetchedResultsController *_Nullable CDXNewFetchedResultsController(
    NSFetchRequest *request,
    NSManagedObjectContext *context,
    NSString *_Nullable sectionNameKeyPath,
    NSString *_Nullable cacheName,
    NSError *_Nullable *_Nullable error
) {
    @try {
        return [[NSFetchedResultsController alloc] initWithFetchRequest:request
                                                   managedObjectContext:context
                                                     sectionNameKeyPath:sectionNameKeyPath
                                                              cacheName:cacheName];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return nil;
    }
}

BOOL CDXPerformFetch(NSFetchedResultsController *controller, NSError *_Nullable *_Nullable error) {
    @try {
        return [controller performFetch:error];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

NSPersistentStore *_Nullable CDXAddPersistentStore(
    NSPersistentStoreCoordinator *coordinator,
    NSString *storeType,
    NSString *_Nullable configuration,
    NSURL *_Nullable url,
    NSDictionary *_Nullable options,
    NSError *_Nullable *_Nullable error
) {
    @try {
        return [coordinator addPersistentStoreWithType:storeType
                                         configuration:configuration
                                                   URL:url
                                               options:options
                                                 error:error];
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return nil;
    }
}

BOOL CDXAddPersistentStoreWithDescription(
    NSPersistentStoreCoordinator *coordinator,
    NSPersistentStoreDescription *description,
    CDXStoreLoadHandler handler,
    NSError *_Nullable *_Nullable error
) {
    @try {
        [coordinator addPersistentStoreWithDescription:description completionHandler:handler];
        return YES;
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}

BOOL CDXLoadPersistentStores(
    NSPersistentContainer *container,
    CDXStoreLoadHandler handler,
    NSError *_Nullable *_Nullable error
) {
    @try {
        [container loadPersistentStoresWithCompletionHandler:handler];
        return YES;
    } @catch (NSException *exception) {
        CDXAssignException(exception, error);
        return NO;
    }
}
