### Faasten Interface Types
The types that a function needs to know in order to make Faasten CloudCalls.

The types are captured in the following list of CloudCalls:

| CloudCall Name | Argument Type | Return Type |
|----------------|---------------|-------------|
| response       | None          | None        |



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
