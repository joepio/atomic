//! Contains some of the most important Atomic Data URLs.

// Classes
pub const CLASS: &str = "https://atomicdata.dev/classes/Class";
pub const PROPERTY: &str = "https://atomicdata.dev/classes/Property";
pub const DATATYPE_CLASS: &str = "https://atomicdata.dev/classes/Datatype";
pub const COMMIT: &str = "https://atomicdata.dev/classes/Commit";
pub const AGENT: &str = "https://atomicdata.dev/classes/Agent";
pub const COLLECTION: &str = "https://atomicdata.dev/classes/Collection";
pub const ENDPOINT: &str = "https://atomicdata.dev/classes/Endpoint";

// Properties
pub const SHORTNAME: &str = "https://atomicdata.dev/properties/shortname";
pub const DESCRIPTION: &str = "https://atomicdata.dev/properties/description";
// ... for Properties
pub const IS_A: &str = "https://atomicdata.dev/properties/isA";
pub const DATATYPE_PROP: &str = "https://atomicdata.dev/properties/datatype";
pub const CLASSTYPE_PROP: &str = "https://atomicdata.dev/properties/classtype";
// ... for Classes
pub const REQUIRES: &str = "https://atomicdata.dev/properties/requires";
pub const RECOMMENDS: &str = "https://atomicdata.dev/properties/recommends";
// ... for Commits
pub const SUBJECT: &str = "https://atomicdata.dev/properties/subject";
pub const SET: &str = "https://atomicdata.dev/properties/set";
pub const REMOVE: &str = "https://atomicdata.dev/properties/remove";
pub const DESTROY: &str = "https://atomicdata.dev/properties/destroy";
pub const SIGNER: &str = "https://atomicdata.dev/properties/signer";
pub const CREATED_AT: &str = "https://atomicdata.dev/properties/createdAt";
pub const SIGNATURE: &str = "https://atomicdata.dev/properties/signature";
// ... for Agents
pub const PUBLIC_KEY: &str = "https://atomicdata.dev/properties/publicKey";
pub const NAME: &str = "https://atomicdata.dev/properties/name";
// ... for Collections
pub const COLLECTION_PROPERTY: &str = "https://atomicdata.dev/properties/collection/property";
pub const COLLECTION_VALUE: &str = "https://atomicdata.dev/properties/collection/value";
pub const COLLECTION_MEMBER_COUNT: &str = "https://atomicdata.dev/properties/collection/totalMembers";
pub const COLLECTION_TOTAL_PAGES: &str = "https://atomicdata.dev/properties/collection/totalPages";
pub const COLLECTION_CURRENT_PAGE: &str = "https://atomicdata.dev/properties/collection/currentPage";
pub const COLLECTION_MEMBERS: &str = "https://atomicdata.dev/properties/collection/members";
pub const COLLECTION_PAGE_SIZE: &str = "https://atomicdata.dev/properties/collection/pageSize";
// ... for Endpoints
pub const ENDPOINT_PARAMETERS: &str = "https://atomicdata.dev/properties/endpoint/parameters";
pub const PATH: &str = "https://atomicdata.dev/properties/path";
// ... for functions

// Datatypes
pub const STRING: &str = "https://atomicdata.dev/datatypes/string";
pub const MARKDOWN: &str = "https://atomicdata.dev/datatypes/markdown";
pub const SLUG: &str = "https://atomicdata.dev/datatypes/slug";
pub const ATOMIC_URL: &str = "https://atomicdata.dev/datatypes/atomicURL";
pub const INTEGER: &str = "https://atomicdata.dev/datatypes/integer";
pub const FLOAT: &str = "https://atomicdata.dev/datatypes/float";
pub const RESOURCE_ARRAY: &str = "https://atomicdata.dev/datatypes/resourceArray";
pub const BOOLEAN: &str = "https://atomicdata.dev/datatypes/boolean";
pub const DATE: &str = "https://atomicdata.dev/datatypes/date";
pub const TIMESTAMP: &str = "https://atomicdata.dev/datatypes/timestamp";

// Methods
pub const INSERT: &str = "https://atomicdata.dev/methods/insert";
pub const DELETE: &str = "https://atomicdata.dev/methods/delete";
