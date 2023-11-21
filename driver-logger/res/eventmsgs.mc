
; // Set of 5 event log resource messages as used by the winlog crate.


; // This is the header section.

LanguageNames=(English=0x409:MSG00409)

SeverityNames=(
               Informational=0x1:STATUS_SEVERITY_INFORMATIONAL
               Warning=0x2:STATUS_SEVERITY_WARNING
               Error=0x3:STATUS_SEVERITY_ERROR
              )


; // The following are the message definitions.

MessageIdTypedef=DWORD

MessageId=0x1
Severity=Error
SymbolicName=MSG_ERROR
Language=English
%1
.

MessageId=0x2
Severity=Warning
SymbolicName=MSG_WARNING
Language=English
%1
.

MessageId=0x3
Severity=Informational
SymbolicName=MSG_INFO
Language=English
%1
.

MessageId=0x4
Severity=Informational
SymbolicName=MSG_DEBUG
Language=English
%1
.

MessageId=0x5
Severity=Informational
SymbolicName=MSG_TRACE
Language=English
%1
.
