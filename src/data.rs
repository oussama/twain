#[derive(Debug)]
#[repr(C)]
pub enum TWCB {
    AUTO = 0,
    CLEAR = 1,
    NOCLEAR = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWAF {
    WAV = 0,
    AIFF = 1,
    AU = 3,
    SND = 4,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWFS {
    FILESYSTEM = 0,
    RECURSIVEDELETE = 1,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWQC {
    GET = 0x0001,
    SET = 0x0002,
    GETDEFAULT = 0x0004,
    GETCURRENT = 0x0008,
    RESET = 0x0010,
    SETCONSTRAINT = 0x0020,
    GETHELP = 0x0100,
    GETLABEL = 0x0200,
    GETLABELENUM = 0x0400,
    CONSTRAINABLE = 0x0040,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWCC {
    SUCCESS = 0,
    BUMMER = 1,
    LOWMEMORY = 2,
    NODS = 3,
    MAXCONNECTIONS = 4,
    OPERATIONERROR = 5,
    BADCAP = 6,
    BADPROTOCOL = 9,
    BADVALUE = 10,
    SEQERROR = 11,
    BADDEST = 12,
    CAPUNSUPPORTED = 13,
    CAPBADOPERATION = 14,
    CAPSEQERROR = 15,
    DENIED = 16,
    FILEEXISTS = 17,
    FILENOTFOUND = 18,
    NOTEMPTY = 19,
    PAPERJAM = 20,
    PAPERDOUBLEFEED = 21,
    FILEWRITEERROR = 22,
    CHECKDEVICEONLINE = 23,
    INTERLOCK = 24,
    DAMAGEDCORNER = 25,
    FOCUSERROR = 26,
    DOCTOOLIGHT = 27,
    DOCTOODARK = 28,
    NOMEDIA = 29,
    CUSTOMBASE = 0x8000,
}
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C)]
pub enum TWRC {
    SUCCESS = 0,
    FAILURE = 1,
    CHECKSTATUS = 2,
    CANCEL = 3,
    DSEVENT = 4,
    NOTDSEVENT = 5,
    XFERDONE = 6,
    ENDOFLIST = 7,
    INFONOTSUPPORTED = 8,
    DATANOTAVAILABLE = 9,
    BUSY = 10,
    SCANNERLOCKED = 11,
    CUSTOMBASE = 0x8000,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWEJ {
    NONE = 0x0000,
    MIDSEPARATOR = 0x0001,
    PATCH1 = 0x0002,
    PATCH2 = 0x0003,
    PATCH3 = 0x0004,
    PATCH4 = 0x0005,
    PATCH6 = 0x0006,
    PATCHT = 0x0007,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWEI {
    BARCODEX = 0x1200,
    BARCODEY = 0x1201,
    BARCODETEXT = 0x1202,
    BARCODETYPE = 0x1203,
    DESHADETOP = 0x1204,
    DESHADELEFT = 0x1205,
    DESHADEHEIGHT = 0x1206,
    DESHADEWIDTH = 0x1207,
    DESHADESIZE = 0x1208,
    SPECKLESREMOVED = 0x1209,
    HORZLINEXCOORD = 0x120A,
    HORZLINEYCOORD = 0x120B,
    HORZLINELENGTH = 0x120C,
    HORZLINETHICKNESS = 0x120D,
    VERTLINEXCOORD = 0x120E,
    VERTLINEYCOORD = 0x120F,
    VERTLINELENGTH = 0x1210,
    VERTLINETHICKNESS = 0x1211,
    PATCHCODE = 0x1212,
    ENDORSEDTEXT = 0x1213,
    FORMCONFIDENCE = 0x1214,
    FORMTEMPLATEMATCH = 0x1215,
    FORMTEMPLATEPAGEMATCH = 0x1216,
    FORMHORZDOCOFFSET = 0x1217,
    FORMVERTDOCOFFSET = 0x1218,
    BARCODECOUNT = 0x1219,
    BARCODECONFIDENCE = 0x121A,
    BARCODEROTATION = 0x121B,
    BARCODETEXTLENGTH = 0x121C,
    DESHADECOUNT = 0x121D,
    DESHADEBLACKCOUNTOLD = 0x121E,
    DESHADEBLACKCOUNTNEW = 0x121F,
    DESHADEBLACKRLMIN = 0x1220,
    DESHADEBLACKRLMAX = 0x1221,
    DESHADEWHITECOUNTOLD = 0x1222,
    DESHADEWHITECOUNTNEW = 0x1223,
    DESHADEWHITERLMIN = 0x1224,
    DESHADEWHITERLAVE = 0x1225,
    DESHADEWHITERLMAX = 0x1226,
    BLACKSPECKLESREMOVED = 0x1227,
    WHITESPECKLESREMOVED = 0x1228,
    HORZLINECOUNT = 0x1229,
    VERTLINECOUNT = 0x122A,
    DESKEWSTATUS = 0x122B,
    SKEWORIGINALANGLE = 0x122C,
    SKEWFINALANGLE = 0x122D,
    SKEWCONFIDENCE = 0x122E,
    SKEWWINDOWX1 = 0x122F,
    SKEWWINDOWY1 = 0x1230,
    SKEWWINDOWX2 = 0x1231,
    SKEWWINDOWY2 = 0x1232,
    SKEWWINDOWX3 = 0x1233,
    SKEWWINDOWY3 = 0x1234,
    SKEWWINDOWX4 = 0x1235,
    SKEWWINDOWY4 = 0x1236,
    BOOKNAME = 0x1238,
    CHAPTERNUMBER = 0x1239,
    DOCUMENTNUMBER = 0x123A,
    PAGENUMBER = 0x123B,
    CAMERA = 0x123C,
    FRAMENUMBER = 0x123D,
    FRAME = 0x123E,
    PIXELFLAVOR = 0x123F,
    ICCPROFILE = 0x1240,
    LASTSEGMENT = 0x1241,
    SEGMENTNUMBER = 0x1242,
    MAGDATA = 0x1243,
    MAGTYPE = 0x1244,
    PAGESIDE = 0x1245,
    FILESYSTEMSOURCE = 0x1246,
    IMAGEMERGED = 0x1247,
    MAGDATALENGTH = 0x1248,
    PAPERCOUNT = 0x1249,
    PRINTERTEXT = 0x124A,
    TWAINDIRECTMETADATA = 0x124B,
}
#[derive(Debug)]
#[repr(C)]
pub enum ACAP {
    XFERMECH = 0x1202,
    AUDIOFILEFORMAT = 0x1201,
}
#[derive(Debug)]
#[repr(C)]
pub enum ICAP {
    COMPRESSION = 0x0100,
    PIXELTYPE = 0x0101,
    UNITS = 0x0102,
    XFERMECH = 0x0103,
    AUTOBRIGHT = 0x1100,
    BRIGHTNESS = 0x1101,
    CONTRAST = 0x1103,
    CUSTHALFTONE = 0x1104,
    EXPOSURETIME = 0x1105,
    FILTER = 0x1106,
    FLASHUSED = 0x1107,
    GAMMA = 0x1108,
    HALFTONES = 0x1109,
    HIGHLIGHT = 0x110a,
    IMAGEFILEFORMAT = 0x110c,
    LAMPSTATE = 0x110d,
    LIGHTSOURCE = 0x110e,
    ORIENTATION = 0x1110,
    PHYSICALWIDTH = 0x1111,
    PHYSICALHEIGHT = 0x1112,
    SHADOW = 0x1113,
    FRAMES = 0x1114,
    XNATIVERESOLUTION = 0x1116,
    YNATIVERESOLUTION = 0x1117,
    XRESOLUTION = 0x1118,
    YRESOLUTION = 0x1119,
    MAXFRAMES = 0x111a,
    TILES = 0x111b,
    BITORDER = 0x111c,
    CCITTKFACTOR = 0x111d,
    LIGHTPATH = 0x111e,
    PIXELFLAVOR = 0x111f,
    PLANARCHUNKY = 0x1120,
    ROTATION = 0x1121,
    SUPPORTEDSIZES = 0x1122,
    THRESHOLD = 0x1123,
    XSCALING = 0x1124,
    YSCALING = 0x1125,
    BITORDERCODES = 0x1126,
    PIXELFLAVORCODES = 0x1127,
    JPEGPIXELTYPE = 0x1128,
    TIMEFILL = 0x112a,
    BITDEPTH = 0x112b,
    BITDEPTHREDUCTION = 0x112c,
    UNDEFINEDIMAGESIZE = 0x112d,
    IMAGEDATASET = 0x112e,
    EXTIMAGEINFO = 0x112f,
    MINIMUMHEIGHT = 0x1130,
    MINIMUMWIDTH = 0x1131,
    AUTODISCARDBLANKPAGES = 0x1134,
    FLIPROTATION = 0x1136,
    BARCODEDETECTIONENABLED = 0x1137,
    SUPPORTEDBARCODETYPES = 0x1138,
    BARCODEMAXSEARCHPRIORITIES = 0x1139,
    BARCODESEARCHPRIORITIES = 0x113a,
    BARCODESEARCHMODE = 0x113b,
    BARCODEMAXRETRIES = 0x113c,
    BARCODETIMEOUT = 0x113d,
    ZOOMFACTOR = 0x113e,
    PATCHCODEDETECTIONENABLED = 0x113f,
    SUPPORTEDPATCHCODETYPES = 0x1140,
    PATCHCODEMAXSEARCHPRIORITIES = 0x1141,
    PATCHCODESEARCHPRIORITIES = 0x1142,
    PATCHCODESEARCHMODE = 0x1143,
    PATCHCODEMAXRETRIES = 0x1144,
    PATCHCODETIMEOUT = 0x1145,
    FLASHUSED2 = 0x1146,
    IMAGEFILTER = 0x1147,
    NOISEFILTER = 0x1148,
    OVERSCAN = 0x1149,
    AUTOMATICBORDERDETECTION = 0x1150,
    AUTOMATICDESKEW = 0x1151,
    AUTOMATICROTATE = 0x1152,
    JPEGQUALITY = 0x1153,
    FEEDERTYPE = 0x1154,
    ICCPROFILE = 0x1155,
    AUTOSIZE = 0x1156,
    AUTOMATICCROPUSESFRAME = 0x1157,
    AUTOMATICLENGTHDETECTION = 0x1158,
    AUTOMATICCOLORENABLED = 0x1159,
    AUTOMATICCOLORNONCOLORPIXELTYPE = 0x115a,
    COLORMANAGEMENTENABLED = 0x115b,
    IMAGEMERGE = 0x115c,
    IMAGEMERGEHEIGHTTHRESHOLD = 0x115d,
    SUPPORTEDEXTIMAGEINFO = 0x115e,
    FILMTYPE = 0x115f,
    MIRROR = 0x1160,
    JPEGSUBSAMPLING = 0x1161,
}
#[derive(Debug)]
#[repr(C)]
pub enum CAP {
    CUSTOMBASE = 0x8000,
    XFERCOUNT = 0x0001,
    AUTHOR = 0x1000,
    CAPTION = 0x1001,
    FEEDERENABLED = 0x1002,
    FEEDERLOADED = 0x1003,
    TIMEDATE = 0x1004,
    SUPPORTEDCAPS = 0x1005,
    EXTENDEDCAPS = 0x1006,
    AUTOFEED = 0x1007,
    CLEARPAGE = 0x1008,
    FEEDPAGE = 0x1009,
    REWINDPAGE = 0x100a,
    INDICATORS = 0x100b,
    PAPERDETECTABLE = 0x100d,
    UICONTROLLABLE = 0x100e,
    DEVICEONLINE = 0x100f,
    AUTOSCAN = 0x1010,
    THUMBNAILSENABLED = 0x1011,
    DUPLEX = 0x1012,
    DUPLEXENABLED = 0x1013,
    ENABLEDSUIONLY = 0x1014,
    CUSTOMDSDATA = 0x1015,
    ENDORSER = 0x1016,
    JOBCONTROL = 0x1017,
    ALARMS = 0x1018,
    ALARMVOLUME = 0x1019,
    AUTOMATICCAPTURE = 0x101a,
    TIMEBEFOREFIRSTCAPTURE = 0x101b,
    TIMEBETWEENCAPTURES = 0x101c,
    MAXBATCHBUFFERS = 0x101e,
    DEVICETIMEDATE = 0x101f,
    POWERSUPPLY = 0x1020,
    CAMERAPREVIEWUI = 0x1021,
    DEVICEEVENT = 0x1022,
    SERIALNUMBER = 0x1024,
    PRINTER = 0x1026,
    PRINTERENABLED = 0x1027,
    PRINTERINDEX = 0x1028,
    PRINTERMODE = 0x1029,
    PRINTERSTRING = 0x102a,
    PRINTERSUFFIX = 0x102b,
    LANGUAGE = 0x102c,
    FEEDERALIGNMENT = 0x102d,
    FEEDERORDER = 0x102e,
    REACQUIREALLOWED = 0x1030,
    BATTERYMINUTES = 0x1032,
    BATTERYPERCENTAGE = 0x1033,
    CAMERASIDE_OR_POWERDOWNTIME = 0x1034,
    SEGMENTED = 0x1035,
    CAMERAENABLED = 0x1036,
    CAMERAORDER = 0x1037,
    MICRENABLED = 0x1038,
    FEEDERPREP = 0x1039,
    FEEDERPOCKET = 0x103a,
    AUTOMATICSENSEMEDIUM = 0x103b,
    CUSTOMINTERFACEGUID = 0x103c,
    SUPPORTEDCAPSSEGMENTUNIQUE = 0x103d,
    SUPPORTEDDATS = 0x103e,
    DOUBLEFEEDDETECTION = 0x103f,
    DOUBLEFEEDDETECTIONLENGTH = 0x1040,
    DOUBLEFEEDDETECTIONSENSITIVITY = 0x1041,
    DOUBLEFEEDDETECTIONRESPONSE = 0x1042,
    PAPERHANDLING = 0x1043,
    INDICATORSMODE = 0x1044,
    PRINTERVERTICALOFFSET = 0x1045,
    POWERSAVETIME = 0x1046,
    PRINTERCHARROTATION = 0x1047,
    PRINTERFONTSTYLE = 0x1048,
    PRINTERINDEXLEADCHAR = 0x1049,
    PRINTERINDEXMAXVALUE = 0x104A,
    PRINTERINDEXNUMDIGITS = 0x104B,
    PRINTERINDEXSTEP = 0x104C,
    PRINTERINDEXTRIGGER = 0x104D,
    PRINTERSTRINGPREVIEW = 0x104E,
    SHEETCOUNT = 0x104F,
    CLEARBUFFERS = 0x101d,
    SUPPORTEDCAPSEXT = 0x100c,
    // FILESYSTEM = //0x????,
    PAGEMULTIPLEACQUIRE = 0x1023,
    PAPERBINDING = 0x102f,
    PASSTHRU = 0x1031,
}
#[derive(Debug)]
#[repr(u16)]
pub enum MSG {
    NULL = 0x0000,
    CUSTOMBASE = 0x8000,
    GET = 0x0001,
    GETCURRENT = 0x0002,
    GETDEFAULT = 0x0003,
    GETFIRST = 0x0004,
    GETNEXT = 0x0005,
    SET = 0x0006,
    RESET = 0x0007,
    QUERYSUPPORT = 0x0008,
    GETHELP = 0x0009,
    GETLABEL = 0x000a,
    GETLABELENUM = 0x000b,
    SETCONSTRAINT = 0x000c,
    XFERREADY = 0x0101,
    CLOSEDSREQ = 0x0102,
    CLOSEDSOK = 0x0103,
    DEVICEEVENT = 0x0104,
    OPENDSM = 0x0301,
    CLOSEDSM = 0x0302,
    OPENDS = 0x0401,
    CLOSEDS = 0x0402,
    USERSELECT = 0x0403,
    DISABLEDS = 0x0501,
    ENABLEDS = 0x0502,
    ENABLEDSUIONLY = 0x0503,
    PROCESSEVENT = 0x0601,
    ENDXFER = 0x0701,
    STOPFEEDER = 0x0702,
    CHANGEDIRECTORY = 0x0801,
    CREATEDIRECTORY = 0x0802,
    DELETE = 0x0803,
    FORMATMEDIA = 0x0804,
    GETCLOSE = 0x0805,
    GETFIRSTFILE = 0x0806,
    GETINFO = 0x0807,
    GETNEXTFILE = 0x0808,
    RENAME = 0x0809,
    COPY = 0x080A,
    AUTOMATICCAPTUREDIRECTORY = 0x080B,
    PASSTHRU = 0x0901,
    REGISTER_CALLBACK = 0x0902,
    RESETALL = 0x0A01,
    SETTASK = 0x0B01,
    CHECKSTATUS = 0x0201,
    INVOKE_CALLBACK = 0x0903,
}
#[derive(Debug)]
#[repr(u16)]
pub enum DAT {
    NULL = 0x0000,
    CUSTOMBASE = 0x8000,
    CAPABILITY = 0x0001,
    EVENT = 0x0002,
    IDENTITY = 0x0003,
    PARENT = 0x0004,
    PENDINGXFERS = 0x0005,
    SETUPMEMXFER = 0x0006,
    SETUPFILEXFER = 0x0007,
    STATUS = 0x0008,
    USERINTERFACE = 0x0009,
    XFERGROUP = 0x000a,
    CUSTOMDSDATA = 0x000c,
    DEVICEEVENT = 0x000d,
    FILESYSTEM = 0x000e,
    PASSTHRU = 0x000f,
    CALLBACK = 0x0010,
    STATUSUTF8 = 0x0011,
    CALLBACK2 = 0x0012,
    METRICS = 0x0013,
    TWAINDIRECT = 0x0014,
    IMAGEINFO = 0x0101,
    IMAGELAYOUT = 0x0102,
    IMAGEMEMXFER = 0x0103,
    IMAGENATIVEXFER = 0x0104,
    IMAGEFILEXFER = 0x0105,
    CIECOLOR = 0x0106,
    GRAYRESPONSE = 0x0107,
    RGBRESPONSE = 0x0108,
    JPEGCOMPRESSION = 0x0109,
    PALETTE8 = 0x010a,
    EXTIMAGEINFO = 0x010b,
    FILTER = 0x010c,
    AUDIOFILEXFER = 0x0201,
    AUDIOINFO = 0x0202,
    AUDIONATIVEXFER = 0x0203,
    ICCPROFILE = 0x0401,
    IMAGEMEMFILEXFER = 0x0402,
    ENTRYPOINT = 0x0403,
    TWUNKIDENTITY = 0x000b,
    SETUPFILEXFER2 = 0x0301,
}
#[derive(Debug)]
#[repr(C)]
pub enum DF {
    DSM2 = 0x10000000,
    APP2 = 0x20000000,
    DS2 = 0x40000000,
}
#[derive(Debug)]
#[repr(u32)]
pub enum DG {
    CONTROL = 0x0001,
    IMAGE = 0x0002,
    AUDIO = 0x0004,
    MASK = 0xFFFF,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWLG {
    DAN_OR_DANISH = 0,
    DUT_OR_DUTCH = 1,
    ENG_OR_ENGLISH = 2,
    FCF_OR_FRENCH_CANADIAN = 3,
    FIN_OR_FINNISH = 4,
    FRN_OR_FRENCH = 5,
    GER_OR_GERMAN = 6,
    ICE_OR_ICELANDIC = 7,
    ITN_OR_ITALIAN = 8,
    NOR_OR_NORWEGIAN = 9,
    POR_OR_PORTUGUESE = 10,
    SPA_OR_SPANISH = 11,
    SWE_OR_SWEDISH = 12,
    USA_OR_ENGLISH_USA = 13,
    AFRIKAANS = 14,
    ALBANIA = 15,
    ARABIC = 16,
    ARABIC_ALGERIA = 17,
    ARABIC_BAHRAIN = 18,
    ARABIC_EGYPT = 19,
    ARABIC_IRAQ = 20,
    ARABIC_JORDAN = 21,
    ARABIC_KUWAIT = 22,
    ARABIC_LEBANON = 23,
    ARABIC_LIBYA = 24,
    ARABIC_MOROCCO = 25,
    ARABIC_OMAN = 26,
    ARABIC_QATAR = 27,
    ARABIC_SAUDIARABIA = 28,
    ARABIC_SYRIA = 29,
    ARABIC_TUNISIA = 30,
    ARABIC_UAE = 31,
    ARABIC_YEMEN = 32,
    BASQUE = 33,
    BYELORUSSIAN = 34,
    BULGARIAN = 35,
    CATALAN = 36,
    CHINESE = 37,
    CHINESE_HONGKONG = 38,
    CHINESE_PRC = 39,
    CHINESE_SINGAPORE = 40,
    CHINESE_SIMPLIFIED = 41,
    CHINESE_TAIWAN = 42,
    CHINESE_TRADITIONAL = 43,
    CROATIA = 44,
    CZECH = 45,
    DUTCH_BELGIAN = 46,
    ENGLISH_AUSTRALIAN = 47,
    ENGLISH_CANADIAN = 48,
    ENGLISH_IRELAND = 49,
    ENGLISH_NEWZEALAND = 50,
    ENGLISH_SOUTHAFRICA = 51,
    ENGLISH_UK = 52,
    ESTONIAN = 53,
    FAEROESE = 54,
    FARSI = 55,
    FRENCH_BELGIAN = 56,
    FRENCH_LUXEMBOURG = 57,
    FRENCH_SWISS = 58,
    GERMAN_AUSTRIAN = 59,
    GERMAN_LUXEMBOURG = 60,
    GERMAN_LIECHTENSTEIN = 61,
    GERMAN_SWISS = 62,
    GREEK = 63,
    HEBREW = 64,
    HUNGARIAN = 65,
    INDONESIAN = 66,
    ITALIAN_SWISS = 67,
    JAPANESE = 68,
    KOREAN = 69,
    KOREAN_JOHAB = 70,
    LATVIAN = 71,
    LITHUANIAN = 72,
    NORWEGIAN_BOKMAL = 73,
    NORWEGIAN_NYNORSK = 74,
    POLISH = 75,
    PORTUGUESE_BRAZIL = 76,
    ROMANIAN = 77,
    RUSSIAN = 78,
    SERBIAN_LATIN = 79,
    SLOVAK = 80,
    SLOVENIAN = 81,
    SPANISH_MEXICAN = 82,
    SPANISH_MODERN = 83,
    THAI = 84,
    TURKISH = 85,
    UKRANIAN = 86,
    ASSAMESE = 87,
    BENGALI = 88,
    BIHARI = 89,
    BODO = 90,
    DOGRI = 91,
    GUJARATI = 92,
    HARYANVI = 93,
    HINDI = 94,
    KANNADA = 95,
    KASHMIRI = 96,
    MALAYALAM = 97,
    MARATHI = 98,
    MARWARI = 99,
    MEGHALAYAN = 100,
    MIZO = 101,
    NAGA = 102,
    ORISSI = 103,
    PUNJABI = 104,
    PUSHTU = 105,
    SERBIAN_CYRILLIC = 106,
    SIKKIMI = 107,
    SWEDISH_FINLAND = 108,
    TAMIL = 109,
    TELUGU = 110,
    TRIPURI = 111,
    URDU = 112,
    VIETNAMESE = 113,
    USERLOCALE = -1,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWCY {
    USA = 1,
    CANADA = 2,
    MEXICO = 3,
    CUBA = 5,
    BRITAIN = 6,
    USSR_OR_RUSSIA = 7,
    EGYPT = 20,
    SOUTHAFRICA = 27,
    GREECE = 30,
    NETHERLANDS = 31,
    BELGIUM = 32,
    ANDORRA_OR_FRANCE_OR_MONACO = 33,
    SPAIN = 34,
    HUNGARY = 36,
    YUGOSLAVIA = 38,
    ITALY_OR_SANMARINO_OR_VATICANCITY = 39,
    ROMANIA = 40,
    LIECHTENSTEIN_OR_SWITZERLAND = 41,
    CZECHOSLOVAKIA = 42,
    AUSTRIA = 43,
    UNITEDKINGDOM = 44,
    DENMARK = 45,
    SWEDEN = 46,
    NORWAY = 47,
    POLAND = 48,
    GERMANY = 49,
    PERU = 51,
    ARGENTINA = 54,
    BRAZIL = 55,
    CHILE = 56,
    COLOMBIA = 57,
    VENEZUELA = 58,
    MALAYSIA = 60,
    AUSTRALIA = 61,
    INDONESIA = 62,
    PHILLIPPINES = 63,
    NEWZEALAND = 64,
    SINGAPORE = 65,
    THAILAND = 66,
    JAPAN = 81,
    KOREA_OR_SOUTHKOREA = 82,
    VIETNAM = 84,
    CHINA = 86,
    TURKEY = 90,
    INDIA = 91,
    PAKISTAN = 92,
    SRILANKA = 94,
    MYANMAR = 95,
    IRAN = 98,
    MOROCCO = 212,
    ALGERIA = 213,
    TUNISIA = 216,
    LIBYA = 218,
    GAMBIA = 220,
    SENEGAL = 221,
    GUINEA = 224,
    IVORYCOAST = 225,
    NIGER = 227,
    TOGO = 228,
    BENIN = 229,
    MAURITIUS = 230,
    LIBERIA = 231,
    GHANA = 233,
    NIGERIA = 234,
    CAMAROON = 237,
    CAPEVERDEIS = 238,
    GABON = 241,
    ZAIRE = 243,
    DIEGOGARCIA = 246,
    ASCENSIONI = 247,
    RWANDA = 250,
    ETHIOPIA = 251,
    KENYA = 254,
    TANZANIA = 255,
    UGANDA = 256,
    ZAMBIA = 260,
    ZIMBABWE = 263,
    NAMIBIA = 264,
    MALAWI = 265,
    LESOTHO = 266,
    BOTSWANA = 267,
    SWAZILAND = 268,
    MAYOTTEIS = 269,
    ERITREA = 291,
    ARUBA = 297,
    FAEROEIS = 298,
    GREENLAND = 299,
    USVIRGINIS = 340,
    GIBRALTER = 350,
    PORTUGAL = 351,
    LUXENBOURG = 352,
    IRELAND = 353,
    ICELAND = 354,
    ALBANIA = 355,
    MALTA = 356,
    CYPRUS = 357,
    FINLAND = 358,
    BULGARIA = 359,
    LITHUANIA = 370,
    LATVIA = 371,
    ESTONIA = 372,
    MOLDOVA = 373,
    ARMENIA = 374,
    BELARUS = 375,
    UKRAINE = 380,
    SERBIA = 381,
    CROATIA = 385,
    SLOVENIA = 386,
    BOSNIAHERZGO = 387,
    MACEDONIA = 389,
    CZECHREPUBLIC = 420,
    SLOVAKIA = 421,
    BELIZE = 501,
    GUATEMALA = 502,
    ELSALVADOR = 503,
    HONDURAS = 504,
    NICARAGUA = 505,
    COSTARICA = 506,
    PANAMA = 507,
    MIQUELON_OR_STPIERRE = 508,
    HAITI = 509,
    GUADELOUPE = 590,
    BOLIVIA = 591,
    GUYANA = 592,
    ECUADOR = 593,
    FRGUIANA = 594,
    PARAGUAY = 595,
    FRANTILLES = 596,
    SURINAME = 597,
    URUGUAY = 598,
    NETHANTILLES = 599,
    SAIPAN = 670,
    GUAM = 671,
    BRUNEI = 673,
    PNEWGUINEA = 675,
    TONGAIS = 676,
    FIJIISLANDS = 679,
    AMERICANSAMOA = 684,
    NEWCALEDONIA = 687,
    FRPOLYNEISA = 689,
    MICRONESIA = 691,
    MARSHALLIS = 692,
    PUERTORICO = 787,
    NORTHKOREA = 850,
    HONGKONG = 852,
    MACAO = 853,
    CAMBODIA = 855,
    BANGLADESH = 880,
    TAIWAN = 886,
    MALDIVES = 960,
    JORDAN = 962,
    IRAQ = 964,
    KUWAIT = 965,
    SAUDIARABIA = 966,
    OMAN = 968,
    UAEMIRATES = 971,
    ISRAEL = 972,
    BAHRAIN = 973,
    QATAR = 974,
    NEPAL = 977,
    AZERBAIJAN = 994,
    GEORGIA = 995,
    AFGHANISTAN = 1001,
    ANGOLA = 1002,
    BHUTAN = 1003,
    BURKINAFASO = 1004,
    BURMA = 1005,
    BURUNDI = 1006,
    CENTRALAFREP = 1007,
    CHAD = 1008,
    CHRISTMASIS_OR_COCOSIS = 1009,
    COMOROS = 1010,
    CONGO = 1011,
    COOKIS = 1012,
    DJIBOUTI = 1013,
    EASTERIS = 1014,
    EQGUINEA = 1015,
    FALKLANDIS = 1016,
    GUINEABISSAU = 1017,
    KIRIBATI = 1018,
    LAOS = 1019,
    LEBANON = 1020,
    MADAGASCAR = 1021,
    MALI = 1022,
    MAURITANIA = 1023,
    MONGOLIA = 1024,
    MOZAMBIQUE = 1025,
    NAURU = 1026,
    NIUE = 1027,
    NORFOLKI = 1028,
    PALAU = 1029,
    PITCAIRNIS = 1030,
    REUNIONI = 1031,
    STHELENA = 1032,
    SAOTOME = 1033,
    SEYCHELLESIS = 1034,
    SIERRALEONE = 1035,
    SOLOMONIS = 1036,
    SOMALI = 1037,
    SUDAN = 1038,
    SYRIA = 1039,
    TUVALU = 1040,
    VANUATU = 1041,
    WAKE = 1042,
    FUTANAIS_OR_WALLISIS = 1043,
    WESTERNSAHARA = 1044,
    WESTERNSAMOA = 1045,
    YEMEN = 1046,
    GUANTANAMOBAY = 5399,
    JAMAICA = 8010,
    MONTSERRAT = 8011,
    NEVIS = 8012,
    STKITTS = 8013,
    STLUCIA = 8014,
    GRENEDINES_OR_STVINCENT = 8015,
    TOBAGO_OR_TRINIDAD = 8016,
    TURKSCAICOS = 8017,
    ANGUILLA = 8090,
    ANTIGUA = 8091,
    BAHAMAS = 8092,
    BARBADOS = 8093,
    BERMUDA = 8094,
    BRITVIRGINIS = 8095,
    CAYMANIS = 8096,
    DOMINICA = 8097,
    DOMINCANREP = 8098,
    GRENADA = 8099,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWUN {
    INCHES = 0,
    CENTIMETERS = 1,
    PICAS = 2,
    POINTS = 3,
    TWIPS = 4,
    PIXELS = 5,
    MILLIMETERS = 6,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWSX {
    NATIVE = 0,
    FILE = 1,
    MEMORY = 2,
    FILE2 = 3,
    MEMFILE = 4,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWSS {
    NONE = 0,
    A4_OR_A4LETTER = 1,
    JISB5_OR_B5LETTER = 2,
    USLETTER = 3,
    USLEGAL = 4,
    A5 = 5,
    ISOB4_OR_B4 = 6,
    ISOB6_OR_B6 = 7,
    B = 8,
    USLEDGER = 9,
    USEXECUTIVE = 10,
    A3 = 11,
    ISOB3_OR_B3 = 12,
    A6 = 13,
    C4 = 14,
    C5 = 15,
    C6 = 16,
    TWSS_4A0 = 17,
    TWSS_2A0 = 18,
    A0 = 19,
    A1 = 20,
    A2 = 21,
    A7 = 22,
    A8 = 23,
    A9 = 24,
    A10 = 25,
    ISOB0 = 26,
    ISOB1 = 27,
    ISOB2 = 28,
    ISOB5 = 29,
    ISOB7 = 30,
    ISOB8 = 31,
    ISOB9 = 32,
    ISOB10 = 33,
    JISB0 = 34,
    JISB1 = 35,
    JISB2 = 36,
    JISB3 = 37,
    JISB4 = 38,
    JISB6 = 39,
    JISB7 = 40,
    JISB8 = 41,
    JISB9 = 42,
    JISB10 = 43,
    C0 = 44,
    C1 = 45,
    C2 = 46,
    C3 = 47,
    C7 = 48,
    C8 = 49,
    C9 = 50,
    C10 = 51,
    USSTATEMENT = 52,
    BUSINESSCARD = 53,
    MAXSIZE = 54,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWCI {
    INFO = 0,
    WARNING = 1,
    ERROR = 2,
    WARMUP = 3,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWPH {
    NORMAL = 0,
    FRAGILE = 1,
    THICK = 2,
    TRIFOLD = 3,
    PHOTOGRAPH = 4,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWJS {
    TWJS_444YCBCR = 0,
    TWJS_444RGB = 1,
    TWJS_422 = 2,
    TWJS_421 = 3,
    TWJS_411 = 4,
    TWJS_420 = 5,
    TWJS_410 = 6,
    TWJS_311 = 7,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWMR {
    NONE = 0,
    VERTICAL = 1,
    HORIZONTAL = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWDP {
    STOP = 0,
    STOPANDWAIT = 1,
    SOUND = 2,
    DONOTIMPRINT = 3,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWUS {
    LOW = 0,
    MEDIUM = 1,
    HIGH = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWDF {
    ULTRASONIC = 0,
    BYLENGTH = 1,
    INFRARED = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWFM {
    POSITIVE = 0,
    NEGATIVE = 1,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWSG {
    NONE = 0,
    AUTO = 1,
    MANUAL = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWPT {
    BW = 0,
    GRAY = 1,
    RGB = 2,
    PALETTE = 3,
    CMY = 4,
    CMYK = 5,
    YUV = 6,
    YUVK = 7,
    CIEXYZ = 8,
    LAB = 9,
    SRGB = 10,
    SCRGB_OR_SRGB64 = 11,
    BGR = 12,
    CIELAB = 13,
    CIELUV = 14,
    YCBCR = 15,
    INFRARED = 16,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWPS {
    EXTERNAL = 0,
    BATTERY = 1,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWCT {
    PAGE = 0,
    PATCH1 = 1,
    PATCH2 = 2,
    PATCH3 = 3,
    PATCH4 = 4,
    PATCHT = 5,
    PATCH6 = 6,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWPR {
    IMPRINTERTOPBEFORE = 0,
    IMPRINTERTOPAFTER = 1,
    IMPRINTERBOTTOMBEFORE = 2,
    IMPRINTERBOTTOMAFTER = 3,
    ENDORSERTOPBEFORE = 4,
    ENDORSERTOPAFTER = 5,
    ENDORSERBOTTOMBEFORE = 6,
    ENDORSERBOTTOMAFTER = 7,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWPM {
    SINGLESTRING = 0,
    MULTISTRING = 1,
    COMPOUNDSTRING = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWPF {
    CHOCOLATE_OR_NORMAL = 0,
    VANILLA_OR_BOLD = 1,
    ITALIC = 2,
    LARGESIZE = 3,
    SMALLSIZE = 4,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWPCH {
    PATCH1 = 0,
    PATCH2 = 1,
    PATCH3 = 2,
    PATCH4 = 3,
    PATCH6 = 4,
    PATCHT = 5,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWPC {
    CHUNKY = 0,
    PLANAR = 1,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWPA {
    RGB = 0,
    GRAY = 1,
    CMY = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWOV {
    NONE = 0,
    AUTO = 1,
    TOPBOTTOM = 2,
    LEFTRIGHT = 3,
    ALL = 4,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWOR {
    ROT0_OR_PORTRAIT = 0,
    ROT90 = 1,
    ROT180 = 2,
    ROT270_OR_LANDSCAPE = 3,
    AUTO = 4,
    AUTOTEXT = 5,
    AUTOPICTURE = 6,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWNF {
    NONE = 0,
    AUTO = 1,
    LONEPIXEL = 2,
    MAJORITYRULE = 3,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWMD {
    MICR = 0,
    RAW = 1,
    INVALID = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWLS {
    RED = 0,
    GREEN = 1,
    BLUE = 2,
    NONE = 3,
    WHITE = 4,
    UV = 5,
    IR = 6,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWLP {
    REFLECTIVE = 0,
    TRANSMISSIVE = 1,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWJQ {
    UNKNOWN = -4,
    LOW = -3,
    MEDIUM = -2,
    HIGH = -1,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWJC {
    NONE = 0,
    JSIC = 1,
    JSIS = 2,
    JSXC = 3,
    JSXS = 4,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWIM {
    NONE = 0,
    FRONTONTOP = 1,
    FRONTONBOTTOM = 2,
    FRONTONLEFT = 3,
    FRONTONRIGHT = 4,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWIF {
    NONE = 0,
    AUTO = 1,
    LOWPASS = 2,
    BANDPASS_OR_TEXT = 3,
    HIGHPASS_OR_FINELINE = 4,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWIC {
    NONE = 0,
    LINK = 1,
    EMBED = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWFY {
    CAMERA = 0,
    CAMERATOP = 1,
    CAMERABOTTOM = 2,
    CAMERAPREVIEW = 3,
    DOMAIN = 4,
    HOST = 5,
    DIRECTORY = 6,
    IMAGE = 7,
    UNKNOWN = 8,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWFT {
    RED = 0,
    GREEN = 1,
    BLUE = 2,
    NONE = 3,
    WHITE = 4,
    CYAN = 5,
    MAGENTA = 6,
    YELLOW = 7,
    BLACK = 8,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWFR {
    BOOK = 0,
    FANFOLD = 1,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWFP {
    POCKETERROR = 0,
    POCKET1 = 1,
    POCKET2 = 2,
    POCKET3 = 3,
    POCKET4 = 4,
    POCKET5 = 5,
    POCKET6 = 6,
    POCKET7 = 7,
    POCKET8 = 8,
    POCKET9 = 9,
    POCKET10 = 10,
    POCKET11 = 11,
    POCKET12 = 12,
    POCKET13 = 13,
    POCKET14 = 14,
    POCKET15 = 15,
    POCKET16 = 16,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWFO {
    FIRSTPAGEFIRST = 0,
    LASTPAGEFIRST = 1,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWFL {
    NONE = 0,
    OFF = 1,
    ON = 2,
    AUTO = 3,
    REDEYE = 4,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWFF {
    TIFF = 0,
    PICT = 1,
    BMP = 2,
    XBM = 3,
    JFIF = 4,
    FPX = 5,
    TIFFMULTI = 6,
    PNG = 7,
    SPIFF = 8,
    EXIF = 9,
    PDF = 10,
    JP2 = 11,
    JPN = 12,
    JPX = 13,
    DEJAVU = 14,
    PDFA = 15,
    PDFA2 = 16,
    PDFRASTER = 17,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWFE {
    GENERAL = 0,
    PHOTO = 1,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWFA {
    NONE = 0,
    LEFT = 1,
    CENTER = 2,
    RIGHT = 3,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWDX {
    NONE = 0,
    TWDX_1PASSDUPLEX = 1,
    TWDX_2PASSDUPLEX = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWDSK {
    SUCCESS = 0,
    REPORTONLY = 1,
    FAIL = 2,
    DISABLED = 3,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWDR {
    GET = 1,
    SET = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWDE {
    CHECKAUTOMATICCAPTURE = 0,
    CHECKBATTERY = 1,
    CHECKDEVICEONLINE = 2,
    CHECKFLASH = 3,
    CHECKPOWERSUPPLY = 4,
    CHECKRESOLUTION = 5,
    DEVICEADDED = 6,
    DEVICEOFFLINE = 7,
    DEVICEREADY = 8,
    DEVICEREMOVED = 9,
    IMAGECAPTURED = 10,
    IMAGEDELETED = 11,
    PAPERDOUBLEFEED = 12,
    PAPERJAM = 13,
    LAMPFAILURE = 14,
    POWERSAVE = 15,
    POWERSAVENOTIFY = 16,
    CUSTOMEVENTS = 0x8000,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWCS {
    BOTH = 0,
    TOP = 1,
    BOTTOM = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWCP {
    NONE = 0,
    PACKBITS = 1,
    GROUP31D = 2,
    GROUP31DEOL = 3,
    GROUP32D = 4,
    GROUP4 = 5,
    JPEG = 6,
    LZW = 7,
    JBIG = 8,
    PNG = 9,
    RLE4 = 10,
    RLE8 = 11,
    BITFIELDS = 12,
    ZIP = 13,
    JPEG2000 = 14,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWBT {
    TWBT_3OF9 = 0,
    TWBT_2OF5INTERLEAVED = 1,
    TWBT_2OF5NONINTERLEAVED = 2,
    CODE93 = 3,
    CODE128 = 4,
    UCC128 = 5,
    CODABAR = 6,
    UPCA = 7,
    UPCE = 8,
    EAN8 = 9,
    EAN13 = 10,
    POSTNET = 11,
    PDF417 = 12,
    TWBT_2OF5INDUSTRIAL = 13,
    TWBT_2OF5MATRIX = 14,
    TWBT_2OF5DATALOGIC = 15,
    TWBT_2OF5IATA = 16,
    TWBT_3OF9FULLASCII = 17,
    CODABARWITHSTARTSTOP = 18,
    MAXICODE = 19,
    QRCODE = 20,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWBR {
    THRESHOLD = 0,
    HALFTONE = 1,
    CUSTHALFTONE = 2,
    DIFFUSION = 3,
    DYNAMICTHRESHOLD = 4,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWBP {
    DISABLE = -2,
    AUTO = -1,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWBO {
    LSBFIRST = 0,
    MSBFIRST = 1,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWBD {
    HORZ = 0,
    VERT = 1,
    HORZVERT = 2,
    VERTHORZ = 3,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWBCOR {
    ROT0 = 0,
    ROT90 = 1,
    ROT180 = 2,
    ROT270 = 3,
    ROTX = 4,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWAS {
    NONE = 0,
    AUTO = 1,
    CURRENT = 2,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWAL {
    ALARM = 0,
    FEEDERERROR = 1,
    FEEDERWARNING = 2,
    BARCODE = 3,
    DOUBLEFEED = 4,
    JAM = 5,
    PATCHCODE = 6,
    POWER = 7,
    SKEW = 8,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWTY {
    INT8 = 0x0000,
    INT16 = 0x0001,
    INT32 = 0x0002,
    UINT8 = 0x0003,
    UINT16 = 0x0004,
    UINT32 = 0x0005,
    BOOL = 0x0006,
    FIX32 = 0x0007,
    FRAME = 0x0008,
    STR32 = 0x0009,
    STR64 = 0x000a,
    STR128 = 0x000b,
    STR255 = 0x000c,
    HANDLE = 0x000f,
    STR1024 = 0x000d,
    UNI512 = 0x000e,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWMF {
    APPOWNS = 0x0001,
    DSMOWNS = 0x0002,
    DSOWNS = 0x0004,
    POINTER = 0x0008,
    HANDLE = 0x0010,
}
#[derive(Debug)]
#[repr(C)]
pub enum TWON {
    ARRAY = 3,
    ENUMERATION = 4,
    ONEVALUE = 5,
    RANGE = 6,
    DSMCODEID = 63,
    DSMID = 461,
    ICONID = 962,
    DONTCARE8 = 0xff,
    DONTCARE16 = 0xffff,
    DONTCARE32 = 0xfffff, // ffff,
}
/*



/* Flags used in TW_MEMORY structure. */








/****************************************************************************
 * Capability Constants                                                     *
 ****************************************************************************/

/* CAP_ALARMS values */

/* ICAP_AUTOSIZE values */

/* TWEI_BARCODEROTATION values */

/* ICAP_BARCODESEARCHMODE values */

/* ICAP_BITORDER values */

/* ICAP_AUTODISCARDBLANKPAGES values */

/* ICAP_BITDEPTHREDUCTION values */

/* ICAP_SUPPORTEDBARCODETYPES and TWEI_BARCODETYPE values*/

/* ICAP_COMPRESSION values*/

/* CAP_CAMERASIDE and TWEI_PAGESIDE values */

/* CAP_DEVICEEVENT values */

/* TW_PASSTHRU.Direction values. */

/* TWEI_DESKEWSTATUS values. */

/* CAP_DUPLEX values */

/* CAP_FEEDERALIGNMENT values */

/* ICAP_FEEDERTYPE values*/

/* ICAP_IMAGEFILEFORMAT values */

/* ICAP_FLASHUSED2 values */

/* CAP_FEEDERORDER values */

/* CAP_FEEDERPOCKET values*/

/* ICAP_FLIPROTATION values */

/* ICAP_FILTER values */

/* TW_FILESYSTEM.FileType values */

/* ICAP_ICCPROFILE values */

/* ICAP_IMAGEFILTER values */

/* ICAP_IMAGEMERGE values */

/* CAP_JOBCONTROL values  */

/* ICAP_JPEGQUALITY values */

/* ICAP_LIGHTPATH values */

/* ICAP_LIGHTSOURCE values */

/* TWEI_MAGTYPE values */

/* ICAP_NOISEFILTER values */

/* ICAP_ORIENTATION values */

/* ICAP_OVERSCAN values */

/* Palette types for TW_PALETTE8 */

/* ICAP_PLANARCHUNKY values */

/* TWEI_PATCHCODE values*/

/* ICAP_PIXELFLAVOR values */

/* CAP_PRINTERMODE values */

/* CAP_PRINTER values */

/* CAP_PRINTERFONTSTYLE Added 2.3 */

/* CAP_PRINTERINDEXTRIGGER Added 2.3 */

/* CAP_POWERSUPPLY values */

/* ICAP_PIXELTYPE values (PT_ means Pixel Type) */

/* CAP_SEGMENTED values */

/* ICAP_FILMTYPE values */

/* CAP_DOUBLEFEEDDETECTION */

/* CAP_DOUBLEFEEDDETECTIONSENSITIVITY */

/* CAP_DOUBLEFEEDDETECTIONRESPONSE */

/* ICAP_MIRROR values */

/* ICAP_JPEGSUBSAMPLING values */

/* CAP_PAPERHANDLING values */

/* CAP_INDICATORSMODE values */

/* ICAP_SUPPORTEDSIZES values (SS_ means Supported Sizes) */

/* ICAP_XFERMECH values (SX_ means Setup XFer) */

/* ICAP_UNITS values (UN_ means UNits) */


/****************************************************************************
 * Country Constants                                                        *
 ****************************************************************************/


/****************************************************************************
 * Language Constants                                                       *
 ****************************************************************************/


/****************************************************************************
 * Data Groups                                                              *
 ****************************************************************************/

/* More Data Functionality may be added in the future.
 * These are for items that need to be determined before DS is opened.
 * NOTE: Supported Functionality constants must be powers of 2 as they are
 *       used as bitflags when Application asks DSM to present a list of DSs.
 *       to support backward capability the App and DS will not use the fields
 */



/****************************************************************************
 *                                                        *
 ****************************************************************************/

/* Data Argument Types for the DG_CONTROL Data Group. */

/* Data Argument Types for the DG_IMAGE Data Group. */

/* Data Argument Types for the DG_AUDIO Data Group. */

/* misplaced */


/****************************************************************************
 * Messages                                                                 *
 ****************************************************************************/

/* All message constants are unique.
 * Messages are grouped according to which DATs they are used with.*/


/* Generic messages may be used with any of several DATs.                   */

/* Messages used with DAT_NULL                                              */

/* Messages used with a pointer to DAT_PARENT data                          */

/* Messages used with a pointer to a DAT_IDENTITY structure                 */

/* Messages used with a pointer to a DAT_USERINTERFACE structure            */

/* Messages used with a pointer to a DAT_EVENT structure                    */

/* Messages used with a pointer to a DAT_PENDINGXFERS structure             */

/* Messages used with a pointer to a DAT_FILESYSTEM structure               */

/* Messages used with a pointer to a DAT_PASSTHRU structure                 */

/* used with DAT_CALLBACK */

/* used with DAT_CAPABILITY */

/* used with DAT_TWAINDIRECT */

/****************************************************************************
 * Capabilities                                                             *
 ****************************************************************************/


/* all data sources are REQUIRED to support these caps */

/* image data sources are REQUIRED to support these caps */

/* all data sources MAY support these caps */



/* image data sources MAY support these caps */

/* image data sources MAY support these audio caps */


/***************************************************************************
 *            Extended Image Info Attributes section  Added 1.7            *
 ***************************************************************************/




/***************************************************************************
 *            Return Codes and Condition Codes section                     *
 ***************************************************************************/



/* Condition Codes: Application gets these by doing DG_CONTROL DAT_STATUS MSG_GET.  */


/* bit patterns: for query the operation that are supported by the data source on a capability */
/* Application gets these through DG_CONTROL/DAT_CAPABILITY/MSG_QUERYSUPPORT */

/****************************************************************************
 * Depreciated Items                                                        *
 ****************************************************************************/
#if defined(WIN32) || defined(WIN64)
        #define TW_HUGE
#elif !defined(TWH_CMP_GNU)
        #define TW_HUGE    huge
#else
        #define TW_HUGE
#endif

typedef BYTE TW_HUGE * HPBYTE;
typedef void TW_HUGE * HPVOID;

typedef unsigned char     TW_STR1024[1026],   FAR *pTW_STR1026, FAR *pTW_STR1024;
typedef wchar_t           TW_UNI512[512],     FAR *pTW_UNI512;









/* CAP_FILESYSTEM values (FS_ means file system) */

/* ICAP_PIXELTYPE values (PT_ means Pixel Type) */

/* ICAP_SUPPORTEDSIZES values (SS_ means Supported Sizes) */

/* ACAP_AUDIOFILEFORMAT values (AF_ means audio format).  Added 1.8 */

/* CAP_CLEARBUFFERS values */

*/
