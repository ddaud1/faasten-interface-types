# Faasten Interface Types
The types that a Faasten function needs to know in order to make Faasten CloudCalls.

The types are captured in the following list of CloudCalls:

| CloudCall Name   | Argument Type | Return Type    |
|------------------|---------------|----------------|
| response         | Response      | Void           |
| buckle_parse     | string        | MaybeBuckle    |
| get_current_label| Void          | Buckle         |
| taint_with_label | Buckle        | Buckle         |
| declassify       | Component     | Buckle         |
| sub_privilege    | TokenList     | Void           |
| root             | Void          | DentResult     |
| dent_open        | DentOpen      | DentOpenResult |
| dent_close       | uint64        | DentResult     |
| dent_create      | DentCreate    | DentResult     |
| dent_update      | DentUpdate    | DentResult     |
| dent_read        | uint64        | DentResult     |
| dent_link        | DentLink      | DentResult     |
| dent_unlink      | DentUnlink    | DentResult     |
| dent_list        | uint64        | DentListResult |
| dent_ls_faceted  | DentLsFaceted | DentLsFacetedResult |
| dent_ls_gate     | uint64        | DentLsGateResult |
| dent_invoke      | DentInvoke    | DentInvokeResult |
| dent_get_blob    | uint64        | BlobResult       |
| blob_create      | BlobCreate    | BlobResult       |
| blob_write       | BlobWrite     | BlobResult       |
| blob_finalize    | BlobFinalize  | BlobResult       |
| blob_read        | BlobRead      | BlobResult       |
| blob_close       | BlobClose     | BlobResult       |


Response response ; 

string buckleParse = 2; // returns MaybeBuckle  
Void getCurrentLabel = 3; // returns Buckle  
Buckle taintWithLabel = 4; // returns Buckle (new current label)  
Component declassify = 5; // returns Buckle (new current label)  
TokenList subPrivilege = 6;  

Void              root           = 99; // returns DentResult  

DentOpen          dentOpen       =  7; // returns DentOpenResult  
uint64            dentClose      =  8; // returns DentResult  
DentCreate        dentCreate     =  9; // returns DentResult  
DentUpdate        dentUpdate     = 10; // returns DentResult  
uint64            dentRead       = 11; // returns DentResult  
uint64            dentList       = 12; // returns DentListResult  
DentLsFaceted     dentLsFaceted  = 13; // returns DentLsFacetedResult  
DentLink          dentLink       = 14; // returns DentResult  
DentUnlink        dentUnlink     = 15; // returns DentResult  
DentInvoke        dentInvoke     = 16; // returns DentInvokeResult  
uint64            dentLsGate     = 17; // returns DentLsGateResult  
uint64            dentGetBlob    = 18; // returns BlobResult  

BlobCreate        blobCreate     = 100; // returns BlobResult  
BlobWrite         blobWrite      = 101; // returns BlobResult  
BlobFinalize      blobFinalize   = 102; // returns BlobResult  
BlobRead          blobRead       = 103;  // returns BlobResult  
BlobClose         blobClose      = 104; // returns BlobResult  
