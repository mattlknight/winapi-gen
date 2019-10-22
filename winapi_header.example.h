/*++

Copyright (c) Microsoft Corporation. All rights reserved.

++*/
#ifdef __cplusplus
extern "C" {
#endif

#include <iprtrmib.h>
#include <ipexport.h>
#include <iptypes.h>
#include <tcpestats.h>

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Retrieves the number of interfaces in the system. These include LAN and  //
// WAN interfaces                                                           //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////


DWORD
WINAPI
GetNumberOfInterfaces(
    _Out_ PDWORD  pdwNumIf
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Gets the MIB-II ifEntry                                                  //
// The dwIndex field of the MIB_IFROW should be set to the index of the     //
// interface being queried                                                  //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

DWORD
WINAPI
GetIfEntry(
    _Inout_ PMIB_IFROW   pIfRow
    );
