/*
 * Sepia2_Lib64.dll
 *
 * Generated from Sepia2_Lib64.dll by winedump.
 *
 * DO NOT SEND GENERATED DLLS FOR INCLUSION INTO WINE !
 *
 */
#ifndef __WINE_SEPIA2_LIB64_DLL_H
#define __WINE_SEPIA2_LIB64_DLL_H

//#include "windef.h"
//#include "wine/debug.h"
//#include "winbase.h"
//#include "winnt.h"

#include "./Portabt.h"
#include "./Sepia2_Def.h"

int __stdcall SEPIA2_LIB64_SEPIA2_COM_DecodeModuleType(int iModuleType, char * cModulType);
int __stdcall SEPIA2_LIB64_SEPIA2_COM_DecodeModuleTypeAbbr(int iModuleType, char * cModulTypeAbbr);
int __stdcall SEPIA2_LIB64_SEPIA2_COM_GetFormatVersion(int iDevIdx, int iSlotId, int iGetPrimary, word * pwFormatVersion);
int __stdcall SEPIA2_LIB64_SEPIA2_COM_GetModuleType(int iDevIdx, int iSlotId, int iGetPrimary, int * piModuleType);
int __stdcall SEPIA2_LIB64_SEPIA2_COM_GetPresetInfo(int iDevIdx, int iSlotId, int iGetPrimary, int iPresetNr, unsigned char * pbIsSet, char * cPresetMemo);
int __stdcall SEPIA2_LIB64_SEPIA2_COM_GetSerialNumber(int iDevIdx, int iSlotId, int iGetPrimary, char * cSerialNumber);
int __stdcall SEPIA2_LIB64_SEPIA2_COM_GetSupplementaryInfos(int iDevIdx, int iSlotId, int iGetPrimary, char * cLabel, char * cReleaseDate, char * cRevision, char * cHdrMemo);
int __stdcall SEPIA2_LIB64_SEPIA2_COM_HasSecondaryModule(int iDevIdx, int iSlotId, int * piHasSecondary);
int __stdcall SEPIA2_LIB64_SEPIA2_COM_IsWritableModule(int iDevIdx, int iSlotId, int iGetPrimary, unsigned char * pbIsWritable);
int __stdcall SEPIA2_LIB64_SEPIA2_COM_RecallPreset(int iDevIdx, int iSlotId, int iGetPrimary, int iPresetNr);
int __stdcall SEPIA2_LIB64_SEPIA2_COM_SaveAsPreset(int iDevIdx, int iSlotId, int iSetPrimary, int iPresetNr, char * cPresetMemo);
/* __cdecl SEPIA2_LIB64_SEPIA2_COM_Slot2PathString(); */
int __stdcall SEPIA2_LIB64_SEPIA2_COM_UpdateModuleData(int iDevIdx, int iSlotId, int iSetPrimary, char * pcDCLFileName);
int __stdcall SEPIA2_LIB64_SEPIA2_FWR_CreateSupportRequestText(int iDevIdx, char * cPreamble, char * cCallingSW, int iOptions, int iBufferLen, char * cBuffer);
int __stdcall SEPIA2_LIB64_SEPIA2_FWR_DecodeErrPhaseName(int iErrPhase, char * cErrorPhase);
int __stdcall SEPIA2_LIB64_SEPIA2_FWR_FreeModuleMap(int iDevIdx);
int __stdcall SEPIA2_LIB64_SEPIA2_FWR_GetLastError(int iDevIdx, int * piErrCode, int * piPhase, int * piLocation, int * piSlot, char * cCondition);
int __stdcall SEPIA2_LIB64_SEPIA2_FWR_GetModuleInfoByMapIdx(int iDevIdx, int iMapIdx, int * piSlotId, unsigned char * pbIsPrimary, unsigned char * pbIsBackPlane, unsigned char * pbHasUptimeCounter);
int __stdcall SEPIA2_LIB64_SEPIA2_FWR_GetModuleMap(int iDevIdx, int iPerformRestart, int * piModuleCount);
int __stdcall SEPIA2_LIB64_SEPIA2_FWR_GetUptimeInfoByMapIdx(int iDevIdx, int iMapIdx, unsigned long * pulMainPowerUp, unsigned long * pulActivePowerUp, unsigned long * pulScaledPowerUp);
int __stdcall SEPIA2_LIB64_SEPIA2_FWR_GetVersion(int iDevIdx, char * cFWVersion);
int __stdcall SEPIA2_LIB64_SEPIA2_FWR_GetWorkingMode(int iDevIdx, int * piMode);
int __stdcall SEPIA2_LIB64_SEPIA2_FWR_RollBackToPermanentValues(int iDevIdx);
int __stdcall SEPIA2_LIB64_SEPIA2_FWR_SetWorkingMode(int iDevIdx, int iMode);
int __stdcall SEPIA2_LIB64_SEPIA2_FWR_StoreAsPermanentValues(int iDevIdx);
/* __cdecl SEPIA2_LIB64_SEPIA2_LIB_9D2CFBCE(); */
/* __cdecl SEPIA2_LIB64_SEPIA2_LIB_B2F3B177(); */
int __stdcall SEPIA2_LIB64_SEPIA2_LIB_DecodeError(int iErrCode, char * cErrorString);
int __stdcall SEPIA2_LIB64_SEPIA2_LIB_GetLibUSBVersion(char * cLibUSBVersion);
int __stdcall SEPIA2_LIB64_SEPIA2_LIB_GetVersion(char * cLibVersion);
int __stdcall SEPIA2_LIB64_SEPIA2_LIB_IsRunningOnWine(unsigned char * pbIsRunningOnWine);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_DecodeOperationMode(int iDevIdx, int iSlotId, int iOperModeIdx, char * pcOperMode);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_DecodeTriggerSource(int iDevIdx, int iSlotId, int iTrgSrcIdx, char * pcTrgSrc, unsigned char * pbFrequencyEnabled, unsigned char * pbTrigLevelEnabled);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_DecodeWavelength(int iDevIdx, int iSlotId, int iWLIdx, int * piWL);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetDeviceInfo(int iDevIdx, int iSlotId, char * pcDeviceID, char * pcDeviceType, char * pcFW_Version, int * piWL_Count);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetFrequency(int iDevIdx, int iSlotId, int * piFrequency);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetFrequencyLimits(int iDevIdx, int iSlotId, int * piMinFreq, int * piMaxFreq);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetGateHighImpedance(int iDevIdx, int iSlotId, unsigned char * pbHighImpedance);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetGatingData(int iDevIdx, int iSlotId, int * piOnTime, int * piOffTimefact);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetGatingEnabled(int iDevIdx, int iSlotId, unsigned char * pbGatingEnabled);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetGatingLimits(int iDevIdx, int iSlotId, int * piMinOnTime, int * piMaxOnTime, int * pbMinOffTimefactor, int * pbMaxOffTimefactor);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetIntensity(int iDevIdx, int iSlotId, int iWLIdx, word * pwIntensity);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetOperationMode(int iDevIdx, int iSlotId, int * piOperModeIdx);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetTriggerLevel(int iDevIdx, int iSlotId, int * piTrgLevel);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetTriggerLevelLimits(int iDevIdx, int iSlotId, int * piTrg_MinLvl, int * piTrg_MaxLvl, int * piTrg_LvlRes);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetTriggerSource(int iDevIdx, int iSlotId, int * piTrgSrcIdx);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_GetWavelengthIdx(int iDevIdx, int iSlotId, int * piWLIdx);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_SetFrequency(int iDevIdx, int iSlotId, int iFrequency);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_SetGateHighImpedance(int iDevIdx, int iSlotId, unsigned char bHighImpedance);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_SetGatingData(int iDevIdx, int iSlotId, int iOnTime, int iOffTimefact);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_SetGatingEnabled(int iDevIdx, int iSlotId, unsigned char bGatingEnabled);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_SetIntensity(int iDevIdx, int iSlotId, int iWLIdx, word wIntensity);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_SetOperationMode(int iDevIdx, int iSlotId, int iOperModeIdx);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_SetTriggerLevel(int iDevIdx, int iSlotId, int iTrgLevel);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_SetTriggerSource(int iDevIdx, int iSlotId, int iTrgSrcIdx);
int __stdcall SEPIA2_LIB64_SEPIA2_PRI_SetWavelengthIdx(int iDevIdx, int iSlotId, int iWLIdx);
int __stdcall SEPIA2_LIB64_SEPIA2_SCM_GetLaserLocked(int iDevIdx, int iSlotId, unsigned char * pbLocked);
int __stdcall SEPIA2_LIB64_SEPIA2_SCM_GetLaserSoftLock(int iDevIdx, int iSlotId, unsigned char * pbSoftLocked);
int __stdcall SEPIA2_LIB64_SEPIA2_SCM_GetPowerAndLaserLEDS(int iDevIdx, int iSlotId, unsigned char * pbPowerLED, unsigned char * pbLaserActiveLED);
int __stdcall SEPIA2_LIB64_SEPIA2_SCM_SetLaserSoftLock(int iDevIdx, int iSlotId, unsigned char bSoftLocked);
int __stdcall SEPIA2_LIB64_SEPIA2_SLM_DecodeFreqTrigMode(int iFreq, char * cFreqTrigMode);
int __stdcall SEPIA2_LIB64_SEPIA2_SLM_DecodeHeadType(int iHeadType, char * cHeadType);
int __stdcall SEPIA2_LIB64_SEPIA2_SLM_GetIntensityFineStep(int iDevIdx, int iSlotId, unsigned short * pwIntensity);
int __stdcall SEPIA2_LIB64_SEPIA2_SLM_GetPulseParameters(int iDevIdx, int iSlotId, int * piFreq, unsigned char * pbPulseMode, int * piHeadType);
int __stdcall SEPIA2_LIB64_SEPIA2_SLM_SetIntensityFineStep(int iDevIdx, int iSlotId, unsigned short wIntensity);
int __stdcall SEPIA2_LIB64_SEPIA2_SLM_SetPulseParameters(int iDevIdx, int iSlotId, int iFreq, unsigned char bPulseMode);
int __stdcall SEPIA2_LIB64_SEPIA2_SML_DecodeHeadType(int iHeadType, char * cHeadType);
int __stdcall SEPIA2_LIB64_SEPIA2_SML_GetParameters(int iDevIdx, int iSlotId, unsigned char * pbPulseMode, int * piHead, unsigned char * pbIntensity);
int __stdcall SEPIA2_LIB64_SEPIA2_SML_SetParameters(int iDevIdx, int iSlotId, unsigned char bPulseMode, unsigned char bIntensity);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_DecodeAUXINSequencerCtrl(int iAUXInCtrl, char * cSequencerCtrl);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_DecodeFreqTrigMode(int iDevIdx, int iSlotId, int iFreqTrigIdx, char * cFreqTrigMode);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_DecodeModuleState(unsigned short wState, char * cStatusText);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_FWReadPage(int iDevIdx, int iSlotId, unsigned short iPageIdx, unsigned char * pbFWPage);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_FWWritePage(int iDevIdx, int iSlotId, unsigned short iPageIdx, unsigned char * pbFWPage);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetAUXIOSequencerCtrl(int iDevIdx, int iSlotId, unsigned char * pbAUXOutCtrl, unsigned char * pbAUXInCtrl);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetBurstLengthArray(int iDevIdx, int iSlotId, long * plBurstLen1, long * plBurstLen2, long * plBurstLen3, long * plBurstLen4, long * plBurstLen5, long * plBurstLen6, long * plBurstLen7, long * plBurstLen8);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetBurstValues(int iDevIdx, int iSlotId, unsigned short * pwDivider, unsigned char * pbPreSync, unsigned char * pbMaskSync);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetDelayUnits(int iDevIdx, int iSlotId, double * pfCoarseDlyStep, unsigned char * pbFineDlyStepCount);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetFWVersion(int iDevIdx, int iSlotId, unsigned long * pulFWVersion);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetFreqTrigMode(int iDevIdx, int iSlotId, int * piFreqTrigIdx, unsigned char * pbSynchronize);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetHWParams(int iDevIdx, int iSlotId, unsigned short * pwHWParTemp1, unsigned short * pwHWParTemp2, unsigned short * pwHWParTemp3, unsigned short * pwHWParVolt1, unsigned short * pwHWParVolt2, unsigned short * pwHWParVolt3, unsigned short * pwHWParVolt4, unsigned short * pwHWParAUX);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetOutNSyncEnable(int iDevIdx, int iSlotId, unsigned char * pbOutEnable, unsigned char * pbSyncEnable, unsigned char * pbSyncInverse);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetSeqOutputInfos(int iDevIdx, int iSlotId, unsigned char bSeqOutIdx, unsigned char * pbDelayed, unsigned char * pbForcedUndelayed, unsigned char * pbOutCombi, unsigned char * pbMaskedCombi, double * pf64CoarseDly, unsigned char * pbFineDly);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetStatusError(int iDevIdx, int iSlotId, unsigned short * pwState, short * piErrorCode);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetTrigSyncFreq(int iDevIdx, int iSlotId, unsigned char * pbFreqStable, unsigned long * pulTrigSyncFreq);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetTriggerLevel(int iDevIdx, int iSlotId, int * piMilliVolt);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_GetTriggerRange(int iDevIdx, int iSlotId, int * piMilliVoltLow, int * piMilliVoltHigh);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_SetAUXIOSequencerCtrl(int iDevIdx, int iSlotId, unsigned char bAUXOutCtrl, unsigned char bAUXInCtrl);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_SetBurstLengthArray(int iDevIdx, int iSlotId, long lBurstLen1, long lBurstLen2, long lBurstLen3, long lBurstLen4, long lBurstLen5, long lBurstLen6, long lBurstLen7, long lBurstLen8);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_SetBurstValues(int iDevIdx, int iSlotId, unsigned short wDivider, unsigned char bPreSync, unsigned char bMaskSync);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_SetFreqTrigMode(int iDevIdx, int iSlotId, int iFreqTrigIdx, unsigned char bSynchronize);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_SetOutNSyncEnable(int iDevIdx, int iSlotId, unsigned char bOutEnable, unsigned char bSyncEnable, unsigned char bSyncInverse);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_SetSeqOutputInfos(int iDevIdx, int iSlotId, unsigned char bSeqOutIdx, unsigned char bDelayed, unsigned char bOutCombi, unsigned char bMaskedCombi, double fCoarseDly, unsigned char bFineDly);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_SetTriggerLevel(int iDevIdx, int iSlotId, int iMilliVolt);
int __stdcall SEPIA2_LIB64_SEPIA2_SOMD_SynchronizeNow(int iDevIdx, int iSlotId);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_DecodeAUXINSequencerCtrl(int iAUXInCtrl, char * cSequencerCtrl);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_DecodeFreqTrigMode(int iDevIdx, int iSlotId, int iFreqTrigMode, char * cFreqTrigMode);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_GetAUXIOSequencerCtrl(int iDevIdx, int iSlotId, unsigned char * pbAUXOutCtrl, unsigned char * pbAUXInCtrl);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_GetBurstLengthArray(int iDevIdx, int iSlotId, long * plBurstLen1, long * plBurstLen2, long * plBurstLen3, long * plBurstLen4, long * plBurstLen5, long * plBurstLen6, long * plBurstLen7, long * plBurstLen8);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_GetBurstValues(int iDevIdx, int iSlotId, unsigned char * pbDivider, unsigned char * pbPreSync, unsigned char * pbMaskSync);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_GetFreqTrigMode(int iDevIdx, int iSlotId, int * piFreqTrigMode);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_GetOutNSyncEnable(int iDevIdx, int iSlotId, unsigned char * pbOutEnable, unsigned char * pbSyncEnable, unsigned char * pbSyncInverse);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_GetTriggerLevel(int iDevIdx, int iSlotId, int * piMilliVolt);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_GetTriggerRange(int iDevIdx, int iSlotId, int * piMilliVoltLow, int * piMilliVoltHigh);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_SetAUXIOSequencerCtrl(int iDevIdx, int iSlotId, unsigned char bAUXOutCtrl, unsigned char bAUXInCtrl);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_SetBurstLengthArray(int iDevIdx, int iSlotId, long lBurstLen1, long lBurstLen2, long lBurstLen3, long lBurstLen4, long lBurstLen5, long lBurstLen6, long lBurstLen7, long lBurstLen8);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_SetBurstValues(int iDevIdx, int iSlotId, unsigned char bDivider, unsigned char bPreSync, unsigned char bMaskSync);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_SetFreqTrigMode(int iDevIdx, int iSlotId, int iFreqTrigMode);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_SetOutNSyncEnable(int iDevIdx, int iSlotId, unsigned char bOutEnable, unsigned char bSyncEnable, unsigned char bSyncInverse);
int __stdcall SEPIA2_LIB64_SEPIA2_SOM_SetTriggerLevel(int iDevIdx, int iSlotId, int iMilliVolt);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_DecodeModuleState(unsigned short wState, char * cStatusText);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_GetDeviceDescription(int iDevIdx, int iSlotId, char * pcDeviceDescription);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_GetFWVersion(int iDevIdx, int iSlotId, unsigned long * pulFWVersion);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_GetFiberAmplifierFail(int iDevIdx, int iSlotId, unsigned char * pbFiberAmpFail);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_GetOperationTimers(int iDevIdx, int iSlotId, unsigned long * pulMainPwrSwitch, unsigned long * pulUTOverAll, unsigned long * pulUTSinceDelivery, unsigned long * pulUTSinceFiberChg);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_GetPumpPowerState(int iDevIdx, int iSlotId, unsigned char * pbPumpState, unsigned char * pbPumpMode);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_GetSensorData(int iDevIdx, int iSlotId, T_ptrSPM_SensorData pSensorData);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_GetStatusError(int iDevIdx, int iSlotId, unsigned short * pwState, short * piErrorCode);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_GetTemperatureAdjust(int iDevIdx, int iSlotId, T_ptrSPM_Temperatures pTempAdjust);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_ResetFiberAmplifierFail(int iDevIdx, int iSlotId, unsigned char bFiberAmpFail);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_SetFRAMWriteProtect(int iDevIdx, int iSlotId, unsigned char bWriteProtect);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_SetPumpPowerState(int iDevIdx, int iSlotId, unsigned char bPumpState, unsigned char bPumpMode);
int __stdcall SEPIA2_LIB64_SEPIA2_SPM_UpdateFirmware(int iDevIdx, int iSlotId, char * pcFWFileName);
int __stdcall SEPIA2_LIB64_SEPIA2_SSM_DecodeFreqTrigMode(int iDevIdx, int iSlotId, int iMainFreqTrigIdx, char * cMainFreqTrig, int * piMainFreq, unsigned char * pbTrigLevelEnabled);
int __stdcall SEPIA2_LIB64_SEPIA2_SSM_GetFRAMWriteProtect(int iDevIdx, int iSlotId, unsigned char * pbWriteProtect);
int __stdcall SEPIA2_LIB64_SEPIA2_SSM_GetTrigLevelRange(int iDevIdx, int iSlotId, int * piUpperTL, int * piLowerTL, int * piResolTL);
int __stdcall SEPIA2_LIB64_SEPIA2_SSM_GetTriggerData(int iDevIdx, int iSlotId, int * piMainFreqTrigIdx, int * piTrigLevel);
int __stdcall SEPIA2_LIB64_SEPIA2_SSM_SetFRAMWriteProtect(int iDevIdx, int iSlotId, unsigned char bWriteProtect);
int __stdcall SEPIA2_LIB64_SEPIA2_SSM_SetTriggerData(int iDevIdx, int iSlotId, int iMainFreqTrigIdx, int iTrigLevel);
int __stdcall SEPIA2_LIB64_SEPIA2_SWM_DecodeRangeIdx(int iDevIdx, int iSlotId, int iRangeIdx, int * iUpperLimit);
int __stdcall SEPIA2_LIB64_SEPIA2_SWM_GetCurveParams(int iDevIdx, int iSlotId, int iCurveIdx, unsigned char * bTBNdx, unsigned short * wPAPml, unsigned short * wRRPml, unsigned short * wPSPml, unsigned short * wRSPml, unsigned short * wWSPml);
int __stdcall SEPIA2_LIB64_SEPIA2_SWM_GetExtAtten(int iDevIdx, int iSlotId, float * fExtAtt);
int __stdcall SEPIA2_LIB64_SEPIA2_SWM_GetUIConstants(int iDevIdx, int iSlotId, unsigned char * bTBNdxCount, unsigned short * wMaxAmplitude, unsigned short * wMaxSlewRate, unsigned short * wExpRampEffect, unsigned short * wMinUserValue, unsigned short * wMaxUserValue, unsigned short * wUserResolution);
int __stdcall SEPIA2_LIB64_SEPIA2_SWM_SetCurveParams(int iDevIdx, int iSlotId, int iCurveIdx, unsigned char bTBNdx, unsigned short wPAPml, unsigned short wRRPml, unsigned short wPSPml, unsigned short wRSPml, unsigned short wWSPml);
int __stdcall SEPIA2_LIB64_SEPIA2_SWM_SetExtAtten(int iDevIdx, int iSlotId, float fExtAtt);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_DecodeModuleState(unsigned short wState, char * cStatusText);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_DecodeModuleType(int iModuleType, char * cModulType);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_GetBeamPos(int iDevIdx, int iSlotId, short * piBeamVPos, short * piBeamHPos);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_GetCalPointInfo(int iDevIdx, int iSlotId, short iWLIdx, short iBWIdx, unsigned long * pulWaveLength, unsigned long * pulBandWidth, short * piBeamVPos, short * piBeamHPos);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_GetCalTableSize(int iDevIdx, int iSlotId, unsigned short * pwWLIdxCount, unsigned short * pwBWIdxCount);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_GetFWVersion(int iDevIdx, int iSlotId, unsigned long * pulFWVersion);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_GetIntensity(int iDevIdx, int iSlotId, unsigned long * pulIntensityRaw, float * pfIntensity);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_GetModuleType(int iDevIdx, int iSlotId, int * piModuleType);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_GetParamRanges(int iDevIdx, int iSlotId, unsigned long * pulUpperWL, unsigned long * pulLowerWL, unsigned long * pulIncrWL, unsigned long * pulPPMToggleWL, unsigned long * pulUpperBW, unsigned long * pulLowerBW, unsigned long * pulIncrBW, int * piUpperBeamPos, int * piLowerBeamPos, int * piIncrBeamPos);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_GetParameters(int iDevIdx, int iSlotId, unsigned long * pulWaveLength, unsigned long * pulBandWidth);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_GetStatusError(int iDevIdx, int iSlotId, unsigned short * pwState, short * piErrorCode);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_SetBeamPos(int iDevIdx, int iSlotId, short iBeamVPos, short iBeamHPos);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_SetCalPointValues(int iDevIdx, int iSlotId, short iWLIdx, short iBWIdx, short iBeamVPos, short iBeamHPos);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_SetCalTableSize(int iDevIdx, int iSlotId, unsigned short wWLIdxCount, unsigned short wBWIdxCount, unsigned char bInit);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_SetCalibrationMode(int iDevIdx, int iSlotId, unsigned char bCalibrationMode);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_SetFRAMWriteProtect(int iDevIdx, int iSlotId, unsigned char bWriteProtect);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_SetParameters(int iDevIdx, int iSlotId, unsigned long ulWaveLength, unsigned long ulBandWidth);
int __stdcall SEPIA2_LIB64_SEPIA2_SWS_UpdateFirmware(int iDevIdx, int iSlotId, char * pcFWFileName);
int __stdcall SEPIA2_LIB64_SEPIA2_USB_CloseDevice(int iDevIdx);
int __stdcall SEPIA2_LIB64_SEPIA2_USB_GetStrDescrByIdx(int iDevIdx, int iDescrIdx, char * cDescriptor);
int __stdcall SEPIA2_LIB64_SEPIA2_USB_GetStrDescriptor(int iDevIdx, char * cDescriptor);
int __stdcall SEPIA2_LIB64_SEPIA2_USB_IsOpenDevice(int iDevIdx, unsigned char * pbIsOpenDevice);
int __stdcall SEPIA2_LIB64_SEPIA2_USB_OpenDevice(int iDevIdx, char * cProductModel, char * cSerialNumber);
int __stdcall SEPIA2_LIB64_SEPIA2_USB_OpenGetSerNumAndClose(int iDevIdx, char * cProductModel, char * cSerialNumber);
int __stdcall SEPIA2_LIB64_SEPIA2_VCL_GetBiasVoltage(int iDevIdx, int iSlotId, int * piBiasVoltage);
int __stdcall SEPIA2_LIB64_SEPIA2_VCL_GetTemperature(int iDevIdx, int iSlotId, int * piTemperature);
int __stdcall SEPIA2_LIB64_SEPIA2_VCL_GetUIConstants(int iDevIdx, int iSlotId, int * piMinUserValueTmp, int * piMaxUserValueTmp, int * piUserResolutionTmp);
int __stdcall SEPIA2_LIB64_SEPIA2_VCL_SetTemperature(int iDevIdx, int iSlotId, int iTemperature);
int __stdcall SEPIA2_LIB64_SEPIA2_VUV_VIR_DecodeFreqTrigMode(int iDevIdx, int iSlotId, int iMainTrigSrcIdx, int iMainFreqDivIdx, char * pcMainFreqTrig, int * piMainFreq, unsigned char * pbTrigDividerEnabled, unsigned char * pbTrigLevelEnabled);
int __stdcall SEPIA2_LIB64_SEPIA2_VUV_VIR_GetDeviceType(int iDevIdx, int iSlotId, char * pcDeviceType, unsigned char * pbOptCW, unsigned char * pbOptFanSwitch);
int __stdcall SEPIA2_LIB64_SEPIA2_VUV_VIR_GetFan(int iDevIdx, int iSlotId, unsigned char * pbFanRunning);
int __stdcall SEPIA2_LIB64_SEPIA2_VUV_VIR_GetIntensity(int iDevIdx, int iSlotId, int * piIntensity);
int __stdcall SEPIA2_LIB64_SEPIA2_VUV_VIR_GetIntensityRange(int iDevIdx, int iSlotId, int * piUpperIntens, int * piLowerIntens, int * piResolIntens);
int __stdcall SEPIA2_LIB64_SEPIA2_VUV_VIR_GetTrigLevelRange(int iDevIdx, int iSlotId, int * piUpperTL, int * piLowerTL, int * piResolTL);
int __stdcall SEPIA2_LIB64_SEPIA2_VUV_VIR_GetTriggerData(int iDevIdx, int iSlotId, int * piMainTrigSrcIdx, int * piMainFreqDivIdx, int * piTrigLevel);
int __stdcall SEPIA2_LIB64_SEPIA2_VUV_VIR_SetFan(int iDevIdx, int iSlotId, unsigned char bFanRunning);
int __stdcall SEPIA2_LIB64_SEPIA2_VUV_VIR_SetIntensity(int iDevIdx, int iSlotId, int iIntensity);
int __stdcall SEPIA2_LIB64_SEPIA2_VUV_VIR_SetTriggerData(int iDevIdx, int iSlotId, int iMainTrigSrcIdx, int iMainFreqDivIdx, int iTrigLevel);



#endif	/* __WINE_SEPIA2_LIB64_DLL_H */
