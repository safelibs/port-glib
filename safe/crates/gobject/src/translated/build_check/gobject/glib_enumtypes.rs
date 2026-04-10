// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
extern "C" {
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_enum_register_static(name: *const gchar, const_static_values: *const GEnumValue) -> GType;
}
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_UNICODE_SPACE_SEPARATOR: C2RustUnnamed = 29;
pub const G_UNICODE_PARAGRAPH_SEPARATOR: C2RustUnnamed = 28;
pub const G_UNICODE_LINE_SEPARATOR: C2RustUnnamed = 27;
pub const G_UNICODE_OTHER_SYMBOL: C2RustUnnamed = 26;
pub const G_UNICODE_MATH_SYMBOL: C2RustUnnamed = 25;
pub const G_UNICODE_MODIFIER_SYMBOL: C2RustUnnamed = 24;
pub const G_UNICODE_CURRENCY_SYMBOL: C2RustUnnamed = 23;
pub const G_UNICODE_OPEN_PUNCTUATION: C2RustUnnamed = 22;
pub const G_UNICODE_OTHER_PUNCTUATION: C2RustUnnamed = 21;
pub const G_UNICODE_INITIAL_PUNCTUATION: C2RustUnnamed = 20;
pub const G_UNICODE_FINAL_PUNCTUATION: C2RustUnnamed = 19;
pub const G_UNICODE_CLOSE_PUNCTUATION: C2RustUnnamed = 18;
pub const G_UNICODE_DASH_PUNCTUATION: C2RustUnnamed = 17;
pub const G_UNICODE_CONNECT_PUNCTUATION: C2RustUnnamed = 16;
pub const G_UNICODE_OTHER_NUMBER: C2RustUnnamed = 15;
pub const G_UNICODE_LETTER_NUMBER: C2RustUnnamed = 14;
pub const G_UNICODE_DECIMAL_NUMBER: C2RustUnnamed = 13;
pub const G_UNICODE_NON_SPACING_MARK: C2RustUnnamed = 12;
pub const G_UNICODE_ENCLOSING_MARK: C2RustUnnamed = 11;
pub const G_UNICODE_SPACING_MARK: C2RustUnnamed = 10;
pub const G_UNICODE_UPPERCASE_LETTER: C2RustUnnamed = 9;
pub const G_UNICODE_TITLECASE_LETTER: C2RustUnnamed = 8;
pub const G_UNICODE_OTHER_LETTER: C2RustUnnamed = 7;
pub const G_UNICODE_MODIFIER_LETTER: C2RustUnnamed = 6;
pub const G_UNICODE_LOWERCASE_LETTER: C2RustUnnamed = 5;
pub const G_UNICODE_SURROGATE: C2RustUnnamed = 4;
pub const G_UNICODE_PRIVATE_USE: C2RustUnnamed = 3;
pub const G_UNICODE_UNASSIGNED: C2RustUnnamed = 2;
pub const G_UNICODE_FORMAT: C2RustUnnamed = 1;
pub const G_UNICODE_CONTROL: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_UNICODE_BREAK_VIRAMA: C2RustUnnamed_0 = 47;
pub const G_UNICODE_BREAK_VIRAMA_FINAL: C2RustUnnamed_0 = 46;
pub const G_UNICODE_BREAK_AKSARA_START: C2RustUnnamed_0 = 45;
pub const G_UNICODE_BREAK_AKSARA_PRE_BASE: C2RustUnnamed_0 = 44;
pub const G_UNICODE_BREAK_AKSARA: C2RustUnnamed_0 = 43;
pub const G_UNICODE_BREAK_ZERO_WIDTH_JOINER: C2RustUnnamed_0 = 42;
pub const G_UNICODE_BREAK_EMOJI_MODIFIER: C2RustUnnamed_0 = 41;
pub const G_UNICODE_BREAK_EMOJI_BASE: C2RustUnnamed_0 = 40;
pub const G_UNICODE_BREAK_REGIONAL_INDICATOR: C2RustUnnamed_0 = 39;
pub const G_UNICODE_BREAK_HEBREW_LETTER: C2RustUnnamed_0 = 38;
pub const G_UNICODE_BREAK_CONDITIONAL_JAPANESE_STARTER: C2RustUnnamed_0 = 37;
pub const G_UNICODE_BREAK_CLOSE_PARENTHESIS: C2RustUnnamed_0 = 36;
pub const G_UNICODE_BREAK_CLOSE_PARANTHESIS: C2RustUnnamed_0 = 36;
pub const G_UNICODE_BREAK_HANGUL_LVT_SYLLABLE: C2RustUnnamed_0 = 35;
pub const G_UNICODE_BREAK_HANGUL_LV_SYLLABLE: C2RustUnnamed_0 = 34;
pub const G_UNICODE_BREAK_HANGUL_T_JAMO: C2RustUnnamed_0 = 33;
pub const G_UNICODE_BREAK_HANGUL_V_JAMO: C2RustUnnamed_0 = 32;
pub const G_UNICODE_BREAK_HANGUL_L_JAMO: C2RustUnnamed_0 = 31;
pub const G_UNICODE_BREAK_WORD_JOINER: C2RustUnnamed_0 = 30;
pub const G_UNICODE_BREAK_NEXT_LINE: C2RustUnnamed_0 = 29;
pub const G_UNICODE_BREAK_UNKNOWN: C2RustUnnamed_0 = 28;
pub const G_UNICODE_BREAK_AMBIGUOUS: C2RustUnnamed_0 = 27;
pub const G_UNICODE_BREAK_COMPLEX_CONTEXT: C2RustUnnamed_0 = 26;
pub const G_UNICODE_BREAK_POSTFIX: C2RustUnnamed_0 = 25;
pub const G_UNICODE_BREAK_PREFIX: C2RustUnnamed_0 = 24;
pub const G_UNICODE_BREAK_ALPHABETIC: C2RustUnnamed_0 = 23;
pub const G_UNICODE_BREAK_SYMBOL: C2RustUnnamed_0 = 22;
pub const G_UNICODE_BREAK_INFIX_SEPARATOR: C2RustUnnamed_0 = 21;
pub const G_UNICODE_BREAK_NUMERIC: C2RustUnnamed_0 = 20;
pub const G_UNICODE_BREAK_IDEOGRAPHIC: C2RustUnnamed_0 = 19;
pub const G_UNICODE_BREAK_EXCLAMATION: C2RustUnnamed_0 = 18;
pub const G_UNICODE_BREAK_QUOTATION: C2RustUnnamed_0 = 17;
pub const G_UNICODE_BREAK_CLOSE_PUNCTUATION: C2RustUnnamed_0 = 16;
pub const G_UNICODE_BREAK_OPEN_PUNCTUATION: C2RustUnnamed_0 = 15;
pub const G_UNICODE_BREAK_NON_STARTER: C2RustUnnamed_0 = 14;
pub const G_UNICODE_BREAK_HYPHEN: C2RustUnnamed_0 = 13;
pub const G_UNICODE_BREAK_BEFORE_AND_AFTER: C2RustUnnamed_0 = 12;
pub const G_UNICODE_BREAK_BEFORE: C2RustUnnamed_0 = 11;
pub const G_UNICODE_BREAK_AFTER: C2RustUnnamed_0 = 10;
pub const G_UNICODE_BREAK_SPACE: C2RustUnnamed_0 = 9;
pub const G_UNICODE_BREAK_CONTINGENT: C2RustUnnamed_0 = 8;
pub const G_UNICODE_BREAK_NON_BREAKING_GLUE: C2RustUnnamed_0 = 7;
pub const G_UNICODE_BREAK_INSEPARABLE: C2RustUnnamed_0 = 6;
pub const G_UNICODE_BREAK_ZERO_WIDTH_SPACE: C2RustUnnamed_0 = 5;
pub const G_UNICODE_BREAK_SURROGATE: C2RustUnnamed_0 = 4;
pub const G_UNICODE_BREAK_COMBINING_MARK: C2RustUnnamed_0 = 3;
pub const G_UNICODE_BREAK_LINE_FEED: C2RustUnnamed_0 = 2;
pub const G_UNICODE_BREAK_CARRIAGE_RETURN: C2RustUnnamed_0 = 1;
pub const G_UNICODE_BREAK_MANDATORY: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_int;
pub const G_UNICODE_SCRIPT_NAG_MUNDARI: C2RustUnnamed_1 = 164;
pub const G_UNICODE_SCRIPT_KAWI: C2RustUnnamed_1 = 163;
pub const G_UNICODE_SCRIPT_MATH: C2RustUnnamed_1 = 162;
pub const G_UNICODE_SCRIPT_VITHKUQI: C2RustUnnamed_1 = 161;
pub const G_UNICODE_SCRIPT_TOTO: C2RustUnnamed_1 = 160;
pub const G_UNICODE_SCRIPT_TANGSA: C2RustUnnamed_1 = 159;
pub const G_UNICODE_SCRIPT_OLD_UYGHUR: C2RustUnnamed_1 = 158;
pub const G_UNICODE_SCRIPT_CYPRO_MINOAN: C2RustUnnamed_1 = 157;
pub const G_UNICODE_SCRIPT_YEZIDI: C2RustUnnamed_1 = 156;
pub const G_UNICODE_SCRIPT_KHITAN_SMALL_SCRIPT: C2RustUnnamed_1 = 155;
pub const G_UNICODE_SCRIPT_DIVES_AKURU: C2RustUnnamed_1 = 154;
pub const G_UNICODE_SCRIPT_CHORASMIAN: C2RustUnnamed_1 = 153;
pub const G_UNICODE_SCRIPT_WANCHO: C2RustUnnamed_1 = 152;
pub const G_UNICODE_SCRIPT_NYIAKENG_PUACHUE_HMONG: C2RustUnnamed_1 = 151;
pub const G_UNICODE_SCRIPT_NANDINAGARI: C2RustUnnamed_1 = 150;
pub const G_UNICODE_SCRIPT_ELYMAIC: C2RustUnnamed_1 = 149;
pub const G_UNICODE_SCRIPT_SOGDIAN: C2RustUnnamed_1 = 148;
pub const G_UNICODE_SCRIPT_OLD_SOGDIAN: C2RustUnnamed_1 = 147;
pub const G_UNICODE_SCRIPT_MEDEFAIDRIN: C2RustUnnamed_1 = 146;
pub const G_UNICODE_SCRIPT_MAKASAR: C2RustUnnamed_1 = 145;
pub const G_UNICODE_SCRIPT_HANIFI_ROHINGYA: C2RustUnnamed_1 = 144;
pub const G_UNICODE_SCRIPT_GUNJALA_GONDI: C2RustUnnamed_1 = 143;
pub const G_UNICODE_SCRIPT_DOGRA: C2RustUnnamed_1 = 142;
pub const G_UNICODE_SCRIPT_ZANABAZAR_SQUARE: C2RustUnnamed_1 = 141;
pub const G_UNICODE_SCRIPT_SOYOMBO: C2RustUnnamed_1 = 140;
pub const G_UNICODE_SCRIPT_NUSHU: C2RustUnnamed_1 = 139;
pub const G_UNICODE_SCRIPT_MASARAM_GONDI: C2RustUnnamed_1 = 138;
pub const G_UNICODE_SCRIPT_TANGUT: C2RustUnnamed_1 = 137;
pub const G_UNICODE_SCRIPT_OSAGE: C2RustUnnamed_1 = 136;
pub const G_UNICODE_SCRIPT_NEWA: C2RustUnnamed_1 = 135;
pub const G_UNICODE_SCRIPT_MARCHEN: C2RustUnnamed_1 = 134;
pub const G_UNICODE_SCRIPT_BHAIKSUKI: C2RustUnnamed_1 = 133;
pub const G_UNICODE_SCRIPT_ADLAM: C2RustUnnamed_1 = 132;
pub const G_UNICODE_SCRIPT_SIGNWRITING: C2RustUnnamed_1 = 131;
pub const G_UNICODE_SCRIPT_OLD_HUNGARIAN: C2RustUnnamed_1 = 130;
pub const G_UNICODE_SCRIPT_MULTANI: C2RustUnnamed_1 = 129;
pub const G_UNICODE_SCRIPT_HATRAN: C2RustUnnamed_1 = 128;
pub const G_UNICODE_SCRIPT_ANATOLIAN_HIEROGLYPHS: C2RustUnnamed_1 = 127;
pub const G_UNICODE_SCRIPT_AHOM: C2RustUnnamed_1 = 126;
pub const G_UNICODE_SCRIPT_WARANG_CITI: C2RustUnnamed_1 = 125;
pub const G_UNICODE_SCRIPT_TIRHUTA: C2RustUnnamed_1 = 124;
pub const G_UNICODE_SCRIPT_SIDDHAM: C2RustUnnamed_1 = 123;
pub const G_UNICODE_SCRIPT_PSALTER_PAHLAVI: C2RustUnnamed_1 = 122;
pub const G_UNICODE_SCRIPT_PAU_CIN_HAU: C2RustUnnamed_1 = 121;
pub const G_UNICODE_SCRIPT_PALMYRENE: C2RustUnnamed_1 = 120;
pub const G_UNICODE_SCRIPT_PAHAWH_HMONG: C2RustUnnamed_1 = 119;
pub const G_UNICODE_SCRIPT_OLD_PERMIC: C2RustUnnamed_1 = 118;
pub const G_UNICODE_SCRIPT_OLD_NORTH_ARABIAN: C2RustUnnamed_1 = 117;
pub const G_UNICODE_SCRIPT_NABATAEAN: C2RustUnnamed_1 = 116;
pub const G_UNICODE_SCRIPT_MRO: C2RustUnnamed_1 = 115;
pub const G_UNICODE_SCRIPT_MODI: C2RustUnnamed_1 = 114;
pub const G_UNICODE_SCRIPT_MENDE_KIKAKUI: C2RustUnnamed_1 = 113;
pub const G_UNICODE_SCRIPT_MANICHAEAN: C2RustUnnamed_1 = 112;
pub const G_UNICODE_SCRIPT_MAHAJANI: C2RustUnnamed_1 = 111;
pub const G_UNICODE_SCRIPT_LINEAR_A: C2RustUnnamed_1 = 110;
pub const G_UNICODE_SCRIPT_KHUDAWADI: C2RustUnnamed_1 = 109;
pub const G_UNICODE_SCRIPT_KHOJKI: C2RustUnnamed_1 = 108;
pub const G_UNICODE_SCRIPT_GRANTHA: C2RustUnnamed_1 = 107;
pub const G_UNICODE_SCRIPT_ELBASAN: C2RustUnnamed_1 = 106;
pub const G_UNICODE_SCRIPT_DUPLOYAN: C2RustUnnamed_1 = 105;
pub const G_UNICODE_SCRIPT_CAUCASIAN_ALBANIAN: C2RustUnnamed_1 = 104;
pub const G_UNICODE_SCRIPT_BASSA_VAH: C2RustUnnamed_1 = 103;
pub const G_UNICODE_SCRIPT_TAKRI: C2RustUnnamed_1 = 102;
pub const G_UNICODE_SCRIPT_SORA_SOMPENG: C2RustUnnamed_1 = 101;
pub const G_UNICODE_SCRIPT_SHARADA: C2RustUnnamed_1 = 100;
pub const G_UNICODE_SCRIPT_MIAO: C2RustUnnamed_1 = 99;
pub const G_UNICODE_SCRIPT_MEROITIC_HIEROGLYPHS: C2RustUnnamed_1 = 98;
pub const G_UNICODE_SCRIPT_MEROITIC_CURSIVE: C2RustUnnamed_1 = 97;
pub const G_UNICODE_SCRIPT_CHAKMA: C2RustUnnamed_1 = 96;
pub const G_UNICODE_SCRIPT_MANDAIC: C2RustUnnamed_1 = 95;
pub const G_UNICODE_SCRIPT_BRAHMI: C2RustUnnamed_1 = 94;
pub const G_UNICODE_SCRIPT_BATAK: C2RustUnnamed_1 = 93;
pub const G_UNICODE_SCRIPT_TAI_VIET: C2RustUnnamed_1 = 92;
pub const G_UNICODE_SCRIPT_TAI_THAM: C2RustUnnamed_1 = 91;
pub const G_UNICODE_SCRIPT_SAMARITAN: C2RustUnnamed_1 = 90;
pub const G_UNICODE_SCRIPT_OLD_TURKIC: C2RustUnnamed_1 = 89;
pub const G_UNICODE_SCRIPT_OLD_SOUTH_ARABIAN: C2RustUnnamed_1 = 88;
pub const G_UNICODE_SCRIPT_MEETEI_MAYEK: C2RustUnnamed_1 = 87;
pub const G_UNICODE_SCRIPT_LISU: C2RustUnnamed_1 = 86;
pub const G_UNICODE_SCRIPT_KAITHI: C2RustUnnamed_1 = 85;
pub const G_UNICODE_SCRIPT_JAVANESE: C2RustUnnamed_1 = 84;
pub const G_UNICODE_SCRIPT_INSCRIPTIONAL_PARTHIAN: C2RustUnnamed_1 = 83;
pub const G_UNICODE_SCRIPT_INSCRIPTIONAL_PAHLAVI: C2RustUnnamed_1 = 82;
pub const G_UNICODE_SCRIPT_IMPERIAL_ARAMAIC: C2RustUnnamed_1 = 81;
pub const G_UNICODE_SCRIPT_EGYPTIAN_HIEROGLYPHS: C2RustUnnamed_1 = 80;
pub const G_UNICODE_SCRIPT_BAMUM: C2RustUnnamed_1 = 79;
pub const G_UNICODE_SCRIPT_AVESTAN: C2RustUnnamed_1 = 78;
pub const G_UNICODE_SCRIPT_LYDIAN: C2RustUnnamed_1 = 77;
pub const G_UNICODE_SCRIPT_LYCIAN: C2RustUnnamed_1 = 76;
pub const G_UNICODE_SCRIPT_CARIAN: C2RustUnnamed_1 = 75;
pub const G_UNICODE_SCRIPT_VAI: C2RustUnnamed_1 = 74;
pub const G_UNICODE_SCRIPT_OL_CHIKI: C2RustUnnamed_1 = 73;
pub const G_UNICODE_SCRIPT_CHAM: C2RustUnnamed_1 = 72;
pub const G_UNICODE_SCRIPT_SAURASHTRA: C2RustUnnamed_1 = 71;
pub const G_UNICODE_SCRIPT_SUNDANESE: C2RustUnnamed_1 = 70;
pub const G_UNICODE_SCRIPT_REJANG: C2RustUnnamed_1 = 69;
pub const G_UNICODE_SCRIPT_LEPCHA: C2RustUnnamed_1 = 68;
pub const G_UNICODE_SCRIPT_KAYAH_LI: C2RustUnnamed_1 = 67;
pub const G_UNICODE_SCRIPT_NKO: C2RustUnnamed_1 = 66;
pub const G_UNICODE_SCRIPT_PHAGS_PA: C2RustUnnamed_1 = 65;
pub const G_UNICODE_SCRIPT_PHOENICIAN: C2RustUnnamed_1 = 64;
pub const G_UNICODE_SCRIPT_CUNEIFORM: C2RustUnnamed_1 = 63;
pub const G_UNICODE_SCRIPT_BALINESE: C2RustUnnamed_1 = 62;
pub const G_UNICODE_SCRIPT_UNKNOWN: C2RustUnnamed_1 = 61;
pub const G_UNICODE_SCRIPT_KHAROSHTHI: C2RustUnnamed_1 = 60;
pub const G_UNICODE_SCRIPT_OLD_PERSIAN: C2RustUnnamed_1 = 59;
pub const G_UNICODE_SCRIPT_SYLOTI_NAGRI: C2RustUnnamed_1 = 58;
pub const G_UNICODE_SCRIPT_TIFINAGH: C2RustUnnamed_1 = 57;
pub const G_UNICODE_SCRIPT_GLAGOLITIC: C2RustUnnamed_1 = 56;
pub const G_UNICODE_SCRIPT_BUGINESE: C2RustUnnamed_1 = 55;
pub const G_UNICODE_SCRIPT_NEW_TAI_LUE: C2RustUnnamed_1 = 54;
pub const G_UNICODE_SCRIPT_UGARITIC: C2RustUnnamed_1 = 53;
pub const G_UNICODE_SCRIPT_TAI_LE: C2RustUnnamed_1 = 52;
pub const G_UNICODE_SCRIPT_LINEAR_B: C2RustUnnamed_1 = 51;
pub const G_UNICODE_SCRIPT_SHAVIAN: C2RustUnnamed_1 = 50;
pub const G_UNICODE_SCRIPT_OSMANYA: C2RustUnnamed_1 = 49;
pub const G_UNICODE_SCRIPT_LIMBU: C2RustUnnamed_1 = 48;
pub const G_UNICODE_SCRIPT_CYPRIOT: C2RustUnnamed_1 = 47;
pub const G_UNICODE_SCRIPT_BRAILLE: C2RustUnnamed_1 = 46;
pub const G_UNICODE_SCRIPT_TAGBANWA: C2RustUnnamed_1 = 45;
pub const G_UNICODE_SCRIPT_BUHID: C2RustUnnamed_1 = 44;
pub const G_UNICODE_SCRIPT_HANUNOO: C2RustUnnamed_1 = 43;
pub const G_UNICODE_SCRIPT_TAGALOG: C2RustUnnamed_1 = 42;
pub const G_UNICODE_SCRIPT_YI: C2RustUnnamed_1 = 41;
pub const G_UNICODE_SCRIPT_CANADIAN_ABORIGINAL: C2RustUnnamed_1 = 40;
pub const G_UNICODE_SCRIPT_TIBETAN: C2RustUnnamed_1 = 39;
pub const G_UNICODE_SCRIPT_THAI: C2RustUnnamed_1 = 38;
pub const G_UNICODE_SCRIPT_THAANA: C2RustUnnamed_1 = 37;
pub const G_UNICODE_SCRIPT_TELUGU: C2RustUnnamed_1 = 36;
pub const G_UNICODE_SCRIPT_TAMIL: C2RustUnnamed_1 = 35;
pub const G_UNICODE_SCRIPT_SYRIAC: C2RustUnnamed_1 = 34;
pub const G_UNICODE_SCRIPT_SINHALA: C2RustUnnamed_1 = 33;
pub const G_UNICODE_SCRIPT_RUNIC: C2RustUnnamed_1 = 32;
pub const G_UNICODE_SCRIPT_ORIYA: C2RustUnnamed_1 = 31;
pub const G_UNICODE_SCRIPT_OLD_ITALIC: C2RustUnnamed_1 = 30;
pub const G_UNICODE_SCRIPT_OGHAM: C2RustUnnamed_1 = 29;
pub const G_UNICODE_SCRIPT_MYANMAR: C2RustUnnamed_1 = 28;
pub const G_UNICODE_SCRIPT_MONGOLIAN: C2RustUnnamed_1 = 27;
pub const G_UNICODE_SCRIPT_MALAYALAM: C2RustUnnamed_1 = 26;
pub const G_UNICODE_SCRIPT_LATIN: C2RustUnnamed_1 = 25;
pub const G_UNICODE_SCRIPT_LAO: C2RustUnnamed_1 = 24;
pub const G_UNICODE_SCRIPT_KHMER: C2RustUnnamed_1 = 23;
pub const G_UNICODE_SCRIPT_KATAKANA: C2RustUnnamed_1 = 22;
pub const G_UNICODE_SCRIPT_KANNADA: C2RustUnnamed_1 = 21;
pub const G_UNICODE_SCRIPT_HIRAGANA: C2RustUnnamed_1 = 20;
pub const G_UNICODE_SCRIPT_HEBREW: C2RustUnnamed_1 = 19;
pub const G_UNICODE_SCRIPT_HANGUL: C2RustUnnamed_1 = 18;
pub const G_UNICODE_SCRIPT_HAN: C2RustUnnamed_1 = 17;
pub const G_UNICODE_SCRIPT_GURMUKHI: C2RustUnnamed_1 = 16;
pub const G_UNICODE_SCRIPT_GUJARATI: C2RustUnnamed_1 = 15;
pub const G_UNICODE_SCRIPT_GREEK: C2RustUnnamed_1 = 14;
pub const G_UNICODE_SCRIPT_GOTHIC: C2RustUnnamed_1 = 13;
pub const G_UNICODE_SCRIPT_GEORGIAN: C2RustUnnamed_1 = 12;
pub const G_UNICODE_SCRIPT_ETHIOPIC: C2RustUnnamed_1 = 11;
pub const G_UNICODE_SCRIPT_DEVANAGARI: C2RustUnnamed_1 = 10;
pub const G_UNICODE_SCRIPT_DESERET: C2RustUnnamed_1 = 9;
pub const G_UNICODE_SCRIPT_CYRILLIC: C2RustUnnamed_1 = 8;
pub const G_UNICODE_SCRIPT_COPTIC: C2RustUnnamed_1 = 7;
pub const G_UNICODE_SCRIPT_CHEROKEE: C2RustUnnamed_1 = 6;
pub const G_UNICODE_SCRIPT_BOPOMOFO: C2RustUnnamed_1 = 5;
pub const G_UNICODE_SCRIPT_BENGALI: C2RustUnnamed_1 = 4;
pub const G_UNICODE_SCRIPT_ARMENIAN: C2RustUnnamed_1 = 3;
pub const G_UNICODE_SCRIPT_ARABIC: C2RustUnnamed_1 = 2;
pub const G_UNICODE_SCRIPT_INHERITED: C2RustUnnamed_1 = 1;
pub const G_UNICODE_SCRIPT_COMMON: C2RustUnnamed_1 = 0;
pub const G_UNICODE_SCRIPT_INVALID_CODE: C2RustUnnamed_1 = -1;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const G_NORMALIZE_NFKC: C2RustUnnamed_2 = 3;
pub const G_NORMALIZE_ALL_COMPOSE: C2RustUnnamed_2 = 3;
pub const G_NORMALIZE_NFKD: C2RustUnnamed_2 = 2;
pub const G_NORMALIZE_ALL: C2RustUnnamed_2 = 2;
pub const G_NORMALIZE_NFC: C2RustUnnamed_2 = 1;
pub const G_NORMALIZE_DEFAULT_COMPOSE: C2RustUnnamed_2 = 1;
pub const G_NORMALIZE_NFD: C2RustUnnamed_2 = 0;
pub const G_NORMALIZE_DEFAULT: C2RustUnnamed_2 = 0;
pub type GType = gsize;
pub type GEnumValue = _GEnumValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumValue {
    pub value: gint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
#[no_mangle]
pub unsafe extern "C" fn g_unicode_type_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        static mut values: [GEnumValue; 31] = [
            _GEnumValue {
                value: G_UNICODE_CONTROL as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_CONTROL\0" as *const u8 as *const gchar,
                value_nick: b"control\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_FORMAT as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_FORMAT\0" as *const u8 as *const gchar,
                value_nick: b"format\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_UNASSIGNED as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_UNASSIGNED\0" as *const u8 as *const gchar,
                value_nick: b"unassigned\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_PRIVATE_USE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_PRIVATE_USE\0" as *const u8 as *const gchar,
                value_nick: b"private-use\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SURROGATE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SURROGATE\0" as *const u8 as *const gchar,
                value_nick: b"surrogate\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_LOWERCASE_LETTER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_LOWERCASE_LETTER\0" as *const u8 as *const gchar,
                value_nick: b"lowercase-letter\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_MODIFIER_LETTER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_MODIFIER_LETTER\0" as *const u8 as *const gchar,
                value_nick: b"modifier-letter\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_OTHER_LETTER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_OTHER_LETTER\0" as *const u8 as *const gchar,
                value_nick: b"other-letter\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_TITLECASE_LETTER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_TITLECASE_LETTER\0" as *const u8 as *const gchar,
                value_nick: b"titlecase-letter\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_UPPERCASE_LETTER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_UPPERCASE_LETTER\0" as *const u8 as *const gchar,
                value_nick: b"uppercase-letter\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SPACING_MARK as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SPACING_MARK\0" as *const u8 as *const gchar,
                value_nick: b"spacing-mark\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_ENCLOSING_MARK as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_ENCLOSING_MARK\0" as *const u8 as *const gchar,
                value_nick: b"enclosing-mark\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_NON_SPACING_MARK as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_NON_SPACING_MARK\0" as *const u8 as *const gchar,
                value_nick: b"non-spacing-mark\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_DECIMAL_NUMBER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_DECIMAL_NUMBER\0" as *const u8 as *const gchar,
                value_nick: b"decimal-number\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_LETTER_NUMBER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_LETTER_NUMBER\0" as *const u8 as *const gchar,
                value_nick: b"letter-number\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_OTHER_NUMBER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_OTHER_NUMBER\0" as *const u8 as *const gchar,
                value_nick: b"other-number\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_CONNECT_PUNCTUATION as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_CONNECT_PUNCTUATION\0" as *const u8 as *const gchar,
                value_nick: b"connect-punctuation\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_DASH_PUNCTUATION as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_DASH_PUNCTUATION\0" as *const u8 as *const gchar,
                value_nick: b"dash-punctuation\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_CLOSE_PUNCTUATION as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_CLOSE_PUNCTUATION\0" as *const u8 as *const gchar,
                value_nick: b"close-punctuation\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_FINAL_PUNCTUATION as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_FINAL_PUNCTUATION\0" as *const u8 as *const gchar,
                value_nick: b"final-punctuation\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_INITIAL_PUNCTUATION as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_INITIAL_PUNCTUATION\0" as *const u8 as *const gchar,
                value_nick: b"initial-punctuation\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_OTHER_PUNCTUATION as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_OTHER_PUNCTUATION\0" as *const u8 as *const gchar,
                value_nick: b"other-punctuation\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_OPEN_PUNCTUATION as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_OPEN_PUNCTUATION\0" as *const u8 as *const gchar,
                value_nick: b"open-punctuation\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_CURRENCY_SYMBOL as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_CURRENCY_SYMBOL\0" as *const u8 as *const gchar,
                value_nick: b"currency-symbol\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_MODIFIER_SYMBOL as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_MODIFIER_SYMBOL\0" as *const u8 as *const gchar,
                value_nick: b"modifier-symbol\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_MATH_SYMBOL as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_MATH_SYMBOL\0" as *const u8 as *const gchar,
                value_nick: b"math-symbol\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_OTHER_SYMBOL as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_OTHER_SYMBOL\0" as *const u8 as *const gchar,
                value_nick: b"other-symbol\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_LINE_SEPARATOR as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_LINE_SEPARATOR\0" as *const u8 as *const gchar,
                value_nick: b"line-separator\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_PARAGRAPH_SEPARATOR as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_PARAGRAPH_SEPARATOR\0" as *const u8 as *const gchar,
                value_nick: b"paragraph-separator\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SPACE_SEPARATOR as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SPACE_SEPARATOR\0" as *const u8 as *const gchar,
                value_nick: b"space-separator\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GUnicodeType\0" as *const u8 as *const gchar),
            &raw const values as *const GEnumValue,
        );
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_unicode_break_type_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        static mut values: [GEnumValue; 50] = [
            _GEnumValue {
                value: G_UNICODE_BREAK_MANDATORY as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_MANDATORY\0" as *const u8 as *const gchar,
                value_nick: b"mandatory\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_CARRIAGE_RETURN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_CARRIAGE_RETURN\0" as *const u8 as *const gchar,
                value_nick: b"carriage-return\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_LINE_FEED as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_LINE_FEED\0" as *const u8 as *const gchar,
                value_nick: b"line-feed\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_COMBINING_MARK as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_COMBINING_MARK\0" as *const u8 as *const gchar,
                value_nick: b"combining-mark\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_SURROGATE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_SURROGATE\0" as *const u8 as *const gchar,
                value_nick: b"surrogate\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_ZERO_WIDTH_SPACE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_ZERO_WIDTH_SPACE\0" as *const u8 as *const gchar,
                value_nick: b"zero-width-space\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_INSEPARABLE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_INSEPARABLE\0" as *const u8 as *const gchar,
                value_nick: b"inseparable\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_NON_BREAKING_GLUE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_NON_BREAKING_GLUE\0" as *const u8 as *const gchar,
                value_nick: b"non-breaking-glue\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_CONTINGENT as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_CONTINGENT\0" as *const u8 as *const gchar,
                value_nick: b"contingent\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_SPACE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_SPACE\0" as *const u8 as *const gchar,
                value_nick: b"space\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_AFTER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_AFTER\0" as *const u8 as *const gchar,
                value_nick: b"after\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_BEFORE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_BEFORE\0" as *const u8 as *const gchar,
                value_nick: b"before\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_BEFORE_AND_AFTER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_BEFORE_AND_AFTER\0" as *const u8 as *const gchar,
                value_nick: b"before-and-after\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_HYPHEN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_HYPHEN\0" as *const u8 as *const gchar,
                value_nick: b"hyphen\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_NON_STARTER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_NON_STARTER\0" as *const u8 as *const gchar,
                value_nick: b"non-starter\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_OPEN_PUNCTUATION as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_OPEN_PUNCTUATION\0" as *const u8 as *const gchar,
                value_nick: b"open-punctuation\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_CLOSE_PUNCTUATION as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_CLOSE_PUNCTUATION\0" as *const u8 as *const gchar,
                value_nick: b"close-punctuation\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_QUOTATION as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_QUOTATION\0" as *const u8 as *const gchar,
                value_nick: b"quotation\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_EXCLAMATION as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_EXCLAMATION\0" as *const u8 as *const gchar,
                value_nick: b"exclamation\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_IDEOGRAPHIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_IDEOGRAPHIC\0" as *const u8 as *const gchar,
                value_nick: b"ideographic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_NUMERIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_NUMERIC\0" as *const u8 as *const gchar,
                value_nick: b"numeric\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_INFIX_SEPARATOR as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_INFIX_SEPARATOR\0" as *const u8 as *const gchar,
                value_nick: b"infix-separator\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_SYMBOL as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_SYMBOL\0" as *const u8 as *const gchar,
                value_nick: b"symbol\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_ALPHABETIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_ALPHABETIC\0" as *const u8 as *const gchar,
                value_nick: b"alphabetic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_PREFIX as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_PREFIX\0" as *const u8 as *const gchar,
                value_nick: b"prefix\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_POSTFIX as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_POSTFIX\0" as *const u8 as *const gchar,
                value_nick: b"postfix\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_COMPLEX_CONTEXT as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_COMPLEX_CONTEXT\0" as *const u8 as *const gchar,
                value_nick: b"complex-context\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_AMBIGUOUS as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_AMBIGUOUS\0" as *const u8 as *const gchar,
                value_nick: b"ambiguous\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_UNKNOWN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_UNKNOWN\0" as *const u8 as *const gchar,
                value_nick: b"unknown\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_NEXT_LINE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_NEXT_LINE\0" as *const u8 as *const gchar,
                value_nick: b"next-line\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_WORD_JOINER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_WORD_JOINER\0" as *const u8 as *const gchar,
                value_nick: b"word-joiner\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_HANGUL_L_JAMO as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_HANGUL_L_JAMO\0" as *const u8 as *const gchar,
                value_nick: b"hangul-l-jamo\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_HANGUL_V_JAMO as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_HANGUL_V_JAMO\0" as *const u8 as *const gchar,
                value_nick: b"hangul-v-jamo\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_HANGUL_T_JAMO as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_HANGUL_T_JAMO\0" as *const u8 as *const gchar,
                value_nick: b"hangul-t-jamo\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_HANGUL_LV_SYLLABLE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_HANGUL_LV_SYLLABLE\0" as *const u8 as *const gchar,
                value_nick: b"hangul-lv-syllable\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_HANGUL_LVT_SYLLABLE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_HANGUL_LVT_SYLLABLE\0" as *const u8 as *const gchar,
                value_nick: b"hangul-lvt-syllable\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_CLOSE_PARANTHESIS as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_CLOSE_PARANTHESIS\0" as *const u8 as *const gchar,
                value_nick: b"close-paranthesis\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_CLOSE_PARENTHESIS as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_CLOSE_PARENTHESIS\0" as *const u8 as *const gchar,
                value_nick: b"close-parenthesis\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_CONDITIONAL_JAPANESE_STARTER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_CONDITIONAL_JAPANESE_STARTER\0" as *const u8
                    as *const gchar,
                value_nick: b"conditional-japanese-starter\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_HEBREW_LETTER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_HEBREW_LETTER\0" as *const u8 as *const gchar,
                value_nick: b"hebrew-letter\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_REGIONAL_INDICATOR as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_REGIONAL_INDICATOR\0" as *const u8 as *const gchar,
                value_nick: b"regional-indicator\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_EMOJI_BASE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_EMOJI_BASE\0" as *const u8 as *const gchar,
                value_nick: b"emoji-base\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_EMOJI_MODIFIER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_EMOJI_MODIFIER\0" as *const u8 as *const gchar,
                value_nick: b"emoji-modifier\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_ZERO_WIDTH_JOINER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_ZERO_WIDTH_JOINER\0" as *const u8 as *const gchar,
                value_nick: b"zero-width-joiner\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_AKSARA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_AKSARA\0" as *const u8 as *const gchar,
                value_nick: b"aksara\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_AKSARA_PRE_BASE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_AKSARA_PRE_BASE\0" as *const u8 as *const gchar,
                value_nick: b"aksara-pre-base\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_AKSARA_START as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_AKSARA_START\0" as *const u8 as *const gchar,
                value_nick: b"aksara-start\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_VIRAMA_FINAL as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_VIRAMA_FINAL\0" as *const u8 as *const gchar,
                value_nick: b"virama-final\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_BREAK_VIRAMA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_BREAK_VIRAMA\0" as *const u8 as *const gchar,
                value_nick: b"virama\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GUnicodeBreakType\0" as *const u8 as *const gchar),
            &raw const values as *const GEnumValue,
        );
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_unicode_script_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        static mut values: [GEnumValue; 167] = [
            _GEnumValue {
                value: G_UNICODE_SCRIPT_INVALID_CODE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_INVALID_CODE\0" as *const u8 as *const gchar,
                value_nick: b"invalid-code\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_COMMON as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_COMMON\0" as *const u8 as *const gchar,
                value_nick: b"common\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_INHERITED as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_INHERITED\0" as *const u8 as *const gchar,
                value_nick: b"inherited\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_ARABIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_ARABIC\0" as *const u8 as *const gchar,
                value_nick: b"arabic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_ARMENIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_ARMENIAN\0" as *const u8 as *const gchar,
                value_nick: b"armenian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_BENGALI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_BENGALI\0" as *const u8 as *const gchar,
                value_nick: b"bengali\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_BOPOMOFO as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_BOPOMOFO\0" as *const u8 as *const gchar,
                value_nick: b"bopomofo\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_CHEROKEE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_CHEROKEE\0" as *const u8 as *const gchar,
                value_nick: b"cherokee\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_COPTIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_COPTIC\0" as *const u8 as *const gchar,
                value_nick: b"coptic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_CYRILLIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_CYRILLIC\0" as *const u8 as *const gchar,
                value_nick: b"cyrillic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_DESERET as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_DESERET\0" as *const u8 as *const gchar,
                value_nick: b"deseret\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_DEVANAGARI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_DEVANAGARI\0" as *const u8 as *const gchar,
                value_nick: b"devanagari\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_ETHIOPIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_ETHIOPIC\0" as *const u8 as *const gchar,
                value_nick: b"ethiopic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_GEORGIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_GEORGIAN\0" as *const u8 as *const gchar,
                value_nick: b"georgian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_GOTHIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_GOTHIC\0" as *const u8 as *const gchar,
                value_nick: b"gothic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_GREEK as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_GREEK\0" as *const u8 as *const gchar,
                value_nick: b"greek\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_GUJARATI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_GUJARATI\0" as *const u8 as *const gchar,
                value_nick: b"gujarati\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_GURMUKHI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_GURMUKHI\0" as *const u8 as *const gchar,
                value_nick: b"gurmukhi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_HAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_HAN\0" as *const u8 as *const gchar,
                value_nick: b"han\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_HANGUL as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_HANGUL\0" as *const u8 as *const gchar,
                value_nick: b"hangul\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_HEBREW as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_HEBREW\0" as *const u8 as *const gchar,
                value_nick: b"hebrew\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_HIRAGANA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_HIRAGANA\0" as *const u8 as *const gchar,
                value_nick: b"hiragana\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_KANNADA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_KANNADA\0" as *const u8 as *const gchar,
                value_nick: b"kannada\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_KATAKANA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_KATAKANA\0" as *const u8 as *const gchar,
                value_nick: b"katakana\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_KHMER as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_KHMER\0" as *const u8 as *const gchar,
                value_nick: b"khmer\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_LAO as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_LAO\0" as *const u8 as *const gchar,
                value_nick: b"lao\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_LATIN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_LATIN\0" as *const u8 as *const gchar,
                value_nick: b"latin\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MALAYALAM as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MALAYALAM\0" as *const u8 as *const gchar,
                value_nick: b"malayalam\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MONGOLIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MONGOLIAN\0" as *const u8 as *const gchar,
                value_nick: b"mongolian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MYANMAR as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MYANMAR\0" as *const u8 as *const gchar,
                value_nick: b"myanmar\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OGHAM as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OGHAM\0" as *const u8 as *const gchar,
                value_nick: b"ogham\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OLD_ITALIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OLD_ITALIC\0" as *const u8 as *const gchar,
                value_nick: b"old-italic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_ORIYA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_ORIYA\0" as *const u8 as *const gchar,
                value_nick: b"oriya\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_RUNIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_RUNIC\0" as *const u8 as *const gchar,
                value_nick: b"runic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SINHALA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SINHALA\0" as *const u8 as *const gchar,
                value_nick: b"sinhala\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SYRIAC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SYRIAC\0" as *const u8 as *const gchar,
                value_nick: b"syriac\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TAMIL as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TAMIL\0" as *const u8 as *const gchar,
                value_nick: b"tamil\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TELUGU as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TELUGU\0" as *const u8 as *const gchar,
                value_nick: b"telugu\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_THAANA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_THAANA\0" as *const u8 as *const gchar,
                value_nick: b"thaana\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_THAI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_THAI\0" as *const u8 as *const gchar,
                value_nick: b"thai\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TIBETAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TIBETAN\0" as *const u8 as *const gchar,
                value_nick: b"tibetan\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_CANADIAN_ABORIGINAL as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_CANADIAN_ABORIGINAL\0" as *const u8 as *const gchar,
                value_nick: b"canadian-aboriginal\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_YI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_YI\0" as *const u8 as *const gchar,
                value_nick: b"yi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TAGALOG as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TAGALOG\0" as *const u8 as *const gchar,
                value_nick: b"tagalog\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_HANUNOO as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_HANUNOO\0" as *const u8 as *const gchar,
                value_nick: b"hanunoo\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_BUHID as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_BUHID\0" as *const u8 as *const gchar,
                value_nick: b"buhid\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TAGBANWA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TAGBANWA\0" as *const u8 as *const gchar,
                value_nick: b"tagbanwa\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_BRAILLE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_BRAILLE\0" as *const u8 as *const gchar,
                value_nick: b"braille\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_CYPRIOT as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_CYPRIOT\0" as *const u8 as *const gchar,
                value_nick: b"cypriot\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_LIMBU as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_LIMBU\0" as *const u8 as *const gchar,
                value_nick: b"limbu\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OSMANYA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OSMANYA\0" as *const u8 as *const gchar,
                value_nick: b"osmanya\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SHAVIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SHAVIAN\0" as *const u8 as *const gchar,
                value_nick: b"shavian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_LINEAR_B as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_LINEAR_B\0" as *const u8 as *const gchar,
                value_nick: b"linear-b\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TAI_LE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TAI_LE\0" as *const u8 as *const gchar,
                value_nick: b"tai-le\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_UGARITIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_UGARITIC\0" as *const u8 as *const gchar,
                value_nick: b"ugaritic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_NEW_TAI_LUE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_NEW_TAI_LUE\0" as *const u8 as *const gchar,
                value_nick: b"new-tai-lue\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_BUGINESE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_BUGINESE\0" as *const u8 as *const gchar,
                value_nick: b"buginese\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_GLAGOLITIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_GLAGOLITIC\0" as *const u8 as *const gchar,
                value_nick: b"glagolitic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TIFINAGH as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TIFINAGH\0" as *const u8 as *const gchar,
                value_nick: b"tifinagh\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SYLOTI_NAGRI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SYLOTI_NAGRI\0" as *const u8 as *const gchar,
                value_nick: b"syloti-nagri\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OLD_PERSIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OLD_PERSIAN\0" as *const u8 as *const gchar,
                value_nick: b"old-persian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_KHAROSHTHI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_KHAROSHTHI\0" as *const u8 as *const gchar,
                value_nick: b"kharoshthi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_UNKNOWN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_UNKNOWN\0" as *const u8 as *const gchar,
                value_nick: b"unknown\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_BALINESE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_BALINESE\0" as *const u8 as *const gchar,
                value_nick: b"balinese\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_CUNEIFORM as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_CUNEIFORM\0" as *const u8 as *const gchar,
                value_nick: b"cuneiform\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_PHOENICIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_PHOENICIAN\0" as *const u8 as *const gchar,
                value_nick: b"phoenician\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_PHAGS_PA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_PHAGS_PA\0" as *const u8 as *const gchar,
                value_nick: b"phags-pa\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_NKO as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_NKO\0" as *const u8 as *const gchar,
                value_nick: b"nko\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_KAYAH_LI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_KAYAH_LI\0" as *const u8 as *const gchar,
                value_nick: b"kayah-li\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_LEPCHA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_LEPCHA\0" as *const u8 as *const gchar,
                value_nick: b"lepcha\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_REJANG as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_REJANG\0" as *const u8 as *const gchar,
                value_nick: b"rejang\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SUNDANESE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SUNDANESE\0" as *const u8 as *const gchar,
                value_nick: b"sundanese\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SAURASHTRA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SAURASHTRA\0" as *const u8 as *const gchar,
                value_nick: b"saurashtra\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_CHAM as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_CHAM\0" as *const u8 as *const gchar,
                value_nick: b"cham\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OL_CHIKI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OL_CHIKI\0" as *const u8 as *const gchar,
                value_nick: b"ol-chiki\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_VAI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_VAI\0" as *const u8 as *const gchar,
                value_nick: b"vai\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_CARIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_CARIAN\0" as *const u8 as *const gchar,
                value_nick: b"carian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_LYCIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_LYCIAN\0" as *const u8 as *const gchar,
                value_nick: b"lycian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_LYDIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_LYDIAN\0" as *const u8 as *const gchar,
                value_nick: b"lydian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_AVESTAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_AVESTAN\0" as *const u8 as *const gchar,
                value_nick: b"avestan\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_BAMUM as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_BAMUM\0" as *const u8 as *const gchar,
                value_nick: b"bamum\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_EGYPTIAN_HIEROGLYPHS as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_EGYPTIAN_HIEROGLYPHS\0" as *const u8 as *const gchar,
                value_nick: b"egyptian-hieroglyphs\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_IMPERIAL_ARAMAIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_IMPERIAL_ARAMAIC\0" as *const u8 as *const gchar,
                value_nick: b"imperial-aramaic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_INSCRIPTIONAL_PAHLAVI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_INSCRIPTIONAL_PAHLAVI\0" as *const u8
                    as *const gchar,
                value_nick: b"inscriptional-pahlavi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_INSCRIPTIONAL_PARTHIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_INSCRIPTIONAL_PARTHIAN\0" as *const u8
                    as *const gchar,
                value_nick: b"inscriptional-parthian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_JAVANESE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_JAVANESE\0" as *const u8 as *const gchar,
                value_nick: b"javanese\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_KAITHI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_KAITHI\0" as *const u8 as *const gchar,
                value_nick: b"kaithi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_LISU as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_LISU\0" as *const u8 as *const gchar,
                value_nick: b"lisu\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MEETEI_MAYEK as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MEETEI_MAYEK\0" as *const u8 as *const gchar,
                value_nick: b"meetei-mayek\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OLD_SOUTH_ARABIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OLD_SOUTH_ARABIAN\0" as *const u8 as *const gchar,
                value_nick: b"old-south-arabian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OLD_TURKIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OLD_TURKIC\0" as *const u8 as *const gchar,
                value_nick: b"old-turkic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SAMARITAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SAMARITAN\0" as *const u8 as *const gchar,
                value_nick: b"samaritan\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TAI_THAM as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TAI_THAM\0" as *const u8 as *const gchar,
                value_nick: b"tai-tham\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TAI_VIET as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TAI_VIET\0" as *const u8 as *const gchar,
                value_nick: b"tai-viet\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_BATAK as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_BATAK\0" as *const u8 as *const gchar,
                value_nick: b"batak\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_BRAHMI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_BRAHMI\0" as *const u8 as *const gchar,
                value_nick: b"brahmi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MANDAIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MANDAIC\0" as *const u8 as *const gchar,
                value_nick: b"mandaic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_CHAKMA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_CHAKMA\0" as *const u8 as *const gchar,
                value_nick: b"chakma\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MEROITIC_CURSIVE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MEROITIC_CURSIVE\0" as *const u8 as *const gchar,
                value_nick: b"meroitic-cursive\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MEROITIC_HIEROGLYPHS as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MEROITIC_HIEROGLYPHS\0" as *const u8 as *const gchar,
                value_nick: b"meroitic-hieroglyphs\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MIAO as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MIAO\0" as *const u8 as *const gchar,
                value_nick: b"miao\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SHARADA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SHARADA\0" as *const u8 as *const gchar,
                value_nick: b"sharada\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SORA_SOMPENG as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SORA_SOMPENG\0" as *const u8 as *const gchar,
                value_nick: b"sora-sompeng\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TAKRI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TAKRI\0" as *const u8 as *const gchar,
                value_nick: b"takri\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_BASSA_VAH as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_BASSA_VAH\0" as *const u8 as *const gchar,
                value_nick: b"bassa-vah\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_CAUCASIAN_ALBANIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_CAUCASIAN_ALBANIAN\0" as *const u8 as *const gchar,
                value_nick: b"caucasian-albanian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_DUPLOYAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_DUPLOYAN\0" as *const u8 as *const gchar,
                value_nick: b"duployan\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_ELBASAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_ELBASAN\0" as *const u8 as *const gchar,
                value_nick: b"elbasan\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_GRANTHA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_GRANTHA\0" as *const u8 as *const gchar,
                value_nick: b"grantha\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_KHOJKI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_KHOJKI\0" as *const u8 as *const gchar,
                value_nick: b"khojki\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_KHUDAWADI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_KHUDAWADI\0" as *const u8 as *const gchar,
                value_nick: b"khudawadi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_LINEAR_A as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_LINEAR_A\0" as *const u8 as *const gchar,
                value_nick: b"linear-a\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MAHAJANI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MAHAJANI\0" as *const u8 as *const gchar,
                value_nick: b"mahajani\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MANICHAEAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MANICHAEAN\0" as *const u8 as *const gchar,
                value_nick: b"manichaean\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MENDE_KIKAKUI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MENDE_KIKAKUI\0" as *const u8 as *const gchar,
                value_nick: b"mende-kikakui\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MODI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MODI\0" as *const u8 as *const gchar,
                value_nick: b"modi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MRO as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MRO\0" as *const u8 as *const gchar,
                value_nick: b"mro\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_NABATAEAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_NABATAEAN\0" as *const u8 as *const gchar,
                value_nick: b"nabataean\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OLD_NORTH_ARABIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OLD_NORTH_ARABIAN\0" as *const u8 as *const gchar,
                value_nick: b"old-north-arabian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OLD_PERMIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OLD_PERMIC\0" as *const u8 as *const gchar,
                value_nick: b"old-permic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_PAHAWH_HMONG as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_PAHAWH_HMONG\0" as *const u8 as *const gchar,
                value_nick: b"pahawh-hmong\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_PALMYRENE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_PALMYRENE\0" as *const u8 as *const gchar,
                value_nick: b"palmyrene\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_PAU_CIN_HAU as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_PAU_CIN_HAU\0" as *const u8 as *const gchar,
                value_nick: b"pau-cin-hau\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_PSALTER_PAHLAVI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_PSALTER_PAHLAVI\0" as *const u8 as *const gchar,
                value_nick: b"psalter-pahlavi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SIDDHAM as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SIDDHAM\0" as *const u8 as *const gchar,
                value_nick: b"siddham\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TIRHUTA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TIRHUTA\0" as *const u8 as *const gchar,
                value_nick: b"tirhuta\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_WARANG_CITI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_WARANG_CITI\0" as *const u8 as *const gchar,
                value_nick: b"warang-citi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_AHOM as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_AHOM\0" as *const u8 as *const gchar,
                value_nick: b"ahom\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_ANATOLIAN_HIEROGLYPHS as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_ANATOLIAN_HIEROGLYPHS\0" as *const u8
                    as *const gchar,
                value_nick: b"anatolian-hieroglyphs\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_HATRAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_HATRAN\0" as *const u8 as *const gchar,
                value_nick: b"hatran\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MULTANI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MULTANI\0" as *const u8 as *const gchar,
                value_nick: b"multani\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OLD_HUNGARIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OLD_HUNGARIAN\0" as *const u8 as *const gchar,
                value_nick: b"old-hungarian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SIGNWRITING as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SIGNWRITING\0" as *const u8 as *const gchar,
                value_nick: b"signwriting\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_ADLAM as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_ADLAM\0" as *const u8 as *const gchar,
                value_nick: b"adlam\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_BHAIKSUKI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_BHAIKSUKI\0" as *const u8 as *const gchar,
                value_nick: b"bhaiksuki\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MARCHEN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MARCHEN\0" as *const u8 as *const gchar,
                value_nick: b"marchen\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_NEWA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_NEWA\0" as *const u8 as *const gchar,
                value_nick: b"newa\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OSAGE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OSAGE\0" as *const u8 as *const gchar,
                value_nick: b"osage\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TANGUT as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TANGUT\0" as *const u8 as *const gchar,
                value_nick: b"tangut\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MASARAM_GONDI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MASARAM_GONDI\0" as *const u8 as *const gchar,
                value_nick: b"masaram-gondi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_NUSHU as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_NUSHU\0" as *const u8 as *const gchar,
                value_nick: b"nushu\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SOYOMBO as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SOYOMBO\0" as *const u8 as *const gchar,
                value_nick: b"soyombo\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_ZANABAZAR_SQUARE as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_ZANABAZAR_SQUARE\0" as *const u8 as *const gchar,
                value_nick: b"zanabazar-square\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_DOGRA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_DOGRA\0" as *const u8 as *const gchar,
                value_nick: b"dogra\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_GUNJALA_GONDI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_GUNJALA_GONDI\0" as *const u8 as *const gchar,
                value_nick: b"gunjala-gondi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_HANIFI_ROHINGYA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_HANIFI_ROHINGYA\0" as *const u8 as *const gchar,
                value_nick: b"hanifi-rohingya\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MAKASAR as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MAKASAR\0" as *const u8 as *const gchar,
                value_nick: b"makasar\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MEDEFAIDRIN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MEDEFAIDRIN\0" as *const u8 as *const gchar,
                value_nick: b"medefaidrin\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OLD_SOGDIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OLD_SOGDIAN\0" as *const u8 as *const gchar,
                value_nick: b"old-sogdian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_SOGDIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_SOGDIAN\0" as *const u8 as *const gchar,
                value_nick: b"sogdian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_ELYMAIC as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_ELYMAIC\0" as *const u8 as *const gchar,
                value_nick: b"elymaic\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_NANDINAGARI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_NANDINAGARI\0" as *const u8 as *const gchar,
                value_nick: b"nandinagari\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_NYIAKENG_PUACHUE_HMONG as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_NYIAKENG_PUACHUE_HMONG\0" as *const u8
                    as *const gchar,
                value_nick: b"nyiakeng-puachue-hmong\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_WANCHO as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_WANCHO\0" as *const u8 as *const gchar,
                value_nick: b"wancho\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_CHORASMIAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_CHORASMIAN\0" as *const u8 as *const gchar,
                value_nick: b"chorasmian\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_DIVES_AKURU as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_DIVES_AKURU\0" as *const u8 as *const gchar,
                value_nick: b"dives-akuru\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_KHITAN_SMALL_SCRIPT as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_KHITAN_SMALL_SCRIPT\0" as *const u8 as *const gchar,
                value_nick: b"khitan-small-script\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_YEZIDI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_YEZIDI\0" as *const u8 as *const gchar,
                value_nick: b"yezidi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_CYPRO_MINOAN as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_CYPRO_MINOAN\0" as *const u8 as *const gchar,
                value_nick: b"cypro-minoan\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_OLD_UYGHUR as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_OLD_UYGHUR\0" as *const u8 as *const gchar,
                value_nick: b"old-uyghur\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TANGSA as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TANGSA\0" as *const u8 as *const gchar,
                value_nick: b"tangsa\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_TOTO as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_TOTO\0" as *const u8 as *const gchar,
                value_nick: b"toto\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_VITHKUQI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_VITHKUQI\0" as *const u8 as *const gchar,
                value_nick: b"vithkuqi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_MATH as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_MATH\0" as *const u8 as *const gchar,
                value_nick: b"math\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_KAWI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_KAWI\0" as *const u8 as *const gchar,
                value_nick: b"kawi\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_UNICODE_SCRIPT_NAG_MUNDARI as ::core::ffi::c_int as gint,
                value_name: b"G_UNICODE_SCRIPT_NAG_MUNDARI\0" as *const u8 as *const gchar,
                value_nick: b"nag-mundari\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GUnicodeScript\0" as *const u8 as *const gchar),
            &raw const values as *const GEnumValue,
        );
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_normalize_mode_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        static mut values: [GEnumValue; 9] = [
            _GEnumValue {
                value: G_NORMALIZE_DEFAULT as ::core::ffi::c_int as gint,
                value_name: b"G_NORMALIZE_DEFAULT\0" as *const u8 as *const gchar,
                value_nick: b"default\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NORMALIZE_NFD as ::core::ffi::c_int as gint,
                value_name: b"G_NORMALIZE_NFD\0" as *const u8 as *const gchar,
                value_nick: b"nfd\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NORMALIZE_DEFAULT_COMPOSE as ::core::ffi::c_int as gint,
                value_name: b"G_NORMALIZE_DEFAULT_COMPOSE\0" as *const u8 as *const gchar,
                value_nick: b"default-compose\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NORMALIZE_NFC as ::core::ffi::c_int as gint,
                value_name: b"G_NORMALIZE_NFC\0" as *const u8 as *const gchar,
                value_nick: b"nfc\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NORMALIZE_ALL as ::core::ffi::c_int as gint,
                value_name: b"G_NORMALIZE_ALL\0" as *const u8 as *const gchar,
                value_nick: b"all\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NORMALIZE_NFKD as ::core::ffi::c_int as gint,
                value_name: b"G_NORMALIZE_NFKD\0" as *const u8 as *const gchar,
                value_nick: b"nfkd\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NORMALIZE_ALL_COMPOSE as ::core::ffi::c_int as gint,
                value_name: b"G_NORMALIZE_ALL_COMPOSE\0" as *const u8 as *const gchar,
                value_nick: b"all-compose\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: G_NORMALIZE_NFKC as ::core::ffi::c_int as gint,
                value_name: b"G_NORMALIZE_NFKC\0" as *const u8 as *const gchar,
                value_nick: b"nfkc\0" as *const u8 as *const gchar,
            },
            _GEnumValue {
                value: 0 as gint,
                value_name: ::core::ptr::null::<gchar>(),
                value_nick: ::core::ptr::null::<gchar>(),
            },
        ];
        let mut g_define_type_id: GType = g_enum_register_static(
            g_intern_static_string(b"GNormalizeMode\0" as *const u8 as *const gchar),
            &raw const values as *const GEnumValue,
        );
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
