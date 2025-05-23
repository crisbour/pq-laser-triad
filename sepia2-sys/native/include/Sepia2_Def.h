//-----------------------------------------------------------------------------
//
//      Sepia2_Def.h
//
//-----------------------------------------------------------------------------
//
//  symbols used by SEPIA2_LIB
//
//-----------------------------------------------------------------------------
//  HISTORY:
//
//  apo  22.12.05   release of the library interface
//
//  apo  12.02.07   introduced SML Multilaser Module (V1.0.2.0)
//
//  apo  02.12.09   introduced SWM Pulsar Waveform Module (V1.0.2.1)
//                  changed SEPIA2OBJECT_SLT to 0x42
//
//  apo  24.02.14   raised library version number to 1.1.0.x (V1.1.0.293)
//
//  apo  25.02.14   now the version number identifies the configuration
//                    x86 vs. x64 (V1.1.xx.294)
//
//  apo  24.04.15   from now on the device index stuff is defined here
//                    (and only here!) (V1.1.xx.429)
//
//  apo  30.11.20   introduced new FW version 2.01.<svn_build>
//
//  apo  22.01.21   raised to DLL version 1.2.<target>.<svn_build>
//                    due to USB driver changes
//
//  apo  25.01.21   introduced VIR, VUV module functions (for VisUV/IR) (V1.2.xx.641)
//
//  apo  05.07.21   introduced PRI module functions (for PUMUCL / Prima) (V1.2.xx.720)
//
//-----------------------------------------------------------------------------
//

#ifndef   __SEPIA2_DEF_H__
  #define __SEPIA2_DEF_H__

  #include <stdbool.h>
  #include "Portabt.h"

  #ifdef _WIN64
    #define LIB_VERSION_REFERENCE                      "1.2.64.*" // this is the part to compare with, but ignore the asterisk
  #else
    #define LIB_VERSION_REFERENCE                      "1.2.32.*" // this is the part to compare with, but ignore the asterisk
  #endif

  #define SEPIA2_VERSIONINFO_LEN                      11          // "1.2.xx.nnn\0" where xx is either 32 or 64 and nnn is the SVN build number
  #define LIB_VERSION_REFERENCE_COMPLEN                7

  #define SEPIA2_FW_VERSIONINFO_LEN                   10          // "2.01.nnn\0" where nnn is the firmware build number
  #define FW_VERSION_REFERENCE_OLD                     "1.05."
  #define FW_VERSION_REFERENCE                         "2.01."
  #define FW_VERSION_REFERENCE_COMPLEN                 5

  #ifndef   SEPIA2_MAX_USB_DEVICES
    #define SEPIA2_MAX_USB_DEVICES                     8
  #endif // SEPIA2_MAX_USB_DEVICES

  #define SEPIA2_PRODID                                7

  #define SEPIA2_STRDSCR_IDX_VENDOR                    1
  #define SEPIA2_STRDSCR_IDX_MODEL                     2
  #define SEPIA2_STRDSCR_IDX_BUILD                     3
  #define SEPIA2_STRDSCR_IDX_SERIAL                    4

  #define SEPIA2_SUPREQ_OPT_NO_PREAMBLE                0x0001
  #define SEPIA2_SUPREQ_OPT_NO_TITLE                   0x0002
  #define SEPIA2_SUPREQ_OPT_NO_CALLING_SW_INDENT       0x0004
  #define SEPIA2_SUPREQ_OPT_NO_SYSTEM_INFO             0x0008


  // error codes  --------------------------------------------------------------------------------------------------------------

  #ifndef    SEPIA2_NO_ERROR
    #define  SEPIA2_NO_ERROR                                    ((short)(0))
  #endif



  #define SEPIA2_SOM_BURSTCHANNEL_COUNT                8

  #define SEPIA2_RESTART                               1
  #define SEPIA2_NO_RESTART                            0

  #define SEPIA2_LASER_LOCKED                          1
  #define SEPIA2_LASER_UNLOCKED                        0

  #define SEPIA2_PRIMARY_MODULE                        1
  #define SEPIA2_SECONDARY_MODULE                      0

  #define SEPIA2_SLM_PULSE_MODE                        1
  #define SEPIA2_SLM_CW_MODE                           0

  #define SEPIA2_SML_PULSE_MODE                        1
  #define SEPIA2_SML_CW_MODE                           0

  #define SEPIA2_SOM_INVERSE_SYNC_MASK                 1
  #define SEPIA2_SOM_STANDARD_SYNC_MASK                0


  #define SEPIA2_USB_STRDECR_LEN                     256
  #define SEPIA2_ERRSTRING_LEN                        64
  #ifndef SEPIA2_FW_ERRCOND_LEN
    #define SEPIA2_FW_ERRCOND_LEN                     55
  #endif
  #define SEPIA2_FW_ERRPHASE_LEN                      24
  #define SEPIA2_SERIALNUMBER_LEN                     13
  #define SEPIA2_PRODUCTMODEL_LEN                     33
  #define SEPIA2_MODULETYPESTRING_LEN                 55
  #define SEPIA2_SLM_FREQ_TRIGMODE_LEN                28
  #define SEPIA2_SLM_HEADTYPE_LEN                     18
  #define SEPIA2_SOM_FREQ_TRIGMODE_LEN                32
  #define SEPIA2_SWS_MODULETYPE_LEN                   20
  #define SEPIA2_SWS_MODULESTATE_LEN                  20
  #define SEPIA2_VUV_VIR_DEVTYPE_LEN                  32
  #define SEPIA2_VUV_VIR_TRIGINFO_LEN                 15
  #define SEPIA2_PRI_DEVICE_ID_LEN                     6
  #define SEPIA2_PRI_DEVTYPE_LEN                      32
  #define SEPIA2_PRI_DEVICE_FW_LEN                     8
  #define SEPIA2_PRI_OPERMODE_LEN                     16
  #define SEPIA2_PRI_TRIGSRC_LEN                      26

  //
  //                                                         //              bit  7         6         5     4    3      2..0
  //      SepiaObjTyp                                        //                   L module  secondary            S
  //           -                                             //                   /         /         laser osc  /      typecnt
  //   construction table                                    //                   H backpl  primary              L or F
  //                                                         //
  #define SEPIA2OBJECT_FRMS                            0xC0  // 1 1 00  0 000     backplane primary   no    no   small  0      // primary backplane, small
  #define SEPIA2OBJECT_FRML                            0xC8  // 1 1 00  1 000     backplane primary   no    no   large  0      // primary backplane, large
  #define SEPIA2OBJECT_SCM                             0x40  // 0 1 00  0 000     module    primary   no    no   d.c.   0      // for Sepia II: Controller Modules (Safety Board)
  #define SEPIA2OBJECT_SLC                             0x41  // 0 1 00  0 001     module    primary   no    no   d.c.   1      // for Solea: Laser Coupler
  #define SEPIA2OBJECT_SLT                             0x42  // 0 1 00  0 010     module    primary   no    no   d.c.   2      // for Module Commissioning Site: Voltage Meter Module
  #define SEPIA2OBJECT_SWM                             0x43  // 0 1 00  0 011     module    primary   no    no   d.c.   3      // for PPL 400: Waveform Shaper Modules
  #define SEPIA2OBJECT_SWS                             0x44  // 0 1 00  0 100     module    primary   no    no   d.c.   4      // for Solea: Wavelength Selector Modules
  #define SEPIA2OBJECT_SPM                             0x45  // 0 1 00  0 101     module    primary   no    no   d.c.   5      // for Solea: Pumpcontrol Modules
  #define SEPIA2OBJECT_LMP1                            0x46  // 0 1 00  0 110     module    primary   no    no   small  6      // for Laser Test Site: Meter Module w. Shutter Control
  #define SEPIA2OBJECT_LMP8                            0x4E  // 0 1 00  1 110     module    primary   no    no   large  6      // for Laser Test Site: 8 Meter Modules
  #define SEPIA2OBJECT_SOM                             0x50  // 0 1 01  0 000     module    primary   no    yes  d.c.   0      // for Sepia II: Oscillator Modules
  #define SEPIA2OBJECT_SOMD                            0x51  // 0 1 01  0 001     module    primary   no    yes  d.c.   1      // for Sepia II: Oscillator Modules with Delay
  #define SEPIA2OBJECT_SML                             0x60  // 0 1 10  0 000     module    primary   yes   no   d.c.   0      // for Sepia II: Multi Laser Modules
  #define SEPIA2OBJECT_VCL                             0x61  // 0 1 10  0 000     module    primary   yes   no   d.c.   1      // for PPL 400: Voltage Controlled Laser Modules
  #define SEPIA2OBJECT_SLM                             0x70  // 0 1 11  0 000     module    primary   yes   yes  d.c.   0      // for Sepia II: Laser Driver Modules
  #define SEPIA2OBJECT_SSM                             0x71  // 0 1 11  0 001     module    primary   yes   yes  d.c.   1      // for Solea: Seed Laser Modules
  #define SEPIA2OBJECT_VIR                             0x72  // 0 1 11  0 010     module    primary   yes   yes  d.c.   2      // for VisIR: Laser Modules
  #define SEPIA2OBJECT_VUV                             0x73  // 0 1 11  0 011     module    primary   yes   yes  d.c.   3      // for VisUV: Laser Modules
  #define SEPIA2OBJECT_PRI                             0x74  // 0 1 11  0 100     module    primary   yes   yes  d.c.   4      // for Prima: Laser Modules (PUMUCL)
  #define SEPIA2OBJECT_LHS                             0x20  // 0 0 10  0 000     module    secondary yes   no   slow   0      // for Sepia II: Laser Head slow
  #define SEPIA2OBJECT_LHF                             0x28  // 0 0 10  1 000     module    secondary yes   no   fast   0      // for Sepia II: Laser Head fast
  #define SEPIA2OBJECT_LH_                             0x29  // 0 0 10  1 001     module    secondary yes   no   fast   1
  #define SEPIA2OBJECT_FAIL                            0xFF

  #define SEPIA2_SLM_FREQ_80MHZ                        0
  #define SEPIA2_SLM_FREQ_40MHZ                        1
  #define SEPIA2_SLM_FREQ_20MHZ                        2
  #define SEPIA2_SLM_FREQ_10MHZ                        3
  #define SEPIA2_SLM_FREQ_5MHZ                         4
  #define SEPIA2_SLM_FREQ_2_5MHZ                       5
  #define SEPIA2_SLM_TRIGMODE_RAISING                  6
  #define SEPIA2_SLM_TRIGMODE_FALLING                  7
  #define SEPIA2_SLM_FREQ_TRIGMODE_COUNT               8


  #define SEPIA2_SLM_HEADTYPE_FAILURE                  0
  #define SEPIA2_SLM_HEADTYPE_LED                      1
  #define SEPIA2_SLM_HEADTYPE_LASER                    2
  #define SEPIA2_SLM_HEADTYPE_NONE                     3
  #define SEPIA2_SLM_HEADTYPE_COUNT                    4


  #define SEPIA2_SML_HEADTYPE_FAILURE                  0
  #define SEPIA2_SML_HEADTYPE_4_LEDS                   1
  #define SEPIA2_SML_HEADTYPE_4_LASERS                 2
  #define SEPIA2_SML_HEADTYPE_COUNT                    3


  #ifndef SEPIA2_SOM_TRIGGERLEVEL_STEP
    #define SEPIA2_SOM_TRIGGERLEVEL_STEP              20 // in mV
    #define SEPIA2_SOM_TRIGGERLEVEL_HALFSTEP           (SEPIA2_SOM_TRIGGERLEVEL_STEP / 2)
  #endif
  #define SEPIA2_SOM_TRIGMODE_RISING                   0
  #define SEPIA2_SOM_TRIGMODE_FALLING                  1
  #define SEPIA2_SOM_INT_OSC_A                         2
  #define SEPIA2_SOM_INT_OSC_B                         3
  #define SEPIA2_SOM_INT_OSC_C                         4
  #define SEPIA2_SOM_FREQ_TRIGMODE_COUNT               5
  #define SEPIA2_SOMD_TRIGMODE_RISING                  0
  #define SEPIA2_SOMD_TRIGMODE_FALLING                 1
  #define SEPIA2_SOMD_INT_OSC_A                        2
  #define SEPIA2_SOMD_INT_OSC_B                        3
  #define SEPIA2_SOMD_INT_OSC_C                        4
  #define SEPIA2_SOMD_FREQ_TRIGMODE_COUNT              5

  #define SEPIA2_SWM_CURVES_COUNT                      2
  #define SEPIA2_SWM_TIMEBASE_RANGES_COUNT             3
  //
  #define SEPIA2_SWM_UI_TABIDX_RESOLUTION              0
  #define SEPIA2_SWM_UI_TABIDX_MIN_USERVALUE           1
  #define SEPIA2_SWM_UI_TABIDX_MAX_USERVALUE           2
  #define SEPIA2_SWM_UI_TABIDX_USER_RESOLUTION         3
  #define SEPIA2_SWM_UI_TABIDX_MAX_AMPLITUDE           4
  #define SEPIA2_SWM_UI_TABIDX_MAX_SLEWRATE            5
  #define SEPIA2_SWM_UI_TABIDX_EXP_RAMP_EFFECT         6
  #define SEPIA2_SWM_UI_TABIDX_TIMEBASERANGES_COUNT    7
  #define SEPIA2_SWM_UI_TABIDX_PULSEDATA_COUNT         8
  #define SEPIA2_SWM_UI_TABIDX_RAMPDATA_COUNT          9
  #define SEPIA2_SWM_UI_TABIDX_DELAYDATA_TB0_COUNT    10
  #define SEPIA2_SWM_UI_TABIDX_DELAYDATA_TB1_COUNT    11
  #define SEPIA2_SWM_UI_TABIDX_DELAYDATA_TB2_COUNT    12

  typedef union {
    unsigned short    us;
    struct {
      byte            VersMin;
      byte            VersMaj;
    }                 vs;
  } T_Version_Short;

  typedef union {
    unsigned long   ul;
    struct {
      unsigned short  BuildNr;
      byte            VersMin;
      byte            VersMaj;
      }               v;
  } T_Module_FWVers;
  typedef T_Module_FWVers* T_ptrModule_FWVersion;

  #ifndef _SPM_TYPES_DEFINED_
    #define _SPM_TYPES_DEFINED_

    #define SEPIA2_SPM_TEMPERATURE_SENSORCOUNT         6

    typedef union {
      word wT [SEPIA2_SPM_TEMPERATURE_SENSORCOUNT];
      struct {
        word wT_Pump1;
        word wT_Pump2;
        word wT_Pump3;
        word wT_Pump4;
        word wT_FiberStack;
        word wT_AuxAdjust;
      } Temperatures;
    } T_SPM_Temperatures;
    typedef T_SPM_Temperatures* T_ptrSPM_Temperatures;

    typedef struct {
      T_SPM_Temperatures Temperatures;
      word               wOverAllCurrent;
      word               wOptionalSensor1;
      word               wOptionalSensor2;
    } T_SPM_SensorData;
    typedef T_SPM_SensorData* T_ptrSPM_SensorData;

  #endif // _SPM_TYPES_DEFINED_


    typedef struct _T_PRI_Constants {
      bool bInitialized;
      //                                             SEPIA2_PRI_OPERMODE_LEN
      //                                             SEPIA2_PRI_TRIGSRC_LEN
      char PrimaModuleID   [SEPIA2_PRI_DEVICE_ID_LEN + 1];
      char PrimaModuleType [SEPIA2_PRI_DEVTYPE_LEN   + 1];
      char PrimaFWVers     [SEPIA2_PRI_DEVICE_FW_LEN + 1];
      //
      float PrimaTemp_min;
      float PrimaTemp_max;
      //
      // 'til here      init with 0x00
      // --- - - - - - - - - - - -----  initializing border
      // from here      init with 0xFF
      //
      int  PrimaUSBIdx;
      int  PrimaSlotId;
      //
      int  PrimaWLCount;
      int  PrimaWLs[3];
      //
      int  PrimaOpModCount;
      int  PrimaOpModOff;
      int  PrimaOpModNarrow;
      int  PrimaOpModBroad;
      int  PrimaOpModCW;
      //
      int  PrimaTrSrcCount;
      int  PrimaTrSrcInt;
      int  PrimaTrSrcExtNIM;
      int  PrimaTrSrcExtTTL;
      int  PrimaTrSrcExtFalling;
      int  PrimaTrSrcExtRising;
      //
    } T_PRI_Constants;
    typedef  T_PRI_Constants* T_ptrPRI_Constants;


#endif // __SEPIA2_DEF_H__
