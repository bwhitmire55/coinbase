pub mod base;
pub mod pro;
pub mod error;
pub mod url;

// STATUS CODE

// 200 - Ok            - Successfull request
// 201 - Created       - New object saved
// 204 - No content    - Object deleted
// 400 - Bad Request   - Returns JSON with error message
// 401 - Unauthorized  - Couldn't authenticate your request
// 402 - 2FA Token required - Re-try requrest with user's 2FA token as 'CB-2FA-Token' header
// 403 - Invalid scope - User hans't authorized necessary scope
// 404 - Not found     - No such object
// 429 - Too many requests - Your connection is being rate limited
// 500 - Internal Server Error - Something went wrong
// 503 - Service Unavailable   - Your connection is being thorttled or the service is down for maintenance

// LOCALIZATION
//     'Accept-Language' header
// de - Dutch
// en - English
// es - Spanish
// es-mx - Spanish (Mexico)
// fr - French
// id - Indonesian
// it - Italian
// nl - Netherlands
// pt - Portugues
// pt-br - Portugues (Brazil)


// TIMESTAMPS
//     ISO8601 format in UTC

// PAGINATION (parameters)
// limit - 0-100 (default 25)
// order 'asc' or 'desc' (default)
// starting_after - supply resource id
// ending_before - supply resource id

// ERRORS
//     Can sometimes return multiple errors per request
// two_factor_required
// param_required
// validation_error
// invalid_request
// personal_details_required
// identity_verification_required
// jumio_verification_required
// jumio_face_match_verification_required
// unverified_email
// authentication_error
// invalid_token
// revoked_token
// expired_token
// invalid_scope
// not_found
// rate_limit_exceeded
// internal_server_error

// VERSIONING
//     Passed in 'CB-VERSION' header in YYYY-MM-DD format

// EXPANDING RESOURCES
//     Pass as param '?expand=all' or separate
//     ?expand[]]=to&expand[]=account
