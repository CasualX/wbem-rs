/*!
Bindings for `wbemcli.h`.
*/

use ::winapi::{ULONG, LONG, LPCWSTR, HRESULT, SAFEARRAY, BSTR, VARIANT, LCID};
type CIMTYPE = LONG;

use ::com_sys::unknown::{IUnknown, IUnknownVtbl};

//----------------------------------------------------------------
// IWbemLocator

com_class!(_extern WbemLocator, {0x4590f811-0x1d3a-0x11d0-0x891f-0x00aa004b2e24}, IWbemLocator);

com_interface! {
	interface IWbemLocator(IWbemLocatorVtbl): IUnknown(IUnknownVtbl);
	{0xdc12a687-0x737f-0x11cf-0x884d-0x00aa004b2e24}
	pub ConnectServer: unsafe extern "stdcall" fn(
		This: *mut IWbemLocator,
		strNetworkResource: BSTR,
		strUser: BSTR,
		strPassword: BSTR,
		strLocale: BSTR,
		lSecurityFlags: LONG,
		strAuthority: BSTR,
		pCtx: *mut IWbemContext,
		ppNamespace: *mut *mut IWbemServices,
	) -> HRESULT,
}

//----------------------------------------------------------------
// IWbemContext

com_class!(_extern WbemContext, {0x674B6698-0xEE92-0x11d0-0xAD71-0x00C04FD8FDFF}, IWbemContext);

com_interface! {
	interface IWbemContext(IWbemContextVtbl): IUnknown(IUnknownVtbl);
	{0x44aca674-0xe8fc-0x11d0-0xa07c-0x00c04fb68820}
	pub Clone: unsafe extern "stdcall" fn(
		This: *mut IWbemContext,
		ppNewCopy: *mut *mut IWbemContext,
	) -> HRESULT,
	pub GetNames: unsafe extern "stdcall" fn(
		This: *mut IWbemContext,
		lFlags: LONG,
		pNames: *mut *mut SAFEARRAY,
	) -> HRESULT,
	pub BeginEnumeration: unsafe extern "stdcall" fn(
		This: *mut IWbemContext,
		lFlags: LONG,
	) -> HRESULT,
	pub Next: unsafe extern "stdcall" fn(
		This: *mut IWbemContext,
		pstrName: *mut BSTR,
		pValue: *mut VARIANT,
	) -> HRESULT,
	pub EndEnumeration: unsafe extern "stdcall" fn(
		This: *mut IWbemContext,
	) -> HRESULT,
	pub SetValue: unsafe extern "stdcall" fn(
		This: *mut IWbemContext,
		wszName: LPCWSTR,
		lFlags: LONG,
		pValue: *mut VARIANT,
	) -> HRESULT,
	pub GetValue: unsafe extern "stdcall" fn(
		This: *mut IWbemContext,
		wszName: LPCWSTR,
		lFlags: LONG,
		pValue: *mut VARIANT,
	) -> HRESULT,
	pub DeleteValue: unsafe extern "stdcall" fn(
		This: *mut IWbemContext,
		wszName: LPCWSTR,
		lFlags: LONG,
	) -> HRESULT,
	pub DeleteAll: unsafe extern "stdcall" fn(
		This: *mut IWbemContext,
	) -> HRESULT,
}

//----------------------------------------------------------------
// IWbemServices

com_interface! {
	interface IWbemServices(IWbemServicesVtbl): IUnknown(IUnknownVtbl);
	{0x9556dc99-0x828c-0x11cf-0xa37e-0x00aa003240c7}
	pub OpenNamespace: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strNamespace: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		ppWorkingNamespace: *mut *mut IWbemServices,
		ppResult: *mut *mut IWbemCallResult,
	) -> HRESULT,
	pub CancelAsyncCall: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		pSink: *mut IWbemObjectSink,
	) -> HRESULT,
	pub QueryObjectSink: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		lFlags: LONG,
		ppResponseHandler: *mut *mut IWbemObjectSink,
	) -> HRESULT,
	pub GetObject: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strObjectPath: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		ppObject: *mut *mut IWbemClassObject,
		ppCallResult: *mut *mut IWbemCallResult,
	) -> HRESULT,
	pub GetObjectAsync: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strObjectPath: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		pResponseHandler: *mut IWbemObjectSink,
	) -> HRESULT,
	pub PutClass: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		pObject: *mut IWbemClassObject,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		ppCallResult: *mut *mut IWbemCallResult,
	) -> HRESULT,
	pub PutClassAsync: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		pObject: *mut IWbemClassObject,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		pResponseHandler: *mut IWbemObjectSink,
	) -> HRESULT,
	pub DeleteClass: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strClass: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		ppCallResult: *mut *mut IWbemCallResult,
	) -> HRESULT,
	pub DeleteClassAsync: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strClass: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		pResponseHandler: *mut IWbemObjectSink,
	) -> HRESULT,
	pub CreateClassEnum: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strSuperclass: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		ppEnum: *mut *mut IEnumWbemClassObject,
	) -> HRESULT,
	pub CreateClassEnumAsync: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strSuperclass: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		pResponseHandler: *mut IWbemObjectSink,
	) -> HRESULT,
	pub PutInstance: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		pInst: *mut IWbemClassObject,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		ppCallResult: *mut *mut IWbemCallResult,
	) -> HRESULT,
	pub PutInstanceAsync: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		pInst: *mut IWbemClassObject,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		pResponseHandler: *mut IWbemObjectSink,
	) -> HRESULT,
	pub DeleteInstance: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strObjectPath: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		ppCallResult: *mut *mut IWbemCallResult,
	) -> HRESULT,
	pub DeleteInstanceAsync: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strObjectPath: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		pResponseHandler: *mut IWbemObjectSink,
	) -> HRESULT,
	pub CreateInstanceEnum: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strFilter: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		ppEnum: *mut *mut IEnumWbemClassObject,
	) -> HRESULT,
	pub CreateInstanceEnumAsync: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strFilter: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		pResponseHandler: *mut IWbemObjectSink,
	) -> HRESULT,
	pub ExecQuery: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strQueryLanguage: BSTR,
		strQuery: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		ppEnum: *mut *mut IEnumWbemClassObject,
	) -> HRESULT,
	pub ExecQueryAsync: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strQueryLanguage: BSTR,
		strQuery: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		pResponseHandler: *mut IWbemObjectSink,
	) -> HRESULT,
	pub ExecNotificationQuery: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strQueryLanguage: BSTR,
		strQuery: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		ppEnum: *mut *mut IEnumWbemClassObject,
	) -> HRESULT,
	pub ExecNotificationQueryAsync: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strQueryLanguage: BSTR,
		strQuery: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		pResponseHandler: *mut IWbemObjectSink,
	) -> HRESULT,
	pub ExecMethod: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strObjectPath: BSTR,
		strMethodName: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		pInParams: *mut IWbemClassObject,
		ppOutParams: *mut *mut IWbemClassObject,
		ppCallResult: *mut *mut IWbemCallResult,
	) -> HRESULT,
	pub ExecMethodAsync: unsafe extern "stdcall" fn(
		This: *mut IWbemServices,
		strObjectPath: BSTR,
		strMethodName: BSTR,
		lFlags: LONG,
		pCtx: *mut IWbemContext,
		pInParams: *mut IWbemClassObject,
		pResponseHandler: *mut IWbemObjectSink,
	) -> HRESULT,
}

//----------------------------------------------------------------
// IWbemCallResult

com_interface! {
	interface IWbemCallResult(IWbemCallResultVtbl): IUnknown(IUnknownVtbl);
	{0x44aca675-0xe8fc-0x11d0-0xa07c-0x00c04fb68820}
	pub GetResultObject: unsafe extern "stdcall" fn(
		This: *mut IWbemCallResult,
		lTimeout: LONG,
		ppResultObject: *mut *mut IWbemClassObject,
	) -> HRESULT,
	pub GetResultString: unsafe extern "stdcall" fn(
		This: *mut IWbemCallResult,
		lTimeout: LONG,
		pstrResultString: *mut BSTR,
	) -> HRESULT,
	pub GetResultServices: unsafe extern "stdcall" fn(
		This: *mut IWbemCallResult,
		lTimeout: LONG,
		ppServices: *mut *mut IWbemServices,
	) -> HRESULT,
	pub GetCallStatus: unsafe extern "stdcall" fn(
		This: *mut IWbemCallResult,
		lTimeout: LONG,
		plStatus: *mut LONG,
	) -> HRESULT,
}

//----------------------------------------------------------------
// IWbemObjectSink

com_interface! {
	interface IWbemObjectSink(IWbemObjectSinkVtbl): IUnknown(IUnknownVtbl);
	{0x7c857801-0x7381-0x11cf-0x884d-0x00aa004b2e24}
	pub Indicate: unsafe extern "stdcall" fn(
		This: *mut IWbemObjectSink,
		lObjectCount: LONG,
		apObjArray: *mut *mut IWbemClassObject,
	) -> HRESULT,
	pub SetStatus: unsafe extern "stdcall" fn(
		This: *mut IWbemObjectSink,
		lFlags: LONG,
		hResult: HRESULT,
		strParam: BSTR,
		pObjParam: *mut IWbemClassObject,
	) -> HRESULT,
}

//----------------------------------------------------------------
// IWbemClassObject

com_class!(_extern WbemClassObject, {0x9A653086-0x174F-0x11d2-0xB5F9-0x00104B703EFD}, IWbemClassObject);

com_interface! {
	interface IWbemClassObject(IWbemClassObjectVtbl): IUnknown(IUnknownVtbl);
	{0xdc12a681-0x737f-0x11cf-0x884d-0x00aa004b2e24}
	pub GetQualifierSet: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		ppQualSet: *mut *mut IWbemQualifierSet,
	) -> HRESULT,
	pub Get: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		wszName: LPCWSTR,
		lFlags: LONG,
		pVal: *mut VARIANT,
		pType: *mut CIMTYPE,
		plFlavor: *mut LONG,
	) -> HRESULT,
	pub Put: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		wszName: LPCWSTR,
		lFlags: LONG,
		pVal: *mut VARIANT,
		Type: CIMTYPE,
	) -> HRESULT,
	pub Delete: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		wszName: LPCWSTR,
	) -> HRESULT,
	pub GetNames: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		wszQualifierName: LPCWSTR,
		lFlags: LONG,
		pQualifierVal: *mut VARIANT,
		pNames: *mut *mut SAFEARRAY,
	) -> HRESULT,
	pub BeginEnumeration: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		lEnumFlags: LONG,
	) -> HRESULT,
	pub Next: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		lFlags: LONG,
		strName: *mut BSTR,
		pVal: *mut VARIANT,
		pType: *mut CIMTYPE,
		plFlavor: *mut LONG,
	) -> HRESULT,
	pub EndEnumeration: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
	) -> HRESULT,
	pub GetPropertyQualifierSet: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		wszProperty: LPCWSTR,
		ppQualSet: *mut *mut IWbemQualifierSet,
	) -> HRESULT,
	pub Clone: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		ppCopy: *mut *mut IWbemClassObject,
	) -> HRESULT,
	pub GetObjectText: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		lFlags: LONG,
		pstrObjectText: *mut BSTR,
	) -> HRESULT,
	pub SpawnDerivedClass: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		lFlags: LONG,
		ppNewClass: *mut *mut IWbemClassObject,
	) -> HRESULT,
	pub SpawnInstance: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		lFlags: LONG,
		ppNewInstance: *mut *mut IWbemClassObject,
	) -> HRESULT,
	pub CompareTo: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		lFlags: LONG,
		pCompareTo: *mut IWbemClassObject,
	) -> HRESULT,
	pub GetPropertyOrigin: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		wszName: LPCWSTR,
		pstrClassName: *mut BSTR,
	) -> HRESULT,
	pub InheritsFrom: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		strAncestor: LPCWSTR,
	) -> HRESULT,
	pub GetMethod: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		wszName: LPCWSTR,
		lFlags: LONG,
		ppInSignature: *mut *mut IWbemClassObject,
		ppOutSignature: *mut *mut IWbemClassObject,
	) -> HRESULT,
	pub PutMethod: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		wszName: LPCWSTR,
		lFlags: LONG,
		pInSignature: *mut IWbemClassObject,
		pOutSignature: *mut IWbemClassObject,
	) -> HRESULT,
	pub DeleteMethod: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		wszName: LPCWSTR,
	) -> HRESULT,
	pub BeginMethodEnumeration: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		lEnumFlags: LONG,
	) -> HRESULT,
	pub NextMethod: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		lFlags: LONG,
		pstrName: *mut BSTR,
		ppInSignature: *mut *mut IWbemClassObject,
		ppOutSignature: *mut *mut IWbemClassObject,
	) -> HRESULT,
	pub EndMethodEnumeration: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
	) -> HRESULT,
	pub GetMethodQualifierSet: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		wszMethod: LPCWSTR,
		ppQualSet: *mut *mut IWbemQualifierSet,
	) -> HRESULT,
	pub GetMethodOrigin: unsafe extern "stdcall" fn(
		This: *mut IWbemClassObject,
		wszMethodName: LPCWSTR,
		pstrClassName: *mut BSTR,
	) -> HRESULT,
}

//----------------------------------------------------------------
// IEnumWbemClassObject

com_interface! {
	interface IEnumWbemClassObject(IEnumWbemClassObjectVtbl): IUnknown(IUnknownVtbl);
	{0xdc12a681-0x737f-0x11cf-0x884d-0x00aa004b2e24}
	pub Reset: unsafe extern "stdcall" fn(
		This: *mut IEnumWbemClassObject,
	) -> HRESULT,
	pub Next: unsafe extern "stdcall" fn(
		This: *mut IEnumWbemClassObject,
		lTimeout: LONG,
		uCount: ULONG,
		apObjects: *mut *mut IWbemClassObject,
		puReturned: *mut ULONG,
	) -> HRESULT,
	pub NextAsync: unsafe extern "stdcall" fn(
		This: *mut IEnumWbemClassObject,
		uCount: ULONG,
		pSink: *mut IWbemObjectSink,
	) -> HRESULT,
	pub Clone: unsafe extern "stdcall" fn(
		This: *mut IEnumWbemClassObject,
		ppEnum: *mut *mut IEnumWbemClassObject,
	) -> HRESULT,
	pub Skip: unsafe extern "stdcall" fn(
		This: *mut IEnumWbemClassObject,
		lTimeout: LONG,
		nCount: ULONG,
	) -> HRESULT,
}

//----------------------------------------------------------------
// IWbemQualifierSet

com_interface! {
	interface IWbemQualifierSet(IWbemQualifierSetVtbl): IUnknown(IUnknownVtbl);
	{0xdc12a680-0x737f-0x11cf-0x884d-0x00aa004b2e24}
	pub Get: unsafe extern "stdcall" fn(
		This: *mut IWbemQualifierSet,
		wszName: LPCWSTR,
		lFlags: LONG,
		pVal: *mut VARIANT,
		plFlavor: *mut LONG,
	) -> HRESULT,
	pub Put: unsafe extern "stdcall" fn(
		This: *mut IWbemQualifierSet,
		wszName: LPCWSTR,
		pVal: *mut VARIANT,
		lFlavor: LONG,
	) -> HRESULT,
	pub Delete: unsafe extern "stdcall" fn(
		This: *mut IWbemQualifierSet,
		wszName: LPCWSTR,
	) -> HRESULT,
	pub GetNames: unsafe extern "stdcall" fn(
		This: *mut IWbemQualifierSet,
		lFlags: LONG,
		pNames: *mut *mut SAFEARRAY,
	) -> HRESULT,
	pub BeginEnumeration: unsafe extern "stdcall" fn(
		This: *mut IWbemQualifierSet,
		lFlags: LONG,
	) -> HRESULT,
	pub Next: unsafe extern "stdcall" fn(
		This: *mut IWbemQualifierSet,
		lFlags: LONG,
		pstrName: *mut BSTR,
		pVal: *mut VARIANT,
		plFlavor: *mut LONG,
	) -> HRESULT,
	pub EndEnumeration: unsafe extern "stdcall" fn(
		This: *mut IWbemQualifierSet,
	) -> HRESULT,
}

//----------------------------------------------------------------
// IWbemStatusCodeText

com_class!(_extern WbemStatusCodeText, {0xeb87e1bd-0x3233-0x11d2-0xaec9-0x00c04fb68820}, IWbemStatusCodeText);

com_interface! {
	interface IWbemStatusCodeText(IWbemStatusCodeTextVtbl): IUnknown(IUnknownVtbl);
	{0xeb87e1bc-0x3233-0x11d2-0xaec9-0x00c04fb68820}
	pub GetErrorCodeText: unsafe extern "stdcall" fn(
		This: *mut IWbemStatusCodeText,
		hRes: HRESULT,
		LocaleId: LCID,
		lFlags: LONG,
		MessageText: *mut BSTR,
	) -> HRESULT,
	pub GetFacilityCodeText: unsafe extern "stdcall" fn(
		This: *mut IWbemStatusCodeText,
		hRes: HRESULT,
		LocaleId: LCID,
		lFlags: LONG,
		MessageText: *mut BSTR,
	) -> HRESULT,
}

//----------------------------------------------------------------
// IWbemBackupRestore

com_class!(_extern WbemBackupRestore, {0xC49E32C6-0xBC8B-0x11d2-0x85D4-0x00105A1F8304}, IWbemBackupRestore);

com_interface! {
	interface IWbemBackupRestore(IWbemBackupRestoreVtbl): IUnknown(IUnknownVtbl);
	{0xC49E32C7-0xBC8B-0x11d2-0x85D4-0x00105A1F8304}
	pub Backup: unsafe extern "stdcall" fn(
		This: *mut IWbemBackupRestore,
		strBackupToFile: LPCWSTR,
		lFlags: LONG,
	) -> HRESULT,
	pub Restore: unsafe extern "stdcall" fn(
		This: *mut IWbemBackupRestore,
		strRestoreFromFile: LPCWSTR,
		lFlags: LONG,
	) -> HRESULT,
}

//----------------------------------------------------------------
// IWbemRefresher

com_class!(_extern WbemRefresher, {0xc71566f2-0x561e-0x11d1-0xad87-0x00c04fd8fdff}, IWbemRefresher);

com_interface! {
	interface IWbemRefresher(IWbemRefresherVtbl): IUnknown(IUnknownVtbl);
	{0x49353c99-0x516b-0x11d1-0xaea6-0x00c04fb68820}
	pub Refresh: unsafe extern "stdcall" fn(
		This: *mut IWbemRefresher,
		lFlags: LONG,
	) -> HRESULT,
}

//----------------------------------------------------------------
// IWbemObjectTextSrc

com_class!(_extern WbemObjectTextSrc, {0x8D1C559D-0x84F0-0x4bb3-0xA7D5-0x56A7435A9BA6}, IWbemObjectTextSrc);

com_interface! {
	interface IWbemObjectTextSrc(IWbemObjectTextSrcVtbl): IUnknown(IUnknownVtbl);
	{0xbfbf883a-0xcad7-0x11d3-0xa11b-0x00105a1f515a}
	pub GetText: unsafe extern "stdcall" fn(
		This: *mut IWbemObjectTextSrc,
		lFlags: LONG,
		pObj: *mut IWbemClassObject,
		uObjTextFormat: ULONG,
		pCtx: *mut IWbemContext,
		strText: *mut BSTR,
	) -> HRESULT,
	pub CreateFromText: unsafe extern "stdcall" fn(
		This: *mut IWbemObjectTextSrc,
		lFlags: LONG,
		pObj: *mut IWbemClassObject,
		uObjTextFormat: ULONG,
		pCtx: *mut IWbemContext,
		pNewObj: *mut *mut IWbemClassObject,
	) -> HRESULT,
}
