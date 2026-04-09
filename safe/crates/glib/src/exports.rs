#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[export_name = "__glib_assert_msg"]
pub static PLACEHOLDER_00003: u8 = 0;

#[export_name = "g_access"]
pub static PLACEHOLDER_00006: u8 = 0;

#[export_name = "g_aligned_alloc0"]
pub static PLACEHOLDER_00009: u8 = 0;

#[export_name = "g_aligned_alloc"]
pub static PLACEHOLDER_00012: u8 = 0;

#[export_name = "g_aligned_free"]
pub static PLACEHOLDER_00015: u8 = 0;

#[export_name = "g_aligned_free_sized"]
pub static PLACEHOLDER_00018: u8 = 0;

#[export_name = "g_allocator_free"]
pub static PLACEHOLDER_00021: u8 = 0;

#[export_name = "g_allocator_new"]
pub static PLACEHOLDER_00024: u8 = 0;

#[export_name = "g_array_append_vals"]
pub static PLACEHOLDER_00027: u8 = 0;

#[export_name = "g_array_binary_search"]
pub static PLACEHOLDER_00030: u8 = 0;

#[export_name = "g_array_copy"]
pub static PLACEHOLDER_00033: u8 = 0;

#[export_name = "g_array_free"]
pub static PLACEHOLDER_00036: u8 = 0;

#[export_name = "g_array_get_element_size"]
pub static PLACEHOLDER_00039: u8 = 0;

#[export_name = "g_array_insert_vals"]
pub static PLACEHOLDER_00042: u8 = 0;

#[export_name = "g_array_new"]
pub static PLACEHOLDER_00045: u8 = 0;

#[export_name = "g_array_new_take"]
pub static PLACEHOLDER_00048: u8 = 0;

#[export_name = "g_array_new_take_zero_terminated"]
pub static PLACEHOLDER_00051: u8 = 0;

#[export_name = "g_array_prepend_vals"]
pub static PLACEHOLDER_00054: u8 = 0;

#[export_name = "g_array_ref"]
pub static PLACEHOLDER_00057: u8 = 0;

#[export_name = "g_array_remove_index"]
pub static PLACEHOLDER_00060: u8 = 0;

#[export_name = "g_array_remove_index_fast"]
pub static PLACEHOLDER_00063: u8 = 0;

#[export_name = "g_array_remove_range"]
pub static PLACEHOLDER_00066: u8 = 0;

#[export_name = "g_array_set_clear_func"]
pub static PLACEHOLDER_00069: u8 = 0;

#[export_name = "g_array_set_size"]
pub static PLACEHOLDER_00072: u8 = 0;

#[export_name = "g_array_sized_new"]
pub static PLACEHOLDER_00075: u8 = 0;

#[export_name = "g_array_sort"]
pub static PLACEHOLDER_00078: u8 = 0;

#[export_name = "g_array_sort_with_data"]
pub static PLACEHOLDER_00081: u8 = 0;

#[export_name = "g_array_steal"]
pub static PLACEHOLDER_00084: u8 = 0;

#[export_name = "g_array_unref"]
pub static PLACEHOLDER_00087: u8 = 0;

#[export_name = "g_ascii_digit_value"]
pub static PLACEHOLDER_00090: u8 = 0;

#[export_name = "g_ascii_dtostr"]
pub static PLACEHOLDER_00093: u8 = 0;

#[export_name = "g_ascii_formatd"]
pub static PLACEHOLDER_00096: u8 = 0;

#[export_name = "g_ascii_strcasecmp"]
pub static PLACEHOLDER_00099: u8 = 0;

#[export_name = "g_ascii_strdown"]
pub static PLACEHOLDER_00102: u8 = 0;

#[export_name = "g_ascii_string_to_signed"]
pub static PLACEHOLDER_00105: u8 = 0;

#[export_name = "g_ascii_string_to_unsigned"]
pub static PLACEHOLDER_00108: u8 = 0;

#[export_name = "g_ascii_strncasecmp"]
pub static PLACEHOLDER_00111: u8 = 0;

#[export_name = "g_ascii_strtod"]
pub static PLACEHOLDER_00114: u8 = 0;

#[export_name = "g_ascii_strtoll"]
pub static PLACEHOLDER_00117: u8 = 0;

#[export_name = "g_ascii_strtoull"]
pub static PLACEHOLDER_00120: u8 = 0;

#[export_name = "g_ascii_strup"]
pub static PLACEHOLDER_00123: u8 = 0;

#[export_name = "g_ascii_table"]
pub static PLACEHOLDER_00126: u8 = 0;

#[export_name = "g_ascii_tolower"]
pub static PLACEHOLDER_00129: u8 = 0;

#[export_name = "g_ascii_toupper"]
pub static PLACEHOLDER_00132: u8 = 0;

#[export_name = "g_ascii_xdigit_value"]
pub static PLACEHOLDER_00135: u8 = 0;

#[export_name = "g_assert_warning"]
pub static PLACEHOLDER_00138: u8 = 0;

#[export_name = "g_assertion_message"]
pub unsafe extern "C" fn export_g_assertion_message(
    domain: *const crate::ffi::gchar,
    file: *const crate::ffi::gchar,
    line: crate::ffi::gint,
    func: *const crate::ffi::gchar,
    message: *const crate::ffi::gchar,
) -> ! {
    crate::runtime::g_assertion_message_impl(domain, file, line, func, message)
}

#[export_name = "g_assertion_message_cmpint"]
pub unsafe extern "C" fn export_g_assertion_message_cmpint(
    domain: *const crate::ffi::gchar,
    file: *const crate::ffi::gchar,
    line: crate::ffi::gint,
    func: *const crate::ffi::gchar,
    expr: *const crate::ffi::gchar,
    arg1: crate::ffi::guint64,
    cmp: *const crate::ffi::gchar,
    arg2: crate::ffi::guint64,
    numtype: crate::ffi::gchar,
) -> ! {
    crate::runtime::g_assertion_message_cmpint_impl(domain, file, line, func, expr, arg1, cmp, arg2, numtype)
}

#[export_name = "g_assertion_message_cmpnum"]
pub unsafe extern "C" fn export_g_assertion_message_cmpnum(
    domain: *const crate::ffi::gchar,
    file: *const crate::ffi::gchar,
    line: crate::ffi::gint,
    func: *const crate::ffi::gchar,
    expr: *const crate::ffi::gchar,
    arg1: crate::ffi::glongdouble,
    cmp: *const crate::ffi::gchar,
    arg2: crate::ffi::glongdouble,
    numtype: crate::ffi::gchar,
) -> ! {
    crate::runtime::g_assertion_message_cmpnum_impl(domain, file, line, func, expr, arg1, cmp, arg2, numtype)
}

#[export_name = "g_assertion_message_cmpstr"]
pub unsafe extern "C" fn export_g_assertion_message_cmpstr(
    domain: *const crate::ffi::gchar,
    file: *const crate::ffi::gchar,
    line: crate::ffi::gint,
    func: *const crate::ffi::gchar,
    expr: *const crate::ffi::gchar,
    arg1: *const crate::ffi::gchar,
    cmp: *const crate::ffi::gchar,
    arg2: *const crate::ffi::gchar,
) -> ! {
    crate::runtime::g_assertion_message_cmpstr_impl(domain, file, line, func, expr, arg1, cmp, arg2)
}

#[export_name = "g_assertion_message_cmpstrv"]
pub static PLACEHOLDER_00149: u8 = 0;

#[export_name = "g_assertion_message_error"]
pub unsafe extern "C" fn export_g_assertion_message_error(
    domain: *const crate::ffi::gchar,
    file: *const crate::ffi::gchar,
    line: crate::ffi::gint,
    func: *const crate::ffi::gchar,
    expr: *const crate::ffi::gchar,
    error: *const crate::abi::GError,
    error_domain: crate::ffi::GQuark,
    error_code: crate::ffi::gint,
) -> ! {
    crate::runtime::g_assertion_message_error_impl(domain, file, line, func, expr, error, error_domain, error_code)
}

#[export_name = "g_assertion_message_expr"]
pub unsafe extern "C" fn export_g_assertion_message_expr(
    domain: *const crate::ffi::gchar,
    file: *const crate::ffi::gchar,
    line: crate::ffi::gint,
    func: *const crate::ffi::gchar,
    expr: *const crate::ffi::gchar,
) -> ! {
    crate::runtime::g_assertion_message_expr_impl(domain, file, line, func, expr)
}

#[export_name = "g_async_queue_length"]
pub static PLACEHOLDER_00156: u8 = 0;

#[export_name = "g_async_queue_length_unlocked"]
pub static PLACEHOLDER_00159: u8 = 0;

#[export_name = "g_async_queue_lock"]
pub static PLACEHOLDER_00162: u8 = 0;

#[export_name = "g_async_queue_new"]
pub static PLACEHOLDER_00165: u8 = 0;

#[export_name = "g_async_queue_new_full"]
pub static PLACEHOLDER_00168: u8 = 0;

#[export_name = "g_async_queue_pop"]
pub static PLACEHOLDER_00171: u8 = 0;

#[export_name = "g_async_queue_pop_unlocked"]
pub static PLACEHOLDER_00174: u8 = 0;

#[export_name = "g_async_queue_push"]
pub static PLACEHOLDER_00177: u8 = 0;

#[export_name = "g_async_queue_push_front"]
pub static PLACEHOLDER_00180: u8 = 0;

#[export_name = "g_async_queue_push_front_unlocked"]
pub static PLACEHOLDER_00183: u8 = 0;

#[export_name = "g_async_queue_push_sorted"]
pub static PLACEHOLDER_00186: u8 = 0;

#[export_name = "g_async_queue_push_sorted_unlocked"]
pub static PLACEHOLDER_00189: u8 = 0;

#[export_name = "g_async_queue_push_unlocked"]
pub static PLACEHOLDER_00192: u8 = 0;

#[export_name = "g_async_queue_ref"]
pub static PLACEHOLDER_00195: u8 = 0;

#[export_name = "g_async_queue_ref_unlocked"]
pub static PLACEHOLDER_00198: u8 = 0;

#[export_name = "g_async_queue_remove"]
pub static PLACEHOLDER_00201: u8 = 0;

#[export_name = "g_async_queue_remove_unlocked"]
pub static PLACEHOLDER_00204: u8 = 0;

#[export_name = "g_async_queue_sort"]
pub static PLACEHOLDER_00207: u8 = 0;

#[export_name = "g_async_queue_sort_unlocked"]
pub static PLACEHOLDER_00210: u8 = 0;

#[export_name = "g_async_queue_timed_pop"]
pub static PLACEHOLDER_00213: u8 = 0;

#[export_name = "g_async_queue_timed_pop_unlocked"]
pub static PLACEHOLDER_00216: u8 = 0;

#[export_name = "g_async_queue_timeout_pop"]
pub static PLACEHOLDER_00219: u8 = 0;

#[export_name = "g_async_queue_timeout_pop_unlocked"]
pub static PLACEHOLDER_00222: u8 = 0;

#[export_name = "g_async_queue_try_pop"]
pub static PLACEHOLDER_00225: u8 = 0;

#[export_name = "g_async_queue_try_pop_unlocked"]
pub static PLACEHOLDER_00228: u8 = 0;

#[export_name = "g_async_queue_unlock"]
pub static PLACEHOLDER_00231: u8 = 0;

#[export_name = "g_async_queue_unref"]
pub static PLACEHOLDER_00234: u8 = 0;

#[export_name = "g_async_queue_unref_and_unlock"]
pub static PLACEHOLDER_00237: u8 = 0;

#[export_name = "g_atexit"]
pub static PLACEHOLDER_00240: u8 = 0;

#[export_name = "g_atomic_int_add"]
pub static PLACEHOLDER_00243: u8 = 0;

#[export_name = "g_atomic_int_and"]
pub static PLACEHOLDER_00246: u8 = 0;

#[export_name = "g_atomic_int_compare_and_exchange"]
pub static PLACEHOLDER_00249: u8 = 0;

#[export_name = "g_atomic_int_compare_and_exchange_full"]
pub static PLACEHOLDER_00252: u8 = 0;

#[export_name = "g_atomic_int_dec_and_test"]
pub static PLACEHOLDER_00255: u8 = 0;

#[export_name = "g_atomic_int_exchange"]
pub static PLACEHOLDER_00258: u8 = 0;

#[export_name = "g_atomic_int_exchange_and_add"]
pub static PLACEHOLDER_00261: u8 = 0;

#[export_name = "g_atomic_int_get"]
pub static PLACEHOLDER_00264: u8 = 0;

#[export_name = "g_atomic_int_inc"]
pub static PLACEHOLDER_00267: u8 = 0;

#[export_name = "g_atomic_int_or"]
pub static PLACEHOLDER_00270: u8 = 0;

#[export_name = "g_atomic_int_set"]
pub static PLACEHOLDER_00273: u8 = 0;

#[export_name = "g_atomic_int_xor"]
pub static PLACEHOLDER_00276: u8 = 0;

#[export_name = "g_atomic_pointer_add"]
pub static PLACEHOLDER_00279: u8 = 0;

#[export_name = "g_atomic_pointer_and"]
pub static PLACEHOLDER_00282: u8 = 0;

#[export_name = "g_atomic_pointer_compare_and_exchange"]
pub static PLACEHOLDER_00285: u8 = 0;

#[export_name = "g_atomic_pointer_compare_and_exchange_full"]
pub static PLACEHOLDER_00288: u8 = 0;

#[export_name = "g_atomic_pointer_exchange"]
pub static PLACEHOLDER_00291: u8 = 0;

#[export_name = "g_atomic_pointer_get"]
pub static PLACEHOLDER_00294: u8 = 0;

#[export_name = "g_atomic_pointer_or"]
pub static PLACEHOLDER_00297: u8 = 0;

#[export_name = "g_atomic_pointer_set"]
pub static PLACEHOLDER_00300: u8 = 0;

#[export_name = "g_atomic_pointer_xor"]
pub static PLACEHOLDER_00303: u8 = 0;

#[export_name = "g_atomic_rc_box_acquire"]
pub static PLACEHOLDER_00306: u8 = 0;

#[export_name = "g_atomic_rc_box_alloc0"]
pub static PLACEHOLDER_00309: u8 = 0;

#[export_name = "g_atomic_rc_box_alloc"]
pub static PLACEHOLDER_00312: u8 = 0;

#[export_name = "g_atomic_rc_box_dup"]
pub static PLACEHOLDER_00315: u8 = 0;

#[export_name = "g_atomic_rc_box_get_size"]
pub static PLACEHOLDER_00318: u8 = 0;

#[export_name = "g_atomic_rc_box_release"]
pub static PLACEHOLDER_00321: u8 = 0;

#[export_name = "g_atomic_rc_box_release_full"]
pub static PLACEHOLDER_00324: u8 = 0;

#[export_name = "g_atomic_ref_count_compare"]
pub static PLACEHOLDER_00327: u8 = 0;

#[export_name = "g_atomic_ref_count_dec"]
pub static PLACEHOLDER_00330: u8 = 0;

#[export_name = "g_atomic_ref_count_inc"]
pub static PLACEHOLDER_00333: u8 = 0;

#[export_name = "g_atomic_ref_count_init"]
pub static PLACEHOLDER_00336: u8 = 0;

#[export_name = "g_base64_decode"]
pub static PLACEHOLDER_00339: u8 = 0;

#[export_name = "g_base64_decode_inplace"]
pub static PLACEHOLDER_00342: u8 = 0;

#[export_name = "g_base64_decode_step"]
pub static PLACEHOLDER_00345: u8 = 0;

#[export_name = "g_base64_encode"]
pub static PLACEHOLDER_00348: u8 = 0;

#[export_name = "g_base64_encode_close"]
pub static PLACEHOLDER_00351: u8 = 0;

#[export_name = "g_base64_encode_step"]
pub static PLACEHOLDER_00354: u8 = 0;

#[export_name = "g_basename"]
pub static PLACEHOLDER_00357: u8 = 0;

#[export_name = "g_bit_lock"]
pub static PLACEHOLDER_00360: u8 = 0;

#[export_name = "g_bit_nth_lsf"]
pub static PLACEHOLDER_00363: u8 = 0;

#[export_name = "g_bit_nth_msf"]
pub static PLACEHOLDER_00366: u8 = 0;

#[export_name = "g_bit_storage"]
pub static PLACEHOLDER_00369: u8 = 0;

#[export_name = "g_bit_trylock"]
pub static PLACEHOLDER_00372: u8 = 0;

#[export_name = "g_bit_unlock"]
pub static PLACEHOLDER_00375: u8 = 0;

#[export_name = "g_blow_chunks"]
pub static PLACEHOLDER_00378: u8 = 0;

#[export_name = "g_bookmark_file_add_application"]
pub static PLACEHOLDER_00381: u8 = 0;

#[export_name = "g_bookmark_file_add_group"]
pub static PLACEHOLDER_00384: u8 = 0;

#[export_name = "g_bookmark_file_copy"]
pub static PLACEHOLDER_00387: u8 = 0;

#[export_name = "g_bookmark_file_error_quark"]
pub static PLACEHOLDER_00390: u8 = 0;

#[export_name = "g_bookmark_file_free"]
pub static PLACEHOLDER_00393: u8 = 0;

#[export_name = "g_bookmark_file_get_added"]
pub static PLACEHOLDER_00396: u8 = 0;

#[export_name = "g_bookmark_file_get_added_date_time"]
pub static PLACEHOLDER_00399: u8 = 0;

#[export_name = "g_bookmark_file_get_app_info"]
pub static PLACEHOLDER_00402: u8 = 0;

#[export_name = "g_bookmark_file_get_application_info"]
pub static PLACEHOLDER_00405: u8 = 0;

#[export_name = "g_bookmark_file_get_applications"]
pub static PLACEHOLDER_00408: u8 = 0;

#[export_name = "g_bookmark_file_get_description"]
pub static PLACEHOLDER_00411: u8 = 0;

#[export_name = "g_bookmark_file_get_groups"]
pub static PLACEHOLDER_00414: u8 = 0;

#[export_name = "g_bookmark_file_get_icon"]
pub static PLACEHOLDER_00417: u8 = 0;

#[export_name = "g_bookmark_file_get_is_private"]
pub static PLACEHOLDER_00420: u8 = 0;

#[export_name = "g_bookmark_file_get_mime_type"]
pub static PLACEHOLDER_00423: u8 = 0;

#[export_name = "g_bookmark_file_get_modified"]
pub static PLACEHOLDER_00426: u8 = 0;

#[export_name = "g_bookmark_file_get_modified_date_time"]
pub static PLACEHOLDER_00429: u8 = 0;

#[export_name = "g_bookmark_file_get_size"]
pub static PLACEHOLDER_00432: u8 = 0;

#[export_name = "g_bookmark_file_get_title"]
pub static PLACEHOLDER_00435: u8 = 0;

#[export_name = "g_bookmark_file_get_uris"]
pub static PLACEHOLDER_00438: u8 = 0;

#[export_name = "g_bookmark_file_get_visited"]
pub static PLACEHOLDER_00441: u8 = 0;

#[export_name = "g_bookmark_file_get_visited_date_time"]
pub static PLACEHOLDER_00444: u8 = 0;

#[export_name = "g_bookmark_file_has_application"]
pub static PLACEHOLDER_00447: u8 = 0;

#[export_name = "g_bookmark_file_has_group"]
pub static PLACEHOLDER_00450: u8 = 0;

#[export_name = "g_bookmark_file_has_item"]
pub static PLACEHOLDER_00453: u8 = 0;

#[export_name = "g_bookmark_file_load_from_data"]
pub static PLACEHOLDER_00456: u8 = 0;

#[export_name = "g_bookmark_file_load_from_data_dirs"]
pub static PLACEHOLDER_00459: u8 = 0;

#[export_name = "g_bookmark_file_load_from_file"]
pub static PLACEHOLDER_00462: u8 = 0;

#[export_name = "g_bookmark_file_move_item"]
pub static PLACEHOLDER_00465: u8 = 0;

#[export_name = "g_bookmark_file_new"]
pub static PLACEHOLDER_00468: u8 = 0;

#[export_name = "g_bookmark_file_remove_application"]
pub static PLACEHOLDER_00471: u8 = 0;

#[export_name = "g_bookmark_file_remove_group"]
pub static PLACEHOLDER_00474: u8 = 0;

#[export_name = "g_bookmark_file_remove_item"]
pub static PLACEHOLDER_00477: u8 = 0;

#[export_name = "g_bookmark_file_set_added"]
pub static PLACEHOLDER_00480: u8 = 0;

#[export_name = "g_bookmark_file_set_added_date_time"]
pub static PLACEHOLDER_00483: u8 = 0;

#[export_name = "g_bookmark_file_set_app_info"]
pub static PLACEHOLDER_00486: u8 = 0;

#[export_name = "g_bookmark_file_set_application_info"]
pub static PLACEHOLDER_00489: u8 = 0;

#[export_name = "g_bookmark_file_set_description"]
pub static PLACEHOLDER_00492: u8 = 0;

#[export_name = "g_bookmark_file_set_groups"]
pub static PLACEHOLDER_00495: u8 = 0;

#[export_name = "g_bookmark_file_set_icon"]
pub static PLACEHOLDER_00498: u8 = 0;

#[export_name = "g_bookmark_file_set_is_private"]
pub static PLACEHOLDER_00501: u8 = 0;

#[export_name = "g_bookmark_file_set_mime_type"]
pub static PLACEHOLDER_00504: u8 = 0;

#[export_name = "g_bookmark_file_set_modified"]
pub static PLACEHOLDER_00507: u8 = 0;

#[export_name = "g_bookmark_file_set_modified_date_time"]
pub static PLACEHOLDER_00510: u8 = 0;

#[export_name = "g_bookmark_file_set_title"]
pub static PLACEHOLDER_00513: u8 = 0;

#[export_name = "g_bookmark_file_set_visited"]
pub static PLACEHOLDER_00516: u8 = 0;

#[export_name = "g_bookmark_file_set_visited_date_time"]
pub static PLACEHOLDER_00519: u8 = 0;

#[export_name = "g_bookmark_file_to_data"]
pub static PLACEHOLDER_00522: u8 = 0;

#[export_name = "g_bookmark_file_to_file"]
pub static PLACEHOLDER_00525: u8 = 0;

#[export_name = "g_build_filename"]
pub static PLACEHOLDER_00528: u8 = 0;

#[export_name = "g_build_filename_valist"]
pub static PLACEHOLDER_00531: u8 = 0;

#[export_name = "g_build_filenamev"]
pub static PLACEHOLDER_00534: u8 = 0;

#[export_name = "g_build_path"]
pub static PLACEHOLDER_00537: u8 = 0;

#[export_name = "g_build_pathv"]
pub static PLACEHOLDER_00540: u8 = 0;

#[export_name = "g_byte_array_append"]
pub static PLACEHOLDER_00543: u8 = 0;

#[export_name = "g_byte_array_free"]
pub static PLACEHOLDER_00546: u8 = 0;

#[export_name = "g_byte_array_free_to_bytes"]
pub static PLACEHOLDER_00549: u8 = 0;

#[export_name = "g_byte_array_new"]
pub static PLACEHOLDER_00552: u8 = 0;

#[export_name = "g_byte_array_new_take"]
pub static PLACEHOLDER_00555: u8 = 0;

#[export_name = "g_byte_array_prepend"]
pub static PLACEHOLDER_00558: u8 = 0;

#[export_name = "g_byte_array_ref"]
pub static PLACEHOLDER_00561: u8 = 0;

#[export_name = "g_byte_array_remove_index"]
pub static PLACEHOLDER_00564: u8 = 0;

#[export_name = "g_byte_array_remove_index_fast"]
pub static PLACEHOLDER_00567: u8 = 0;

#[export_name = "g_byte_array_remove_range"]
pub static PLACEHOLDER_00570: u8 = 0;

#[export_name = "g_byte_array_set_size"]
pub static PLACEHOLDER_00573: u8 = 0;

#[export_name = "g_byte_array_sized_new"]
pub static PLACEHOLDER_00576: u8 = 0;

#[export_name = "g_byte_array_sort"]
pub static PLACEHOLDER_00579: u8 = 0;

#[export_name = "g_byte_array_sort_with_data"]
pub static PLACEHOLDER_00582: u8 = 0;

#[export_name = "g_byte_array_steal"]
pub static PLACEHOLDER_00585: u8 = 0;

#[export_name = "g_byte_array_unref"]
pub static PLACEHOLDER_00588: u8 = 0;

#[export_name = "g_bytes_compare"]
pub static PLACEHOLDER_00591: u8 = 0;

#[export_name = "g_bytes_equal"]
pub static PLACEHOLDER_00594: u8 = 0;

#[export_name = "g_bytes_get_data"]
pub static PLACEHOLDER_00597: u8 = 0;

#[export_name = "g_bytes_get_region"]
pub static PLACEHOLDER_00600: u8 = 0;

#[export_name = "g_bytes_get_size"]
pub static PLACEHOLDER_00603: u8 = 0;

#[export_name = "g_bytes_hash"]
pub static PLACEHOLDER_00606: u8 = 0;

#[export_name = "g_bytes_new"]
pub static PLACEHOLDER_00609: u8 = 0;

#[export_name = "g_bytes_new_from_bytes"]
pub static PLACEHOLDER_00612: u8 = 0;

#[export_name = "g_bytes_new_static"]
pub static PLACEHOLDER_00615: u8 = 0;

#[export_name = "g_bytes_new_take"]
pub static PLACEHOLDER_00618: u8 = 0;

#[export_name = "g_bytes_new_with_free_func"]
pub static PLACEHOLDER_00621: u8 = 0;

#[export_name = "g_bytes_ref"]
pub static PLACEHOLDER_00624: u8 = 0;

#[export_name = "g_bytes_unref"]
pub static PLACEHOLDER_00627: u8 = 0;

#[export_name = "g_bytes_unref_to_array"]
pub static PLACEHOLDER_00630: u8 = 0;

#[export_name = "g_bytes_unref_to_data"]
pub static PLACEHOLDER_00633: u8 = 0;

#[export_name = "g_cache_destroy"]
pub static PLACEHOLDER_00636: u8 = 0;

#[export_name = "g_cache_insert"]
pub static PLACEHOLDER_00639: u8 = 0;

#[export_name = "g_cache_key_foreach"]
pub static PLACEHOLDER_00642: u8 = 0;

#[export_name = "g_cache_new"]
pub static PLACEHOLDER_00645: u8 = 0;

#[export_name = "g_cache_remove"]
pub static PLACEHOLDER_00648: u8 = 0;

#[export_name = "g_cache_value_foreach"]
pub static PLACEHOLDER_00651: u8 = 0;

#[export_name = "g_canonicalize_filename"]
pub static PLACEHOLDER_00654: u8 = 0;

#[export_name = "g_chdir"]
pub static PLACEHOLDER_00657: u8 = 0;

#[export_name = "g_checksum_copy"]
pub static PLACEHOLDER_00660: u8 = 0;

#[export_name = "g_checksum_free"]
pub static PLACEHOLDER_00663: u8 = 0;

#[export_name = "g_checksum_get_digest"]
pub static PLACEHOLDER_00666: u8 = 0;

#[export_name = "g_checksum_get_string"]
pub static PLACEHOLDER_00669: u8 = 0;

#[export_name = "g_checksum_new"]
pub static PLACEHOLDER_00672: u8 = 0;

#[export_name = "g_checksum_reset"]
pub static PLACEHOLDER_00675: u8 = 0;

#[export_name = "g_checksum_type_get_length"]
pub static PLACEHOLDER_00678: u8 = 0;

#[export_name = "g_checksum_update"]
pub static PLACEHOLDER_00681: u8 = 0;

#[export_name = "g_child_watch_add"]
pub static PLACEHOLDER_00684: u8 = 0;

#[export_name = "g_child_watch_add_full"]
pub static PLACEHOLDER_00687: u8 = 0;

#[export_name = "g_child_watch_funcs"]
pub static PLACEHOLDER_00690: u8 = 0;

#[export_name = "g_child_watch_source_new"]
pub static PLACEHOLDER_00693: u8 = 0;

#[export_name = "g_chmod"]
pub static PLACEHOLDER_00696: u8 = 0;

#[export_name = "g_clear_error"]
pub static PLACEHOLDER_00699: u8 = 0;

#[export_name = "g_clear_handle_id"]
pub static PLACEHOLDER_00702: u8 = 0;

#[export_name = "g_clear_list"]
pub static PLACEHOLDER_00705: u8 = 0;

#[export_name = "g_clear_pointer"]
pub static PLACEHOLDER_00708: u8 = 0;

#[export_name = "g_clear_slist"]
pub static PLACEHOLDER_00711: u8 = 0;

#[export_name = "g_close"]
pub static PLACEHOLDER_00714: u8 = 0;

#[export_name = "g_closefrom"]
pub static PLACEHOLDER_00717: u8 = 0;

#[export_name = "g_completion_add_items"]
pub static PLACEHOLDER_00720: u8 = 0;

#[export_name = "g_completion_clear_items"]
pub static PLACEHOLDER_00723: u8 = 0;

#[export_name = "g_completion_complete"]
pub static PLACEHOLDER_00726: u8 = 0;

#[export_name = "g_completion_complete_utf8"]
pub static PLACEHOLDER_00729: u8 = 0;

#[export_name = "g_completion_free"]
pub static PLACEHOLDER_00732: u8 = 0;

#[export_name = "g_completion_new"]
pub static PLACEHOLDER_00735: u8 = 0;

#[export_name = "g_completion_remove_items"]
pub static PLACEHOLDER_00738: u8 = 0;

#[export_name = "g_completion_set_compare"]
pub static PLACEHOLDER_00741: u8 = 0;

#[export_name = "g_compute_checksum_for_bytes"]
pub static PLACEHOLDER_00744: u8 = 0;

#[export_name = "g_compute_checksum_for_data"]
pub static PLACEHOLDER_00747: u8 = 0;

#[export_name = "g_compute_checksum_for_string"]
pub static PLACEHOLDER_00750: u8 = 0;

#[export_name = "g_compute_hmac_for_bytes"]
pub static PLACEHOLDER_00753: u8 = 0;

#[export_name = "g_compute_hmac_for_data"]
pub static PLACEHOLDER_00756: u8 = 0;

#[export_name = "g_compute_hmac_for_string"]
pub static PLACEHOLDER_00759: u8 = 0;

#[export_name = "g_cond_broadcast"]
pub static PLACEHOLDER_00762: u8 = 0;

#[export_name = "g_cond_clear"]
pub static PLACEHOLDER_00765: u8 = 0;

#[export_name = "g_cond_free"]
pub static PLACEHOLDER_00768: u8 = 0;

#[export_name = "g_cond_init"]
pub static PLACEHOLDER_00771: u8 = 0;

#[export_name = "g_cond_new"]
pub static PLACEHOLDER_00774: u8 = 0;

#[export_name = "g_cond_signal"]
pub static PLACEHOLDER_00777: u8 = 0;

#[export_name = "g_cond_timed_wait"]
pub static PLACEHOLDER_00780: u8 = 0;

#[export_name = "g_cond_wait"]
pub static PLACEHOLDER_00783: u8 = 0;

#[export_name = "g_cond_wait_until"]
pub static PLACEHOLDER_00786: u8 = 0;

#[export_name = "g_convert"]
pub static PLACEHOLDER_00789: u8 = 0;

#[export_name = "g_convert_error_quark"]
pub static PLACEHOLDER_00792: u8 = 0;

#[export_name = "g_convert_with_fallback"]
pub static PLACEHOLDER_00795: u8 = 0;

#[export_name = "g_convert_with_iconv"]
pub static PLACEHOLDER_00798: u8 = 0;

#[export_name = "g_creat"]
pub static PLACEHOLDER_00801: u8 = 0;

#[export_name = "g_datalist_clear"]
pub static PLACEHOLDER_00804: u8 = 0;

#[export_name = "g_datalist_foreach"]
pub static PLACEHOLDER_00807: u8 = 0;

#[export_name = "g_datalist_get_data"]
pub static PLACEHOLDER_00810: u8 = 0;

#[export_name = "g_datalist_get_flags"]
pub static PLACEHOLDER_00813: u8 = 0;

#[export_name = "g_datalist_id_dup_data"]
pub static PLACEHOLDER_00816: u8 = 0;

#[export_name = "g_datalist_id_get_data"]
pub static PLACEHOLDER_00819: u8 = 0;

#[export_name = "g_datalist_id_remove_multiple"]
pub static PLACEHOLDER_00822: u8 = 0;

#[export_name = "g_datalist_id_remove_no_notify"]
pub static PLACEHOLDER_00825: u8 = 0;

#[export_name = "g_datalist_id_replace_data"]
pub static PLACEHOLDER_00828: u8 = 0;

#[export_name = "g_datalist_id_set_data_full"]
pub static PLACEHOLDER_00831: u8 = 0;

#[export_name = "g_datalist_init"]
pub static PLACEHOLDER_00834: u8 = 0;

#[export_name = "g_datalist_set_flags"]
pub static PLACEHOLDER_00837: u8 = 0;

#[export_name = "g_datalist_unset_flags"]
pub static PLACEHOLDER_00840: u8 = 0;

#[export_name = "g_dataset_destroy"]
pub static PLACEHOLDER_00843: u8 = 0;

#[export_name = "g_dataset_foreach"]
pub static PLACEHOLDER_00846: u8 = 0;

#[export_name = "g_dataset_id_get_data"]
pub static PLACEHOLDER_00849: u8 = 0;

#[export_name = "g_dataset_id_remove_no_notify"]
pub static PLACEHOLDER_00852: u8 = 0;

#[export_name = "g_dataset_id_set_data_full"]
pub static PLACEHOLDER_00855: u8 = 0;

#[export_name = "g_date_add_days"]
pub static PLACEHOLDER_00858: u8 = 0;

#[export_name = "g_date_add_months"]
pub static PLACEHOLDER_00861: u8 = 0;

#[export_name = "g_date_add_years"]
pub static PLACEHOLDER_00864: u8 = 0;

#[export_name = "g_date_clamp"]
pub static PLACEHOLDER_00867: u8 = 0;

#[export_name = "g_date_clear"]
pub static PLACEHOLDER_00870: u8 = 0;

#[export_name = "g_date_compare"]
pub static PLACEHOLDER_00873: u8 = 0;

#[export_name = "g_date_copy"]
pub static PLACEHOLDER_00876: u8 = 0;

#[export_name = "g_date_days_between"]
pub static PLACEHOLDER_00879: u8 = 0;

#[export_name = "g_date_free"]
pub static PLACEHOLDER_00882: u8 = 0;

#[export_name = "g_date_get_day"]
pub static PLACEHOLDER_00885: u8 = 0;

#[export_name = "g_date_get_day_of_year"]
pub static PLACEHOLDER_00888: u8 = 0;

#[export_name = "g_date_get_days_in_month"]
pub static PLACEHOLDER_00891: u8 = 0;

#[export_name = "g_date_get_iso8601_week_of_year"]
pub static PLACEHOLDER_00894: u8 = 0;

#[export_name = "g_date_get_julian"]
pub static PLACEHOLDER_00897: u8 = 0;

#[export_name = "g_date_get_monday_week_of_year"]
pub static PLACEHOLDER_00900: u8 = 0;

#[export_name = "g_date_get_monday_weeks_in_year"]
pub static PLACEHOLDER_00903: u8 = 0;

#[export_name = "g_date_get_month"]
pub static PLACEHOLDER_00906: u8 = 0;

#[export_name = "g_date_get_sunday_week_of_year"]
pub static PLACEHOLDER_00909: u8 = 0;

#[export_name = "g_date_get_sunday_weeks_in_year"]
pub static PLACEHOLDER_00912: u8 = 0;

#[export_name = "g_date_get_weekday"]
pub static PLACEHOLDER_00915: u8 = 0;

#[export_name = "g_date_get_year"]
pub static PLACEHOLDER_00918: u8 = 0;

#[export_name = "g_date_is_first_of_month"]
pub static PLACEHOLDER_00921: u8 = 0;

#[export_name = "g_date_is_last_of_month"]
pub static PLACEHOLDER_00924: u8 = 0;

#[export_name = "g_date_is_leap_year"]
pub static PLACEHOLDER_00927: u8 = 0;

#[export_name = "g_date_new"]
pub static PLACEHOLDER_00930: u8 = 0;

#[export_name = "g_date_new_dmy"]
pub static PLACEHOLDER_00933: u8 = 0;

#[export_name = "g_date_new_julian"]
pub static PLACEHOLDER_00936: u8 = 0;

#[export_name = "g_date_order"]
pub static PLACEHOLDER_00939: u8 = 0;

#[export_name = "g_date_set_day"]
pub static PLACEHOLDER_00942: u8 = 0;

#[export_name = "g_date_set_dmy"]
pub static PLACEHOLDER_00945: u8 = 0;

#[export_name = "g_date_set_julian"]
pub static PLACEHOLDER_00948: u8 = 0;

#[export_name = "g_date_set_month"]
pub static PLACEHOLDER_00951: u8 = 0;

#[export_name = "g_date_set_parse"]
pub static PLACEHOLDER_00954: u8 = 0;

#[export_name = "g_date_set_time"]
pub static PLACEHOLDER_00957: u8 = 0;

#[export_name = "g_date_set_time_t"]
pub static PLACEHOLDER_00960: u8 = 0;

#[export_name = "g_date_set_time_val"]
pub static PLACEHOLDER_00963: u8 = 0;

#[export_name = "g_date_set_year"]
pub static PLACEHOLDER_00966: u8 = 0;

#[export_name = "g_date_strftime"]
pub static PLACEHOLDER_00969: u8 = 0;

#[export_name = "g_date_subtract_days"]
pub static PLACEHOLDER_00972: u8 = 0;

#[export_name = "g_date_subtract_months"]
pub static PLACEHOLDER_00975: u8 = 0;

#[export_name = "g_date_subtract_years"]
pub static PLACEHOLDER_00978: u8 = 0;

#[export_name = "g_date_time_add"]
pub static PLACEHOLDER_00981: u8 = 0;

#[export_name = "g_date_time_add_days"]
pub static PLACEHOLDER_00984: u8 = 0;

#[export_name = "g_date_time_add_full"]
pub static PLACEHOLDER_00987: u8 = 0;

#[export_name = "g_date_time_add_hours"]
pub static PLACEHOLDER_00990: u8 = 0;

#[export_name = "g_date_time_add_minutes"]
pub static PLACEHOLDER_00993: u8 = 0;

#[export_name = "g_date_time_add_months"]
pub static PLACEHOLDER_00996: u8 = 0;

#[export_name = "g_date_time_add_seconds"]
pub static PLACEHOLDER_00999: u8 = 0;

#[export_name = "g_date_time_add_weeks"]
pub static PLACEHOLDER_01002: u8 = 0;

#[export_name = "g_date_time_add_years"]
pub static PLACEHOLDER_01005: u8 = 0;

#[export_name = "g_date_time_compare"]
pub static PLACEHOLDER_01008: u8 = 0;

#[export_name = "g_date_time_difference"]
pub static PLACEHOLDER_01011: u8 = 0;

#[export_name = "g_date_time_equal"]
pub static PLACEHOLDER_01014: u8 = 0;

#[export_name = "g_date_time_format"]
pub static PLACEHOLDER_01017: u8 = 0;

#[export_name = "g_date_time_format_iso8601"]
pub static PLACEHOLDER_01020: u8 = 0;

#[export_name = "g_date_time_get_day_of_month"]
pub static PLACEHOLDER_01023: u8 = 0;

#[export_name = "g_date_time_get_day_of_week"]
pub static PLACEHOLDER_01026: u8 = 0;

#[export_name = "g_date_time_get_day_of_year"]
pub static PLACEHOLDER_01029: u8 = 0;

#[export_name = "g_date_time_get_hour"]
pub static PLACEHOLDER_01032: u8 = 0;

#[export_name = "g_date_time_get_microsecond"]
pub static PLACEHOLDER_01035: u8 = 0;

#[export_name = "g_date_time_get_minute"]
pub static PLACEHOLDER_01038: u8 = 0;

#[export_name = "g_date_time_get_month"]
pub static PLACEHOLDER_01041: u8 = 0;

#[export_name = "g_date_time_get_second"]
pub static PLACEHOLDER_01044: u8 = 0;

#[export_name = "g_date_time_get_seconds"]
pub static PLACEHOLDER_01047: u8 = 0;

#[export_name = "g_date_time_get_timezone"]
pub static PLACEHOLDER_01050: u8 = 0;

#[export_name = "g_date_time_get_timezone_abbreviation"]
pub static PLACEHOLDER_01053: u8 = 0;

#[export_name = "g_date_time_get_utc_offset"]
pub static PLACEHOLDER_01056: u8 = 0;

#[export_name = "g_date_time_get_week_numbering_year"]
pub static PLACEHOLDER_01059: u8 = 0;

#[export_name = "g_date_time_get_week_of_year"]
pub static PLACEHOLDER_01062: u8 = 0;

#[export_name = "g_date_time_get_year"]
pub static PLACEHOLDER_01065: u8 = 0;

#[export_name = "g_date_time_get_ymd"]
pub static PLACEHOLDER_01068: u8 = 0;

#[export_name = "g_date_time_hash"]
pub static PLACEHOLDER_01071: u8 = 0;

#[export_name = "g_date_time_is_daylight_savings"]
pub static PLACEHOLDER_01074: u8 = 0;

#[export_name = "g_date_time_new"]
pub static PLACEHOLDER_01077: u8 = 0;

#[export_name = "g_date_time_new_from_iso8601"]
pub static PLACEHOLDER_01080: u8 = 0;

#[export_name = "g_date_time_new_from_timeval_local"]
pub static PLACEHOLDER_01083: u8 = 0;

#[export_name = "g_date_time_new_from_timeval_utc"]
pub static PLACEHOLDER_01086: u8 = 0;

#[export_name = "g_date_time_new_from_unix_local"]
pub static PLACEHOLDER_01089: u8 = 0;

#[export_name = "g_date_time_new_from_unix_local_usec"]
pub static PLACEHOLDER_01092: u8 = 0;

#[export_name = "g_date_time_new_from_unix_utc"]
pub static PLACEHOLDER_01095: u8 = 0;

#[export_name = "g_date_time_new_from_unix_utc_usec"]
pub static PLACEHOLDER_01098: u8 = 0;

#[export_name = "g_date_time_new_local"]
pub static PLACEHOLDER_01101: u8 = 0;

#[export_name = "g_date_time_new_now"]
pub static PLACEHOLDER_01104: u8 = 0;

#[export_name = "g_date_time_new_now_local"]
pub static PLACEHOLDER_01107: u8 = 0;

#[export_name = "g_date_time_new_now_utc"]
pub static PLACEHOLDER_01110: u8 = 0;

#[export_name = "g_date_time_new_utc"]
pub static PLACEHOLDER_01113: u8 = 0;

#[export_name = "g_date_time_ref"]
pub static PLACEHOLDER_01116: u8 = 0;

#[export_name = "g_date_time_to_local"]
pub static PLACEHOLDER_01119: u8 = 0;

#[export_name = "g_date_time_to_timeval"]
pub static PLACEHOLDER_01122: u8 = 0;

#[export_name = "g_date_time_to_timezone"]
pub static PLACEHOLDER_01125: u8 = 0;

#[export_name = "g_date_time_to_unix"]
pub static PLACEHOLDER_01128: u8 = 0;

#[export_name = "g_date_time_to_unix_usec"]
pub static PLACEHOLDER_01131: u8 = 0;

#[export_name = "g_date_time_to_utc"]
pub static PLACEHOLDER_01134: u8 = 0;

#[export_name = "g_date_time_unref"]
pub static PLACEHOLDER_01137: u8 = 0;

#[export_name = "g_date_to_struct_tm"]
pub static PLACEHOLDER_01140: u8 = 0;

#[export_name = "g_date_valid"]
pub static PLACEHOLDER_01143: u8 = 0;

#[export_name = "g_date_valid_day"]
pub static PLACEHOLDER_01146: u8 = 0;

#[export_name = "g_date_valid_dmy"]
pub static PLACEHOLDER_01149: u8 = 0;

#[export_name = "g_date_valid_julian"]
pub static PLACEHOLDER_01152: u8 = 0;

#[export_name = "g_date_valid_month"]
pub static PLACEHOLDER_01155: u8 = 0;

#[export_name = "g_date_valid_weekday"]
pub static PLACEHOLDER_01158: u8 = 0;

#[export_name = "g_date_valid_year"]
pub static PLACEHOLDER_01161: u8 = 0;

#[export_name = "g_dcgettext"]
pub static PLACEHOLDER_01164: u8 = 0;

#[export_name = "g_dgettext"]
pub static PLACEHOLDER_01167: u8 = 0;

#[export_name = "g_dir_close"]
pub static PLACEHOLDER_01170: u8 = 0;

#[export_name = "g_dir_make_tmp"]
pub static PLACEHOLDER_01173: u8 = 0;

#[export_name = "g_dir_open"]
pub static PLACEHOLDER_01176: u8 = 0;

#[export_name = "g_dir_read_name"]
pub static PLACEHOLDER_01179: u8 = 0;

#[export_name = "g_dir_ref"]
pub static PLACEHOLDER_01182: u8 = 0;

#[export_name = "g_dir_rewind"]
pub static PLACEHOLDER_01185: u8 = 0;

#[export_name = "g_dir_unref"]
pub static PLACEHOLDER_01188: u8 = 0;

#[export_name = "g_direct_equal"]
pub static PLACEHOLDER_01191: u8 = 0;

#[export_name = "g_direct_hash"]
pub static PLACEHOLDER_01194: u8 = 0;

#[export_name = "g_dngettext"]
pub static PLACEHOLDER_01197: u8 = 0;

#[export_name = "g_double_equal"]
pub static PLACEHOLDER_01200: u8 = 0;

#[export_name = "g_double_hash"]
pub static PLACEHOLDER_01203: u8 = 0;

#[export_name = "g_dpgettext2"]
pub static PLACEHOLDER_01206: u8 = 0;

#[export_name = "g_dpgettext"]
pub static PLACEHOLDER_01209: u8 = 0;

#[export_name = "g_environ_getenv"]
pub static PLACEHOLDER_01212: u8 = 0;

#[export_name = "g_environ_setenv"]
pub static PLACEHOLDER_01215: u8 = 0;

#[export_name = "g_environ_unsetenv"]
pub static PLACEHOLDER_01218: u8 = 0;

#[export_name = "g_error_copy"]
pub static PLACEHOLDER_01221: u8 = 0;

#[export_name = "g_error_domain_register"]
pub static PLACEHOLDER_01224: u8 = 0;

#[export_name = "g_error_domain_register_static"]
pub static PLACEHOLDER_01227: u8 = 0;

#[export_name = "g_error_free"]
pub static PLACEHOLDER_01230: u8 = 0;

#[export_name = "g_error_matches"]
pub static PLACEHOLDER_01233: u8 = 0;

#[export_name = "g_error_new"]
pub static PLACEHOLDER_01236: u8 = 0;

#[export_name = "g_error_new_literal"]
pub static PLACEHOLDER_01239: u8 = 0;

#[export_name = "g_error_new_valist"]
pub static PLACEHOLDER_01242: u8 = 0;

#[export_name = "g_fdwalk_set_cloexec"]
pub static PLACEHOLDER_01245: u8 = 0;

#[export_name = "g_file_error_from_errno"]
pub static PLACEHOLDER_01248: u8 = 0;

#[export_name = "g_file_error_quark"]
pub static PLACEHOLDER_01251: u8 = 0;

#[export_name = "g_file_get_contents"]
pub static PLACEHOLDER_01254: u8 = 0;

#[export_name = "g_file_open_tmp"]
pub static PLACEHOLDER_01257: u8 = 0;

#[export_name = "g_file_read_link"]
pub static PLACEHOLDER_01260: u8 = 0;

#[export_name = "g_file_set_contents"]
pub static PLACEHOLDER_01263: u8 = 0;

#[export_name = "g_file_set_contents_full"]
pub static PLACEHOLDER_01266: u8 = 0;

#[export_name = "g_file_test"]
pub static PLACEHOLDER_01269: u8 = 0;

#[export_name = "g_filename_display_basename"]
pub static PLACEHOLDER_01272: u8 = 0;

#[export_name = "g_filename_display_name"]
pub static PLACEHOLDER_01275: u8 = 0;

#[export_name = "g_filename_from_uri"]
pub static PLACEHOLDER_01278: u8 = 0;

#[export_name = "g_filename_from_utf8"]
pub static PLACEHOLDER_01281: u8 = 0;

#[export_name = "g_filename_to_uri"]
pub static PLACEHOLDER_01284: u8 = 0;

#[export_name = "g_filename_to_utf8"]
pub static PLACEHOLDER_01287: u8 = 0;

#[export_name = "g_find_program_in_path"]
pub static PLACEHOLDER_01290: u8 = 0;

#[export_name = "g_fopen"]
pub static PLACEHOLDER_01293: u8 = 0;

#[export_name = "g_format_size"]
pub static PLACEHOLDER_01296: u8 = 0;

#[export_name = "g_format_size_for_display"]
pub static PLACEHOLDER_01299: u8 = 0;

#[export_name = "g_format_size_full"]
pub static PLACEHOLDER_01302: u8 = 0;

#[export_name = "g_fprintf"]
pub static PLACEHOLDER_01305: u8 = 0;

#[export_name = "g_free"]
pub static PLACEHOLDER_01308: u8 = 0;

#[export_name = "g_free_sized"]
pub static PLACEHOLDER_01311: u8 = 0;

#[export_name = "g_freopen"]
pub static PLACEHOLDER_01314: u8 = 0;

#[export_name = "g_fsync"]
pub static PLACEHOLDER_01317: u8 = 0;

#[export_name = "g_get_application_name"]
pub static PLACEHOLDER_01320: u8 = 0;

#[export_name = "g_get_charset"]
pub static PLACEHOLDER_01323: u8 = 0;

#[export_name = "g_get_codeset"]
pub static PLACEHOLDER_01326: u8 = 0;

#[export_name = "g_get_console_charset"]
pub static PLACEHOLDER_01329: u8 = 0;

#[export_name = "g_get_current_dir"]
pub static PLACEHOLDER_01332: u8 = 0;

#[export_name = "g_get_current_time"]
pub static PLACEHOLDER_01335: u8 = 0;

#[export_name = "g_get_environ"]
pub static PLACEHOLDER_01338: u8 = 0;

#[export_name = "g_get_filename_charsets"]
pub static PLACEHOLDER_01341: u8 = 0;

#[export_name = "g_get_home_dir"]
pub static PLACEHOLDER_01344: u8 = 0;

#[export_name = "g_get_host_name"]
pub static PLACEHOLDER_01347: u8 = 0;

#[export_name = "g_get_language_names"]
pub static PLACEHOLDER_01350: u8 = 0;

#[export_name = "g_get_language_names_with_category"]
pub static PLACEHOLDER_01353: u8 = 0;

#[export_name = "g_get_locale_variants"]
pub static PLACEHOLDER_01356: u8 = 0;

#[export_name = "g_get_monotonic_time"]
pub static PLACEHOLDER_01359: u8 = 0;

#[export_name = "g_get_num_processors"]
pub static PLACEHOLDER_01362: u8 = 0;

#[export_name = "g_get_os_info"]
pub static PLACEHOLDER_01365: u8 = 0;

#[export_name = "g_get_prgname"]
pub static PLACEHOLDER_01368: u8 = 0;

#[export_name = "g_get_real_name"]
pub static PLACEHOLDER_01371: u8 = 0;

#[export_name = "g_get_real_time"]
pub static PLACEHOLDER_01374: u8 = 0;

#[export_name = "g_get_system_config_dirs"]
pub static PLACEHOLDER_01377: u8 = 0;

#[export_name = "g_get_system_data_dirs"]
pub static PLACEHOLDER_01380: u8 = 0;

#[export_name = "g_get_tmp_dir"]
pub static PLACEHOLDER_01383: u8 = 0;

#[export_name = "g_get_user_cache_dir"]
pub static PLACEHOLDER_01386: u8 = 0;

#[export_name = "g_get_user_config_dir"]
pub static PLACEHOLDER_01389: u8 = 0;

#[export_name = "g_get_user_data_dir"]
pub static PLACEHOLDER_01392: u8 = 0;

#[export_name = "g_get_user_name"]
pub static PLACEHOLDER_01395: u8 = 0;

#[export_name = "g_get_user_runtime_dir"]
pub static PLACEHOLDER_01398: u8 = 0;

#[export_name = "g_get_user_special_dir"]
pub static PLACEHOLDER_01401: u8 = 0;

#[export_name = "g_get_user_state_dir"]
pub static PLACEHOLDER_01404: u8 = 0;

#[export_name = "g_getenv"]
pub unsafe extern "C" fn export_g_getenv(
    variable: *const crate::ffi::gchar,
) -> *const crate::ffi::gchar {
    crate::runtime::g_getenv_impl(variable)
}

#[export_name = "g_hash_table_add"]
pub static PLACEHOLDER_01409: u8 = 0;

#[export_name = "g_hash_table_contains"]
pub static PLACEHOLDER_01412: u8 = 0;

#[export_name = "g_hash_table_destroy"]
pub static PLACEHOLDER_01415: u8 = 0;

#[export_name = "g_hash_table_find"]
pub static PLACEHOLDER_01418: u8 = 0;

#[export_name = "g_hash_table_foreach"]
pub static PLACEHOLDER_01421: u8 = 0;

#[export_name = "g_hash_table_foreach_remove"]
pub static PLACEHOLDER_01424: u8 = 0;

#[export_name = "g_hash_table_foreach_steal"]
pub static PLACEHOLDER_01427: u8 = 0;

#[export_name = "g_hash_table_get_keys"]
pub static PLACEHOLDER_01430: u8 = 0;

#[export_name = "g_hash_table_get_keys_as_array"]
pub static PLACEHOLDER_01433: u8 = 0;

#[export_name = "g_hash_table_get_keys_as_ptr_array"]
pub static PLACEHOLDER_01436: u8 = 0;

#[export_name = "g_hash_table_get_values"]
pub static PLACEHOLDER_01439: u8 = 0;

#[export_name = "g_hash_table_get_values_as_ptr_array"]
pub static PLACEHOLDER_01442: u8 = 0;

#[export_name = "g_hash_table_insert"]
pub static PLACEHOLDER_01445: u8 = 0;

#[export_name = "g_hash_table_iter_get_hash_table"]
pub static PLACEHOLDER_01448: u8 = 0;

#[export_name = "g_hash_table_iter_init"]
pub static PLACEHOLDER_01451: u8 = 0;

#[export_name = "g_hash_table_iter_next"]
pub static PLACEHOLDER_01454: u8 = 0;

#[export_name = "g_hash_table_iter_remove"]
pub static PLACEHOLDER_01457: u8 = 0;

#[export_name = "g_hash_table_iter_replace"]
pub static PLACEHOLDER_01460: u8 = 0;

#[export_name = "g_hash_table_iter_steal"]
pub static PLACEHOLDER_01463: u8 = 0;

#[export_name = "g_hash_table_lookup"]
pub static PLACEHOLDER_01466: u8 = 0;

#[export_name = "g_hash_table_lookup_extended"]
pub static PLACEHOLDER_01469: u8 = 0;

#[export_name = "g_hash_table_new"]
pub static PLACEHOLDER_01472: u8 = 0;

#[export_name = "g_hash_table_new_full"]
pub static PLACEHOLDER_01475: u8 = 0;

#[export_name = "g_hash_table_new_similar"]
pub static PLACEHOLDER_01478: u8 = 0;

#[export_name = "g_hash_table_ref"]
pub static PLACEHOLDER_01481: u8 = 0;

#[export_name = "g_hash_table_remove"]
pub static PLACEHOLDER_01484: u8 = 0;

#[export_name = "g_hash_table_remove_all"]
pub static PLACEHOLDER_01487: u8 = 0;

#[export_name = "g_hash_table_replace"]
pub static PLACEHOLDER_01490: u8 = 0;

#[export_name = "g_hash_table_size"]
pub static PLACEHOLDER_01493: u8 = 0;

#[export_name = "g_hash_table_steal"]
pub static PLACEHOLDER_01496: u8 = 0;

#[export_name = "g_hash_table_steal_all"]
pub static PLACEHOLDER_01499: u8 = 0;

#[export_name = "g_hash_table_steal_all_keys"]
pub static PLACEHOLDER_01502: u8 = 0;

#[export_name = "g_hash_table_steal_all_values"]
pub static PLACEHOLDER_01505: u8 = 0;

#[export_name = "g_hash_table_steal_extended"]
pub static PLACEHOLDER_01508: u8 = 0;

#[export_name = "g_hash_table_unref"]
pub static PLACEHOLDER_01511: u8 = 0;

#[export_name = "g_hmac_copy"]
pub static PLACEHOLDER_01514: u8 = 0;

#[export_name = "g_hmac_get_digest"]
pub static PLACEHOLDER_01517: u8 = 0;

#[export_name = "g_hmac_get_string"]
pub static PLACEHOLDER_01520: u8 = 0;

#[export_name = "g_hmac_new"]
pub static PLACEHOLDER_01523: u8 = 0;

#[export_name = "g_hmac_ref"]
pub static PLACEHOLDER_01526: u8 = 0;

#[export_name = "g_hmac_unref"]
pub static PLACEHOLDER_01529: u8 = 0;

#[export_name = "g_hmac_update"]
pub static PLACEHOLDER_01532: u8 = 0;

#[export_name = "g_hook_alloc"]
pub static PLACEHOLDER_01535: u8 = 0;

#[export_name = "g_hook_compare_ids"]
pub static PLACEHOLDER_01538: u8 = 0;

#[export_name = "g_hook_destroy"]
pub static PLACEHOLDER_01541: u8 = 0;

#[export_name = "g_hook_destroy_link"]
pub static PLACEHOLDER_01544: u8 = 0;

#[export_name = "g_hook_find"]
pub static PLACEHOLDER_01547: u8 = 0;

#[export_name = "g_hook_find_data"]
pub static PLACEHOLDER_01550: u8 = 0;

#[export_name = "g_hook_find_func"]
pub static PLACEHOLDER_01553: u8 = 0;

#[export_name = "g_hook_find_func_data"]
pub static PLACEHOLDER_01556: u8 = 0;

#[export_name = "g_hook_first_valid"]
pub static PLACEHOLDER_01559: u8 = 0;

#[export_name = "g_hook_free"]
pub static PLACEHOLDER_01562: u8 = 0;

#[export_name = "g_hook_get"]
pub static PLACEHOLDER_01565: u8 = 0;

#[export_name = "g_hook_insert_before"]
pub static PLACEHOLDER_01568: u8 = 0;

#[export_name = "g_hook_insert_sorted"]
pub static PLACEHOLDER_01571: u8 = 0;

#[export_name = "g_hook_list_clear"]
pub static PLACEHOLDER_01574: u8 = 0;

#[export_name = "g_hook_list_init"]
pub static PLACEHOLDER_01577: u8 = 0;

#[export_name = "g_hook_list_invoke"]
pub static PLACEHOLDER_01580: u8 = 0;

#[export_name = "g_hook_list_invoke_check"]
pub static PLACEHOLDER_01583: u8 = 0;

#[export_name = "g_hook_list_marshal"]
pub static PLACEHOLDER_01586: u8 = 0;

#[export_name = "g_hook_list_marshal_check"]
pub static PLACEHOLDER_01589: u8 = 0;

#[export_name = "g_hook_next_valid"]
pub static PLACEHOLDER_01592: u8 = 0;

#[export_name = "g_hook_prepend"]
pub static PLACEHOLDER_01595: u8 = 0;

#[export_name = "g_hook_ref"]
pub static PLACEHOLDER_01598: u8 = 0;

#[export_name = "g_hook_unref"]
pub static PLACEHOLDER_01601: u8 = 0;

#[export_name = "g_hostname_is_ascii_encoded"]
pub static PLACEHOLDER_01604: u8 = 0;

#[export_name = "g_hostname_is_ip_address"]
pub static PLACEHOLDER_01607: u8 = 0;

#[export_name = "g_hostname_is_non_ascii"]
pub static PLACEHOLDER_01610: u8 = 0;

#[export_name = "g_hostname_to_ascii"]
pub static PLACEHOLDER_01613: u8 = 0;

#[export_name = "g_hostname_to_unicode"]
pub static PLACEHOLDER_01616: u8 = 0;

#[export_name = "g_iconv"]
pub static PLACEHOLDER_01619: u8 = 0;

#[export_name = "g_iconv_close"]
pub static PLACEHOLDER_01622: u8 = 0;

#[export_name = "g_iconv_open"]
pub static PLACEHOLDER_01625: u8 = 0;

#[export_name = "g_idle_add"]
pub static PLACEHOLDER_01628: u8 = 0;

#[export_name = "g_idle_add_full"]
pub static PLACEHOLDER_01631: u8 = 0;

#[export_name = "g_idle_add_once"]
pub static PLACEHOLDER_01634: u8 = 0;

#[export_name = "g_idle_funcs"]
pub static PLACEHOLDER_01637: u8 = 0;

#[export_name = "g_idle_remove_by_data"]
pub static PLACEHOLDER_01640: u8 = 0;

#[export_name = "g_idle_source_new"]
pub static PLACEHOLDER_01643: u8 = 0;

#[export_name = "g_int64_equal"]
pub static PLACEHOLDER_01646: u8 = 0;

#[export_name = "g_int64_hash"]
pub static PLACEHOLDER_01649: u8 = 0;

#[export_name = "g_int_equal"]
pub static PLACEHOLDER_01652: u8 = 0;

#[export_name = "g_int_hash"]
pub static PLACEHOLDER_01655: u8 = 0;

#[export_name = "g_intern_static_string"]
pub static PLACEHOLDER_01658: u8 = 0;

#[export_name = "g_intern_string"]
pub static PLACEHOLDER_01661: u8 = 0;

#[export_name = "g_io_add_watch"]
pub static PLACEHOLDER_01664: u8 = 0;

#[export_name = "g_io_add_watch_full"]
pub static PLACEHOLDER_01667: u8 = 0;

#[export_name = "g_io_channel_close"]
pub static PLACEHOLDER_01670: u8 = 0;

#[export_name = "g_io_channel_error_from_errno"]
pub static PLACEHOLDER_01673: u8 = 0;

#[export_name = "g_io_channel_error_quark"]
pub static PLACEHOLDER_01676: u8 = 0;

#[export_name = "g_io_channel_flush"]
pub static PLACEHOLDER_01679: u8 = 0;

#[export_name = "g_io_channel_get_buffer_condition"]
pub static PLACEHOLDER_01682: u8 = 0;

#[export_name = "g_io_channel_get_buffer_size"]
pub static PLACEHOLDER_01685: u8 = 0;

#[export_name = "g_io_channel_get_buffered"]
pub static PLACEHOLDER_01688: u8 = 0;

#[export_name = "g_io_channel_get_close_on_unref"]
pub static PLACEHOLDER_01691: u8 = 0;

#[export_name = "g_io_channel_get_encoding"]
pub static PLACEHOLDER_01694: u8 = 0;

#[export_name = "g_io_channel_get_flags"]
pub static PLACEHOLDER_01697: u8 = 0;

#[export_name = "g_io_channel_get_line_term"]
pub static PLACEHOLDER_01700: u8 = 0;

#[export_name = "g_io_channel_init"]
pub static PLACEHOLDER_01703: u8 = 0;

#[export_name = "g_io_channel_new_file"]
pub static PLACEHOLDER_01706: u8 = 0;

#[export_name = "g_io_channel_read"]
pub static PLACEHOLDER_01709: u8 = 0;

#[export_name = "g_io_channel_read_chars"]
pub static PLACEHOLDER_01712: u8 = 0;

#[export_name = "g_io_channel_read_line"]
pub static PLACEHOLDER_01715: u8 = 0;

#[export_name = "g_io_channel_read_line_string"]
pub static PLACEHOLDER_01718: u8 = 0;

#[export_name = "g_io_channel_read_to_end"]
pub static PLACEHOLDER_01721: u8 = 0;

#[export_name = "g_io_channel_read_unichar"]
pub static PLACEHOLDER_01724: u8 = 0;

#[export_name = "g_io_channel_ref"]
pub static PLACEHOLDER_01727: u8 = 0;

#[export_name = "g_io_channel_seek"]
pub static PLACEHOLDER_01730: u8 = 0;

#[export_name = "g_io_channel_seek_position"]
pub static PLACEHOLDER_01733: u8 = 0;

#[export_name = "g_io_channel_set_buffer_size"]
pub static PLACEHOLDER_01736: u8 = 0;

#[export_name = "g_io_channel_set_buffered"]
pub static PLACEHOLDER_01739: u8 = 0;

#[export_name = "g_io_channel_set_close_on_unref"]
pub static PLACEHOLDER_01742: u8 = 0;

#[export_name = "g_io_channel_set_encoding"]
pub static PLACEHOLDER_01745: u8 = 0;

#[export_name = "g_io_channel_set_flags"]
pub static PLACEHOLDER_01748: u8 = 0;

#[export_name = "g_io_channel_set_line_term"]
pub static PLACEHOLDER_01751: u8 = 0;

#[export_name = "g_io_channel_shutdown"]
pub static PLACEHOLDER_01754: u8 = 0;

#[export_name = "g_io_channel_unix_get_fd"]
pub static PLACEHOLDER_01757: u8 = 0;

#[export_name = "g_io_channel_unix_new"]
pub static PLACEHOLDER_01760: u8 = 0;

#[export_name = "g_io_channel_unref"]
pub static PLACEHOLDER_01763: u8 = 0;

#[export_name = "g_io_channel_write"]
pub static PLACEHOLDER_01766: u8 = 0;

#[export_name = "g_io_channel_write_chars"]
pub static PLACEHOLDER_01769: u8 = 0;

#[export_name = "g_io_channel_write_unichar"]
pub static PLACEHOLDER_01772: u8 = 0;

#[export_name = "g_io_create_watch"]
pub static PLACEHOLDER_01775: u8 = 0;

#[export_name = "g_io_watch_funcs"]
pub static PLACEHOLDER_01778: u8 = 0;

#[export_name = "g_key_file_error_quark"]
pub static PLACEHOLDER_01781: u8 = 0;

#[export_name = "g_key_file_free"]
pub static PLACEHOLDER_01784: u8 = 0;

#[export_name = "g_key_file_get_boolean"]
pub static PLACEHOLDER_01787: u8 = 0;

#[export_name = "g_key_file_get_boolean_list"]
pub static PLACEHOLDER_01790: u8 = 0;

#[export_name = "g_key_file_get_comment"]
pub static PLACEHOLDER_01793: u8 = 0;

#[export_name = "g_key_file_get_double"]
pub static PLACEHOLDER_01796: u8 = 0;

#[export_name = "g_key_file_get_double_list"]
pub static PLACEHOLDER_01799: u8 = 0;

#[export_name = "g_key_file_get_groups"]
pub static PLACEHOLDER_01802: u8 = 0;

#[export_name = "g_key_file_get_int64"]
pub static PLACEHOLDER_01805: u8 = 0;

#[export_name = "g_key_file_get_integer"]
pub static PLACEHOLDER_01808: u8 = 0;

#[export_name = "g_key_file_get_integer_list"]
pub static PLACEHOLDER_01811: u8 = 0;

#[export_name = "g_key_file_get_keys"]
pub static PLACEHOLDER_01814: u8 = 0;

#[export_name = "g_key_file_get_locale_for_key"]
pub static PLACEHOLDER_01817: u8 = 0;

#[export_name = "g_key_file_get_locale_string"]
pub static PLACEHOLDER_01820: u8 = 0;

#[export_name = "g_key_file_get_locale_string_list"]
pub static PLACEHOLDER_01823: u8 = 0;

#[export_name = "g_key_file_get_start_group"]
pub static PLACEHOLDER_01826: u8 = 0;

#[export_name = "g_key_file_get_string"]
pub static PLACEHOLDER_01829: u8 = 0;

#[export_name = "g_key_file_get_string_list"]
pub static PLACEHOLDER_01832: u8 = 0;

#[export_name = "g_key_file_get_uint64"]
pub static PLACEHOLDER_01835: u8 = 0;

#[export_name = "g_key_file_get_value"]
pub static PLACEHOLDER_01838: u8 = 0;

#[export_name = "g_key_file_has_group"]
pub static PLACEHOLDER_01841: u8 = 0;

#[export_name = "g_key_file_has_key"]
pub static PLACEHOLDER_01844: u8 = 0;

#[export_name = "g_key_file_load_from_bytes"]
pub static PLACEHOLDER_01847: u8 = 0;

#[export_name = "g_key_file_load_from_data"]
pub static PLACEHOLDER_01850: u8 = 0;

#[export_name = "g_key_file_load_from_data_dirs"]
pub static PLACEHOLDER_01853: u8 = 0;

#[export_name = "g_key_file_load_from_dirs"]
pub static PLACEHOLDER_01856: u8 = 0;

#[export_name = "g_key_file_load_from_file"]
pub static PLACEHOLDER_01859: u8 = 0;

#[export_name = "g_key_file_new"]
pub static PLACEHOLDER_01862: u8 = 0;

#[export_name = "g_key_file_ref"]
pub static PLACEHOLDER_01865: u8 = 0;

#[export_name = "g_key_file_remove_comment"]
pub static PLACEHOLDER_01868: u8 = 0;

#[export_name = "g_key_file_remove_group"]
pub static PLACEHOLDER_01871: u8 = 0;

#[export_name = "g_key_file_remove_key"]
pub static PLACEHOLDER_01874: u8 = 0;

#[export_name = "g_key_file_save_to_file"]
pub static PLACEHOLDER_01877: u8 = 0;

#[export_name = "g_key_file_set_boolean"]
pub static PLACEHOLDER_01880: u8 = 0;

#[export_name = "g_key_file_set_boolean_list"]
pub static PLACEHOLDER_01883: u8 = 0;

#[export_name = "g_key_file_set_comment"]
pub static PLACEHOLDER_01886: u8 = 0;

#[export_name = "g_key_file_set_double"]
pub static PLACEHOLDER_01889: u8 = 0;

#[export_name = "g_key_file_set_double_list"]
pub static PLACEHOLDER_01892: u8 = 0;

#[export_name = "g_key_file_set_int64"]
pub static PLACEHOLDER_01895: u8 = 0;

#[export_name = "g_key_file_set_integer"]
pub static PLACEHOLDER_01898: u8 = 0;

#[export_name = "g_key_file_set_integer_list"]
pub static PLACEHOLDER_01901: u8 = 0;

#[export_name = "g_key_file_set_list_separator"]
pub static PLACEHOLDER_01904: u8 = 0;

#[export_name = "g_key_file_set_locale_string"]
pub static PLACEHOLDER_01907: u8 = 0;

#[export_name = "g_key_file_set_locale_string_list"]
pub static PLACEHOLDER_01910: u8 = 0;

#[export_name = "g_key_file_set_string"]
pub static PLACEHOLDER_01913: u8 = 0;

#[export_name = "g_key_file_set_string_list"]
pub static PLACEHOLDER_01916: u8 = 0;

#[export_name = "g_key_file_set_uint64"]
pub static PLACEHOLDER_01919: u8 = 0;

#[export_name = "g_key_file_set_value"]
pub static PLACEHOLDER_01922: u8 = 0;

#[export_name = "g_key_file_to_data"]
pub static PLACEHOLDER_01925: u8 = 0;

#[export_name = "g_key_file_unref"]
pub static PLACEHOLDER_01928: u8 = 0;

#[export_name = "g_list_alloc"]
pub static PLACEHOLDER_01931: u8 = 0;

#[export_name = "g_list_append"]
pub static PLACEHOLDER_01934: u8 = 0;

#[export_name = "g_list_concat"]
pub static PLACEHOLDER_01937: u8 = 0;

#[export_name = "g_list_copy"]
pub static PLACEHOLDER_01940: u8 = 0;

#[export_name = "g_list_copy_deep"]
pub static PLACEHOLDER_01943: u8 = 0;

#[export_name = "g_list_delete_link"]
pub static PLACEHOLDER_01946: u8 = 0;

#[export_name = "g_list_find"]
pub static PLACEHOLDER_01949: u8 = 0;

#[export_name = "g_list_find_custom"]
pub static PLACEHOLDER_01952: u8 = 0;

#[export_name = "g_list_first"]
pub static PLACEHOLDER_01955: u8 = 0;

#[export_name = "g_list_foreach"]
pub static PLACEHOLDER_01958: u8 = 0;

#[export_name = "g_list_free"]
pub static PLACEHOLDER_01961: u8 = 0;

#[export_name = "g_list_free_1"]
pub static PLACEHOLDER_01964: u8 = 0;

#[export_name = "g_list_free_full"]
pub static PLACEHOLDER_01967: u8 = 0;

#[export_name = "g_list_index"]
pub static PLACEHOLDER_01970: u8 = 0;

#[export_name = "g_list_insert"]
pub static PLACEHOLDER_01973: u8 = 0;

#[export_name = "g_list_insert_before"]
pub static PLACEHOLDER_01976: u8 = 0;

#[export_name = "g_list_insert_before_link"]
pub static PLACEHOLDER_01979: u8 = 0;

#[export_name = "g_list_insert_sorted"]
pub static PLACEHOLDER_01982: u8 = 0;

#[export_name = "g_list_insert_sorted_with_data"]
pub static PLACEHOLDER_01985: u8 = 0;

#[export_name = "g_list_last"]
pub static PLACEHOLDER_01988: u8 = 0;

#[export_name = "g_list_length"]
pub static PLACEHOLDER_01991: u8 = 0;

#[export_name = "g_list_nth"]
pub static PLACEHOLDER_01994: u8 = 0;

#[export_name = "g_list_nth_data"]
pub static PLACEHOLDER_01997: u8 = 0;

#[export_name = "g_list_nth_prev"]
pub static PLACEHOLDER_02000: u8 = 0;

#[export_name = "g_list_pop_allocator"]
pub static PLACEHOLDER_02003: u8 = 0;

#[export_name = "g_list_position"]
pub static PLACEHOLDER_02006: u8 = 0;

#[export_name = "g_list_prepend"]
pub static PLACEHOLDER_02009: u8 = 0;

#[export_name = "g_list_push_allocator"]
pub static PLACEHOLDER_02012: u8 = 0;

#[export_name = "g_list_remove"]
pub static PLACEHOLDER_02015: u8 = 0;

#[export_name = "g_list_remove_all"]
pub static PLACEHOLDER_02018: u8 = 0;

#[export_name = "g_list_remove_link"]
pub static PLACEHOLDER_02021: u8 = 0;

#[export_name = "g_list_reverse"]
pub static PLACEHOLDER_02024: u8 = 0;

#[export_name = "g_list_sort"]
pub static PLACEHOLDER_02027: u8 = 0;

#[export_name = "g_list_sort_with_data"]
pub static PLACEHOLDER_02030: u8 = 0;

#[export_name = "g_listenv"]
pub static PLACEHOLDER_02033: u8 = 0;

#[export_name = "g_locale_from_utf8"]
pub static PLACEHOLDER_02036: u8 = 0;

#[export_name = "g_locale_to_utf8"]
pub static PLACEHOLDER_02039: u8 = 0;

#[export_name = "g_log"]
pub static PLACEHOLDER_02042: u8 = 0;

#[export_name = "g_log_default_handler"]
pub static PLACEHOLDER_02045: u8 = 0;

#[export_name = "g_log_get_debug_enabled"]
pub static PLACEHOLDER_02048: u8 = 0;

#[export_name = "g_log_remove_handler"]
pub static PLACEHOLDER_02051: u8 = 0;

#[export_name = "g_log_set_always_fatal"]
pub static PLACEHOLDER_02054: u8 = 0;

#[export_name = "g_log_set_debug_enabled"]
pub static PLACEHOLDER_02057: u8 = 0;

#[export_name = "g_log_set_default_handler"]
pub static PLACEHOLDER_02060: u8 = 0;

#[export_name = "g_log_set_fatal_mask"]
pub static PLACEHOLDER_02063: u8 = 0;

#[export_name = "g_log_set_handler"]
pub static PLACEHOLDER_02066: u8 = 0;

#[export_name = "g_log_set_handler_full"]
pub static PLACEHOLDER_02069: u8 = 0;

#[export_name = "g_log_set_writer_func"]
pub static PLACEHOLDER_02072: u8 = 0;

#[export_name = "g_log_structured"]
pub static PLACEHOLDER_02075: u8 = 0;

#[export_name = "g_log_structured_array"]
pub static PLACEHOLDER_02078: u8 = 0;

#[export_name = "g_log_structured_standard"]
pub static PLACEHOLDER_02081: u8 = 0;

#[export_name = "g_log_variant"]
pub static PLACEHOLDER_02084: u8 = 0;

#[export_name = "g_log_writer_default"]
pub static PLACEHOLDER_02087: u8 = 0;

#[export_name = "g_log_writer_default_set_debug_domains"]
pub static PLACEHOLDER_02090: u8 = 0;

#[export_name = "g_log_writer_default_set_use_stderr"]
pub static PLACEHOLDER_02093: u8 = 0;

#[export_name = "g_log_writer_default_would_drop"]
pub static PLACEHOLDER_02096: u8 = 0;

#[export_name = "g_log_writer_format_fields"]
pub static PLACEHOLDER_02099: u8 = 0;

#[export_name = "g_log_writer_is_journald"]
pub static PLACEHOLDER_02102: u8 = 0;

#[export_name = "g_log_writer_journald"]
pub static PLACEHOLDER_02105: u8 = 0;

#[export_name = "g_log_writer_standard_streams"]
pub static PLACEHOLDER_02108: u8 = 0;

#[export_name = "g_log_writer_supports_color"]
pub static PLACEHOLDER_02111: u8 = 0;

#[export_name = "g_log_writer_syslog"]
pub static PLACEHOLDER_02114: u8 = 0;

#[export_name = "g_logv"]
pub static PLACEHOLDER_02117: u8 = 0;

#[export_name = "g_lstat"]
pub static PLACEHOLDER_02120: u8 = 0;

#[export_name = "g_main_context_acquire"]
pub static PLACEHOLDER_02123: u8 = 0;

#[export_name = "g_main_context_add_poll"]
pub static PLACEHOLDER_02126: u8 = 0;

#[export_name = "g_main_context_check"]
pub static PLACEHOLDER_02129: u8 = 0;

#[export_name = "g_main_context_default"]
pub static PLACEHOLDER_02132: u8 = 0;

#[export_name = "g_main_context_dispatch"]
pub static PLACEHOLDER_02135: u8 = 0;

#[export_name = "g_main_context_find_source_by_funcs_user_data"]
pub static PLACEHOLDER_02138: u8 = 0;

#[export_name = "g_main_context_find_source_by_id"]
pub static PLACEHOLDER_02141: u8 = 0;

#[export_name = "g_main_context_find_source_by_user_data"]
pub static PLACEHOLDER_02144: u8 = 0;

#[export_name = "g_main_context_get_poll_func"]
pub static PLACEHOLDER_02147: u8 = 0;

#[export_name = "g_main_context_get_thread_default"]
pub static PLACEHOLDER_02150: u8 = 0;

#[export_name = "g_main_context_invoke"]
pub static PLACEHOLDER_02153: u8 = 0;

#[export_name = "g_main_context_invoke_full"]
pub static PLACEHOLDER_02156: u8 = 0;

#[export_name = "g_main_context_is_owner"]
pub static PLACEHOLDER_02159: u8 = 0;

#[export_name = "g_main_context_iteration"]
pub static PLACEHOLDER_02162: u8 = 0;

#[export_name = "g_main_context_new"]
pub static PLACEHOLDER_02165: u8 = 0;

#[export_name = "g_main_context_new_with_flags"]
pub static PLACEHOLDER_02168: u8 = 0;

#[export_name = "g_main_context_pending"]
pub static PLACEHOLDER_02171: u8 = 0;

#[export_name = "g_main_context_pop_thread_default"]
pub static PLACEHOLDER_02174: u8 = 0;

#[export_name = "g_main_context_prepare"]
pub static PLACEHOLDER_02177: u8 = 0;

#[export_name = "g_main_context_push_thread_default"]
pub static PLACEHOLDER_02180: u8 = 0;

#[export_name = "g_main_context_query"]
pub static PLACEHOLDER_02183: u8 = 0;

#[export_name = "g_main_context_ref"]
pub static PLACEHOLDER_02186: u8 = 0;

#[export_name = "g_main_context_ref_thread_default"]
pub static PLACEHOLDER_02189: u8 = 0;

#[export_name = "g_main_context_release"]
pub static PLACEHOLDER_02192: u8 = 0;

#[export_name = "g_main_context_remove_poll"]
pub static PLACEHOLDER_02195: u8 = 0;

#[export_name = "g_main_context_set_poll_func"]
pub static PLACEHOLDER_02198: u8 = 0;

#[export_name = "g_main_context_unref"]
pub static PLACEHOLDER_02201: u8 = 0;

#[export_name = "g_main_context_wait"]
pub static PLACEHOLDER_02204: u8 = 0;

#[export_name = "g_main_context_wakeup"]
pub static PLACEHOLDER_02207: u8 = 0;

#[export_name = "g_main_current_source"]
pub static PLACEHOLDER_02210: u8 = 0;

#[export_name = "g_main_depth"]
pub static PLACEHOLDER_02213: u8 = 0;

#[export_name = "g_main_loop_get_context"]
pub static PLACEHOLDER_02216: u8 = 0;

#[export_name = "g_main_loop_is_running"]
pub static PLACEHOLDER_02219: u8 = 0;

#[export_name = "g_main_loop_new"]
pub static PLACEHOLDER_02222: u8 = 0;

#[export_name = "g_main_loop_quit"]
pub static PLACEHOLDER_02225: u8 = 0;

#[export_name = "g_main_loop_ref"]
pub static PLACEHOLDER_02228: u8 = 0;

#[export_name = "g_main_loop_run"]
pub static PLACEHOLDER_02231: u8 = 0;

#[export_name = "g_main_loop_unref"]
pub static PLACEHOLDER_02234: u8 = 0;

#[export_name = "g_malloc0"]
pub static PLACEHOLDER_02237: u8 = 0;

#[export_name = "g_malloc0_n"]
pub static PLACEHOLDER_02240: u8 = 0;

#[export_name = "g_malloc"]
pub static PLACEHOLDER_02243: u8 = 0;

#[export_name = "g_malloc_n"]
pub static PLACEHOLDER_02246: u8 = 0;

#[export_name = "g_mapped_file_free"]
pub static PLACEHOLDER_02249: u8 = 0;

#[export_name = "g_mapped_file_get_bytes"]
pub static PLACEHOLDER_02252: u8 = 0;

#[export_name = "g_mapped_file_get_contents"]
pub static PLACEHOLDER_02255: u8 = 0;

#[export_name = "g_mapped_file_get_length"]
pub static PLACEHOLDER_02258: u8 = 0;

#[export_name = "g_mapped_file_new"]
pub static PLACEHOLDER_02261: u8 = 0;

#[export_name = "g_mapped_file_new_from_fd"]
pub static PLACEHOLDER_02264: u8 = 0;

#[export_name = "g_mapped_file_ref"]
pub static PLACEHOLDER_02267: u8 = 0;

#[export_name = "g_mapped_file_unref"]
pub static PLACEHOLDER_02270: u8 = 0;

#[export_name = "g_markup_collect_attributes"]
pub static PLACEHOLDER_02273: u8 = 0;

#[export_name = "g_markup_error_quark"]
pub static PLACEHOLDER_02276: u8 = 0;

#[export_name = "g_markup_escape_text"]
pub static PLACEHOLDER_02279: u8 = 0;

#[export_name = "g_markup_parse_context_end_parse"]
pub static PLACEHOLDER_02282: u8 = 0;

#[export_name = "g_markup_parse_context_free"]
pub static PLACEHOLDER_02285: u8 = 0;

#[export_name = "g_markup_parse_context_get_element"]
pub static PLACEHOLDER_02288: u8 = 0;

#[export_name = "g_markup_parse_context_get_element_stack"]
pub static PLACEHOLDER_02291: u8 = 0;

#[export_name = "g_markup_parse_context_get_position"]
pub static PLACEHOLDER_02294: u8 = 0;

#[export_name = "g_markup_parse_context_get_user_data"]
pub static PLACEHOLDER_02297: u8 = 0;

#[export_name = "g_markup_parse_context_new"]
pub static PLACEHOLDER_02300: u8 = 0;

#[export_name = "g_markup_parse_context_parse"]
pub static PLACEHOLDER_02303: u8 = 0;

#[export_name = "g_markup_parse_context_pop"]
pub static PLACEHOLDER_02306: u8 = 0;

#[export_name = "g_markup_parse_context_push"]
pub static PLACEHOLDER_02309: u8 = 0;

#[export_name = "g_markup_parse_context_ref"]
pub static PLACEHOLDER_02312: u8 = 0;

#[export_name = "g_markup_parse_context_unref"]
pub static PLACEHOLDER_02315: u8 = 0;

#[export_name = "g_markup_printf_escaped"]
pub static PLACEHOLDER_02318: u8 = 0;

#[export_name = "g_markup_vprintf_escaped"]
pub static PLACEHOLDER_02321: u8 = 0;

#[export_name = "g_match_info_expand_references"]
pub static PLACEHOLDER_02324: u8 = 0;

#[export_name = "g_match_info_fetch"]
pub static PLACEHOLDER_02327: u8 = 0;

#[export_name = "g_match_info_fetch_all"]
pub static PLACEHOLDER_02330: u8 = 0;

#[export_name = "g_match_info_fetch_named"]
pub static PLACEHOLDER_02333: u8 = 0;

#[export_name = "g_match_info_fetch_named_pos"]
pub static PLACEHOLDER_02336: u8 = 0;

#[export_name = "g_match_info_fetch_pos"]
pub static PLACEHOLDER_02339: u8 = 0;

#[export_name = "g_match_info_free"]
pub static PLACEHOLDER_02342: u8 = 0;

#[export_name = "g_match_info_get_match_count"]
pub static PLACEHOLDER_02345: u8 = 0;

#[export_name = "g_match_info_get_regex"]
pub static PLACEHOLDER_02348: u8 = 0;

#[export_name = "g_match_info_get_string"]
pub static PLACEHOLDER_02351: u8 = 0;

#[export_name = "g_match_info_is_partial_match"]
pub static PLACEHOLDER_02354: u8 = 0;

#[export_name = "g_match_info_matches"]
pub static PLACEHOLDER_02357: u8 = 0;

#[export_name = "g_match_info_next"]
pub static PLACEHOLDER_02360: u8 = 0;

#[export_name = "g_match_info_ref"]
pub static PLACEHOLDER_02363: u8 = 0;

#[export_name = "g_match_info_unref"]
pub static PLACEHOLDER_02366: u8 = 0;

#[export_name = "g_mem_chunk_alloc0"]
pub static PLACEHOLDER_02369: u8 = 0;

#[export_name = "g_mem_chunk_alloc"]
pub static PLACEHOLDER_02372: u8 = 0;

#[export_name = "g_mem_chunk_clean"]
pub static PLACEHOLDER_02375: u8 = 0;

#[export_name = "g_mem_chunk_destroy"]
pub static PLACEHOLDER_02378: u8 = 0;

#[export_name = "g_mem_chunk_free"]
pub static PLACEHOLDER_02381: u8 = 0;

#[export_name = "g_mem_chunk_info"]
pub static PLACEHOLDER_02384: u8 = 0;

#[export_name = "g_mem_chunk_new"]
pub static PLACEHOLDER_02387: u8 = 0;

#[export_name = "g_mem_chunk_print"]
pub static PLACEHOLDER_02390: u8 = 0;

#[export_name = "g_mem_chunk_reset"]
pub static PLACEHOLDER_02393: u8 = 0;

#[export_name = "g_mem_gc_friendly"]
pub static PLACEHOLDER_02396: u8 = 0;

#[export_name = "g_mem_is_system_malloc"]
pub static PLACEHOLDER_02399: u8 = 0;

#[export_name = "g_mem_profile"]
pub static PLACEHOLDER_02402: u8 = 0;

#[export_name = "g_mem_set_vtable"]
pub static PLACEHOLDER_02405: u8 = 0;

#[export_name = "g_memdup2"]
pub static PLACEHOLDER_02408: u8 = 0;

#[export_name = "g_memdup"]
pub static PLACEHOLDER_02411: u8 = 0;

#[export_name = "g_mkdir"]
pub static PLACEHOLDER_02414: u8 = 0;

#[export_name = "g_mkdir_with_parents"]
pub static PLACEHOLDER_02417: u8 = 0;

#[export_name = "g_mkdtemp"]
pub static PLACEHOLDER_02420: u8 = 0;

#[export_name = "g_mkdtemp_full"]
pub static PLACEHOLDER_02423: u8 = 0;

#[export_name = "g_mkstemp"]
pub static PLACEHOLDER_02426: u8 = 0;

#[export_name = "g_mkstemp_full"]
pub static PLACEHOLDER_02429: u8 = 0;

#[export_name = "g_mutex_clear"]
pub static PLACEHOLDER_02432: u8 = 0;

#[export_name = "g_mutex_free"]
pub static PLACEHOLDER_02435: u8 = 0;

#[export_name = "g_mutex_init"]
pub static PLACEHOLDER_02438: u8 = 0;

#[export_name = "g_mutex_lock"]
pub static PLACEHOLDER_02441: u8 = 0;

#[export_name = "g_mutex_new"]
pub static PLACEHOLDER_02444: u8 = 0;

#[export_name = "g_mutex_trylock"]
pub static PLACEHOLDER_02447: u8 = 0;

#[export_name = "g_mutex_unlock"]
pub static PLACEHOLDER_02450: u8 = 0;

#[export_name = "g_node_child_index"]
pub static PLACEHOLDER_02453: u8 = 0;

#[export_name = "g_node_child_position"]
pub static PLACEHOLDER_02456: u8 = 0;

#[export_name = "g_node_children_foreach"]
pub static PLACEHOLDER_02459: u8 = 0;

#[export_name = "g_node_copy"]
pub static PLACEHOLDER_02462: u8 = 0;

#[export_name = "g_node_copy_deep"]
pub static PLACEHOLDER_02465: u8 = 0;

#[export_name = "g_node_depth"]
pub static PLACEHOLDER_02468: u8 = 0;

#[export_name = "g_node_destroy"]
pub static PLACEHOLDER_02471: u8 = 0;

#[export_name = "g_node_find"]
pub static PLACEHOLDER_02474: u8 = 0;

#[export_name = "g_node_find_child"]
pub static PLACEHOLDER_02477: u8 = 0;

#[export_name = "g_node_first_sibling"]
pub static PLACEHOLDER_02480: u8 = 0;

#[export_name = "g_node_get_root"]
pub static PLACEHOLDER_02483: u8 = 0;

#[export_name = "g_node_insert"]
pub static PLACEHOLDER_02486: u8 = 0;

#[export_name = "g_node_insert_after"]
pub static PLACEHOLDER_02489: u8 = 0;

#[export_name = "g_node_insert_before"]
pub static PLACEHOLDER_02492: u8 = 0;

#[export_name = "g_node_is_ancestor"]
pub static PLACEHOLDER_02495: u8 = 0;

#[export_name = "g_node_last_child"]
pub static PLACEHOLDER_02498: u8 = 0;

#[export_name = "g_node_last_sibling"]
pub static PLACEHOLDER_02501: u8 = 0;

#[export_name = "g_node_max_height"]
pub static PLACEHOLDER_02504: u8 = 0;

#[export_name = "g_node_n_children"]
pub static PLACEHOLDER_02507: u8 = 0;

#[export_name = "g_node_n_nodes"]
pub static PLACEHOLDER_02510: u8 = 0;

#[export_name = "g_node_new"]
pub static PLACEHOLDER_02513: u8 = 0;

#[export_name = "g_node_nth_child"]
pub static PLACEHOLDER_02516: u8 = 0;

#[export_name = "g_node_pop_allocator"]
pub static PLACEHOLDER_02519: u8 = 0;

#[export_name = "g_node_prepend"]
pub static PLACEHOLDER_02522: u8 = 0;

#[export_name = "g_node_push_allocator"]
pub static PLACEHOLDER_02525: u8 = 0;

#[export_name = "g_node_reverse_children"]
pub static PLACEHOLDER_02528: u8 = 0;

#[export_name = "g_node_traverse"]
pub static PLACEHOLDER_02531: u8 = 0;

#[export_name = "g_node_unlink"]
pub static PLACEHOLDER_02534: u8 = 0;

#[export_name = "g_nullify_pointer"]
pub static PLACEHOLDER_02537: u8 = 0;

#[export_name = "g_number_parser_error_quark"]
pub static PLACEHOLDER_02540: u8 = 0;

#[export_name = "g_on_error_query"]
pub static PLACEHOLDER_02543: u8 = 0;

#[export_name = "g_on_error_stack_trace"]
pub static PLACEHOLDER_02546: u8 = 0;

#[export_name = "g_once_impl"]
pub static PLACEHOLDER_02549: u8 = 0;

#[export_name = "g_once_init_enter"]
pub static PLACEHOLDER_02552: u8 = 0;

#[export_name = "g_once_init_enter_impl"]
pub static PLACEHOLDER_02555: u8 = 0;

#[export_name = "g_once_init_enter_pointer"]
pub static PLACEHOLDER_02558: u8 = 0;

#[export_name = "g_once_init_leave"]
pub static PLACEHOLDER_02561: u8 = 0;

#[export_name = "g_once_init_leave_pointer"]
pub static PLACEHOLDER_02564: u8 = 0;

#[export_name = "g_open"]
pub static PLACEHOLDER_02567: u8 = 0;

#[export_name = "g_option_context_add_group"]
pub static PLACEHOLDER_02570: u8 = 0;

#[export_name = "g_option_context_add_main_entries"]
pub static PLACEHOLDER_02573: u8 = 0;

#[export_name = "g_option_context_free"]
pub static PLACEHOLDER_02576: u8 = 0;

#[export_name = "g_option_context_get_description"]
pub static PLACEHOLDER_02579: u8 = 0;

#[export_name = "g_option_context_get_help"]
pub static PLACEHOLDER_02582: u8 = 0;

#[export_name = "g_option_context_get_help_enabled"]
pub static PLACEHOLDER_02585: u8 = 0;

#[export_name = "g_option_context_get_ignore_unknown_options"]
pub static PLACEHOLDER_02588: u8 = 0;

#[export_name = "g_option_context_get_main_group"]
pub static PLACEHOLDER_02591: u8 = 0;

#[export_name = "g_option_context_get_strict_posix"]
pub static PLACEHOLDER_02594: u8 = 0;

#[export_name = "g_option_context_get_summary"]
pub static PLACEHOLDER_02597: u8 = 0;

#[export_name = "g_option_context_new"]
pub static PLACEHOLDER_02600: u8 = 0;

#[export_name = "g_option_context_parse"]
pub static PLACEHOLDER_02603: u8 = 0;

#[export_name = "g_option_context_parse_strv"]
pub static PLACEHOLDER_02606: u8 = 0;

#[export_name = "g_option_context_set_description"]
pub static PLACEHOLDER_02609: u8 = 0;

#[export_name = "g_option_context_set_help_enabled"]
pub static PLACEHOLDER_02612: u8 = 0;

#[export_name = "g_option_context_set_ignore_unknown_options"]
pub static PLACEHOLDER_02615: u8 = 0;

#[export_name = "g_option_context_set_main_group"]
pub static PLACEHOLDER_02618: u8 = 0;

#[export_name = "g_option_context_set_strict_posix"]
pub static PLACEHOLDER_02621: u8 = 0;

#[export_name = "g_option_context_set_summary"]
pub static PLACEHOLDER_02624: u8 = 0;

#[export_name = "g_option_context_set_translate_func"]
pub static PLACEHOLDER_02627: u8 = 0;

#[export_name = "g_option_context_set_translation_domain"]
pub static PLACEHOLDER_02630: u8 = 0;

#[export_name = "g_option_error_quark"]
pub static PLACEHOLDER_02633: u8 = 0;

#[export_name = "g_option_group_add_entries"]
pub static PLACEHOLDER_02636: u8 = 0;

#[export_name = "g_option_group_free"]
pub static PLACEHOLDER_02639: u8 = 0;

#[export_name = "g_option_group_new"]
pub static PLACEHOLDER_02642: u8 = 0;

#[export_name = "g_option_group_ref"]
pub static PLACEHOLDER_02645: u8 = 0;

#[export_name = "g_option_group_set_error_hook"]
pub static PLACEHOLDER_02648: u8 = 0;

#[export_name = "g_option_group_set_parse_hooks"]
pub static PLACEHOLDER_02651: u8 = 0;

#[export_name = "g_option_group_set_translate_func"]
pub static PLACEHOLDER_02654: u8 = 0;

#[export_name = "g_option_group_set_translation_domain"]
pub static PLACEHOLDER_02657: u8 = 0;

#[export_name = "g_option_group_unref"]
pub static PLACEHOLDER_02660: u8 = 0;

#[export_name = "g_parse_debug_string"]
pub static PLACEHOLDER_02663: u8 = 0;

#[export_name = "g_path_buf_clear"]
pub static PLACEHOLDER_02666: u8 = 0;

#[export_name = "g_path_buf_clear_to_path"]
pub static PLACEHOLDER_02669: u8 = 0;

#[export_name = "g_path_buf_copy"]
pub static PLACEHOLDER_02672: u8 = 0;

#[export_name = "g_path_buf_equal"]
pub static PLACEHOLDER_02675: u8 = 0;

#[export_name = "g_path_buf_free"]
pub static PLACEHOLDER_02678: u8 = 0;

#[export_name = "g_path_buf_free_to_path"]
pub static PLACEHOLDER_02681: u8 = 0;

#[export_name = "g_path_buf_init"]
pub static PLACEHOLDER_02684: u8 = 0;

#[export_name = "g_path_buf_init_from_path"]
pub static PLACEHOLDER_02687: u8 = 0;

#[export_name = "g_path_buf_new"]
pub static PLACEHOLDER_02690: u8 = 0;

#[export_name = "g_path_buf_new_from_path"]
pub static PLACEHOLDER_02693: u8 = 0;

#[export_name = "g_path_buf_pop"]
pub static PLACEHOLDER_02696: u8 = 0;

#[export_name = "g_path_buf_push"]
pub static PLACEHOLDER_02699: u8 = 0;

#[export_name = "g_path_buf_set_extension"]
pub static PLACEHOLDER_02702: u8 = 0;

#[export_name = "g_path_buf_set_filename"]
pub static PLACEHOLDER_02705: u8 = 0;

#[export_name = "g_path_buf_to_path"]
pub static PLACEHOLDER_02708: u8 = 0;

#[export_name = "g_path_get_basename"]
pub static PLACEHOLDER_02711: u8 = 0;

#[export_name = "g_path_get_dirname"]
pub static PLACEHOLDER_02714: u8 = 0;

#[export_name = "g_path_is_absolute"]
pub static PLACEHOLDER_02717: u8 = 0;

#[export_name = "g_path_skip_root"]
pub static PLACEHOLDER_02720: u8 = 0;

#[export_name = "g_pattern_match"]
pub static PLACEHOLDER_02723: u8 = 0;

#[export_name = "g_pattern_match_simple"]
pub static PLACEHOLDER_02726: u8 = 0;

#[export_name = "g_pattern_match_string"]
pub static PLACEHOLDER_02729: u8 = 0;

#[export_name = "g_pattern_spec_copy"]
pub static PLACEHOLDER_02732: u8 = 0;

#[export_name = "g_pattern_spec_equal"]
pub static PLACEHOLDER_02735: u8 = 0;

#[export_name = "g_pattern_spec_free"]
pub static PLACEHOLDER_02738: u8 = 0;

#[export_name = "g_pattern_spec_match"]
pub static PLACEHOLDER_02741: u8 = 0;

#[export_name = "g_pattern_spec_match_string"]
pub static PLACEHOLDER_02744: u8 = 0;

#[export_name = "g_pattern_spec_new"]
pub static PLACEHOLDER_02747: u8 = 0;

#[export_name = "g_pointer_bit_lock"]
pub static PLACEHOLDER_02750: u8 = 0;

#[export_name = "g_pointer_bit_lock_and_get"]
pub static PLACEHOLDER_02753: u8 = 0;

#[export_name = "g_pointer_bit_lock_mask_ptr"]
pub static PLACEHOLDER_02756: u8 = 0;

#[export_name = "g_pointer_bit_trylock"]
pub static PLACEHOLDER_02759: u8 = 0;

#[export_name = "g_pointer_bit_unlock"]
pub static PLACEHOLDER_02762: u8 = 0;

#[export_name = "g_pointer_bit_unlock_and_set"]
pub static PLACEHOLDER_02765: u8 = 0;

#[export_name = "g_poll"]
pub static PLACEHOLDER_02768: u8 = 0;

#[export_name = "g_prefix_error"]
pub static PLACEHOLDER_02771: u8 = 0;

#[export_name = "g_prefix_error_literal"]
pub static PLACEHOLDER_02774: u8 = 0;

#[export_name = "g_print"]
pub static PLACEHOLDER_02777: u8 = 0;

#[export_name = "g_printerr"]
pub static PLACEHOLDER_02780: u8 = 0;

#[export_name = "g_printf"]
pub static PLACEHOLDER_02783: u8 = 0;

#[export_name = "g_printf_string_upper_bound"]
pub static PLACEHOLDER_02786: u8 = 0;

#[export_name = "g_private_get"]
pub static PLACEHOLDER_02789: u8 = 0;

#[export_name = "g_private_new"]
pub static PLACEHOLDER_02792: u8 = 0;

#[export_name = "g_private_replace"]
pub static PLACEHOLDER_02795: u8 = 0;

#[export_name = "g_private_set"]
pub static PLACEHOLDER_02798: u8 = 0;

#[export_name = "g_propagate_error"]
pub static PLACEHOLDER_02801: u8 = 0;

#[export_name = "g_propagate_prefixed_error"]
pub static PLACEHOLDER_02804: u8 = 0;

#[export_name = "g_ptr_array_add"]
pub static PLACEHOLDER_02807: u8 = 0;

#[export_name = "g_ptr_array_copy"]
pub static PLACEHOLDER_02810: u8 = 0;

#[export_name = "g_ptr_array_extend"]
pub static PLACEHOLDER_02813: u8 = 0;

#[export_name = "g_ptr_array_extend_and_steal"]
pub static PLACEHOLDER_02816: u8 = 0;

#[export_name = "g_ptr_array_find"]
pub static PLACEHOLDER_02819: u8 = 0;

#[export_name = "g_ptr_array_find_with_equal_func"]
pub static PLACEHOLDER_02822: u8 = 0;

#[export_name = "g_ptr_array_foreach"]
pub static PLACEHOLDER_02825: u8 = 0;

#[export_name = "g_ptr_array_free"]
pub static PLACEHOLDER_02828: u8 = 0;

#[export_name = "g_ptr_array_insert"]
pub static PLACEHOLDER_02831: u8 = 0;

#[export_name = "g_ptr_array_is_null_terminated"]
pub static PLACEHOLDER_02834: u8 = 0;

#[export_name = "g_ptr_array_new"]
pub static PLACEHOLDER_02837: u8 = 0;

#[export_name = "g_ptr_array_new_from_array"]
pub static PLACEHOLDER_02840: u8 = 0;

#[export_name = "g_ptr_array_new_from_null_terminated_array"]
pub static PLACEHOLDER_02843: u8 = 0;

#[export_name = "g_ptr_array_new_full"]
pub static PLACEHOLDER_02846: u8 = 0;

#[export_name = "g_ptr_array_new_null_terminated"]
pub static PLACEHOLDER_02849: u8 = 0;

#[export_name = "g_ptr_array_new_take"]
pub static PLACEHOLDER_02852: u8 = 0;

#[export_name = "g_ptr_array_new_take_null_terminated"]
pub static PLACEHOLDER_02855: u8 = 0;

#[export_name = "g_ptr_array_new_with_free_func"]
pub static PLACEHOLDER_02858: u8 = 0;

#[export_name = "g_ptr_array_ref"]
pub static PLACEHOLDER_02861: u8 = 0;

#[export_name = "g_ptr_array_remove"]
pub static PLACEHOLDER_02864: u8 = 0;

#[export_name = "g_ptr_array_remove_fast"]
pub static PLACEHOLDER_02867: u8 = 0;

#[export_name = "g_ptr_array_remove_index"]
pub static PLACEHOLDER_02870: u8 = 0;

#[export_name = "g_ptr_array_remove_index_fast"]
pub static PLACEHOLDER_02873: u8 = 0;

#[export_name = "g_ptr_array_remove_range"]
pub static PLACEHOLDER_02876: u8 = 0;

#[export_name = "g_ptr_array_set_free_func"]
pub static PLACEHOLDER_02879: u8 = 0;

#[export_name = "g_ptr_array_set_size"]
pub static PLACEHOLDER_02882: u8 = 0;

#[export_name = "g_ptr_array_sized_new"]
pub static PLACEHOLDER_02885: u8 = 0;

#[export_name = "g_ptr_array_sort"]
pub static PLACEHOLDER_02888: u8 = 0;

#[export_name = "g_ptr_array_sort_values"]
pub static PLACEHOLDER_02891: u8 = 0;

#[export_name = "g_ptr_array_sort_values_with_data"]
pub static PLACEHOLDER_02894: u8 = 0;

#[export_name = "g_ptr_array_sort_with_data"]
pub static PLACEHOLDER_02897: u8 = 0;

#[export_name = "g_ptr_array_steal"]
pub static PLACEHOLDER_02900: u8 = 0;

#[export_name = "g_ptr_array_steal_index"]
pub static PLACEHOLDER_02903: u8 = 0;

#[export_name = "g_ptr_array_steal_index_fast"]
pub static PLACEHOLDER_02906: u8 = 0;

#[export_name = "g_ptr_array_unref"]
pub static PLACEHOLDER_02909: u8 = 0;

#[export_name = "g_qsort_with_data"]
pub static PLACEHOLDER_02912: u8 = 0;

#[export_name = "g_quark_from_static_string"]
pub static PLACEHOLDER_02915: u8 = 0;

#[export_name = "g_quark_from_string"]
pub static PLACEHOLDER_02918: u8 = 0;

#[export_name = "g_quark_to_string"]
pub static PLACEHOLDER_02921: u8 = 0;

#[export_name = "g_quark_try_string"]
pub static PLACEHOLDER_02924: u8 = 0;

#[export_name = "g_queue_clear"]
pub static PLACEHOLDER_02927: u8 = 0;

#[export_name = "g_queue_clear_full"]
pub static PLACEHOLDER_02930: u8 = 0;

#[export_name = "g_queue_copy"]
pub static PLACEHOLDER_02933: u8 = 0;

#[export_name = "g_queue_delete_link"]
pub static PLACEHOLDER_02936: u8 = 0;

#[export_name = "g_queue_find"]
pub static PLACEHOLDER_02939: u8 = 0;

#[export_name = "g_queue_find_custom"]
pub static PLACEHOLDER_02942: u8 = 0;

#[export_name = "g_queue_foreach"]
pub static PLACEHOLDER_02945: u8 = 0;

#[export_name = "g_queue_free"]
pub static PLACEHOLDER_02948: u8 = 0;

#[export_name = "g_queue_free_full"]
pub static PLACEHOLDER_02951: u8 = 0;

#[export_name = "g_queue_get_length"]
pub static PLACEHOLDER_02954: u8 = 0;

#[export_name = "g_queue_index"]
pub static PLACEHOLDER_02957: u8 = 0;

#[export_name = "g_queue_init"]
pub static PLACEHOLDER_02960: u8 = 0;

#[export_name = "g_queue_insert_after"]
pub static PLACEHOLDER_02963: u8 = 0;

#[export_name = "g_queue_insert_after_link"]
pub static PLACEHOLDER_02966: u8 = 0;

#[export_name = "g_queue_insert_before"]
pub static PLACEHOLDER_02969: u8 = 0;

#[export_name = "g_queue_insert_before_link"]
pub static PLACEHOLDER_02972: u8 = 0;

#[export_name = "g_queue_insert_sorted"]
pub static PLACEHOLDER_02975: u8 = 0;

#[export_name = "g_queue_is_empty"]
pub static PLACEHOLDER_02978: u8 = 0;

#[export_name = "g_queue_link_index"]
pub static PLACEHOLDER_02981: u8 = 0;

#[export_name = "g_queue_new"]
pub static PLACEHOLDER_02984: u8 = 0;

#[export_name = "g_queue_peek_head"]
pub static PLACEHOLDER_02987: u8 = 0;

#[export_name = "g_queue_peek_head_link"]
pub static PLACEHOLDER_02990: u8 = 0;

#[export_name = "g_queue_peek_nth"]
pub static PLACEHOLDER_02993: u8 = 0;

#[export_name = "g_queue_peek_nth_link"]
pub static PLACEHOLDER_02996: u8 = 0;

#[export_name = "g_queue_peek_tail"]
pub static PLACEHOLDER_02999: u8 = 0;

#[export_name = "g_queue_peek_tail_link"]
pub static PLACEHOLDER_03002: u8 = 0;

#[export_name = "g_queue_pop_head"]
pub static PLACEHOLDER_03005: u8 = 0;

#[export_name = "g_queue_pop_head_link"]
pub static PLACEHOLDER_03008: u8 = 0;

#[export_name = "g_queue_pop_nth"]
pub static PLACEHOLDER_03011: u8 = 0;

#[export_name = "g_queue_pop_nth_link"]
pub static PLACEHOLDER_03014: u8 = 0;

#[export_name = "g_queue_pop_tail"]
pub static PLACEHOLDER_03017: u8 = 0;

#[export_name = "g_queue_pop_tail_link"]
pub static PLACEHOLDER_03020: u8 = 0;

#[export_name = "g_queue_push_head"]
pub static PLACEHOLDER_03023: u8 = 0;

#[export_name = "g_queue_push_head_link"]
pub static PLACEHOLDER_03026: u8 = 0;

#[export_name = "g_queue_push_nth"]
pub static PLACEHOLDER_03029: u8 = 0;

#[export_name = "g_queue_push_nth_link"]
pub static PLACEHOLDER_03032: u8 = 0;

#[export_name = "g_queue_push_tail"]
pub static PLACEHOLDER_03035: u8 = 0;

#[export_name = "g_queue_push_tail_link"]
pub static PLACEHOLDER_03038: u8 = 0;

#[export_name = "g_queue_remove"]
pub static PLACEHOLDER_03041: u8 = 0;

#[export_name = "g_queue_remove_all"]
pub static PLACEHOLDER_03044: u8 = 0;

#[export_name = "g_queue_reverse"]
pub static PLACEHOLDER_03047: u8 = 0;

#[export_name = "g_queue_sort"]
pub static PLACEHOLDER_03050: u8 = 0;

#[export_name = "g_queue_unlink"]
pub static PLACEHOLDER_03053: u8 = 0;

#[export_name = "g_rand_copy"]
pub static PLACEHOLDER_03056: u8 = 0;

#[export_name = "g_rand_double"]
pub static PLACEHOLDER_03059: u8 = 0;

#[export_name = "g_rand_double_range"]
pub static PLACEHOLDER_03062: u8 = 0;

#[export_name = "g_rand_free"]
pub static PLACEHOLDER_03065: u8 = 0;

#[export_name = "g_rand_int"]
pub static PLACEHOLDER_03068: u8 = 0;

#[export_name = "g_rand_int_range"]
pub static PLACEHOLDER_03071: u8 = 0;

#[export_name = "g_rand_new"]
pub static PLACEHOLDER_03074: u8 = 0;

#[export_name = "g_rand_new_with_seed"]
pub static PLACEHOLDER_03077: u8 = 0;

#[export_name = "g_rand_new_with_seed_array"]
pub static PLACEHOLDER_03080: u8 = 0;

#[export_name = "g_rand_set_seed"]
pub static PLACEHOLDER_03083: u8 = 0;

#[export_name = "g_rand_set_seed_array"]
pub static PLACEHOLDER_03086: u8 = 0;

#[export_name = "g_random_double"]
pub static PLACEHOLDER_03089: u8 = 0;

#[export_name = "g_random_double_range"]
pub static PLACEHOLDER_03092: u8 = 0;

#[export_name = "g_random_int"]
pub static PLACEHOLDER_03095: u8 = 0;

#[export_name = "g_random_int_range"]
pub static PLACEHOLDER_03098: u8 = 0;

#[export_name = "g_random_set_seed"]
pub static PLACEHOLDER_03101: u8 = 0;

#[export_name = "g_rc_box_acquire"]
pub static PLACEHOLDER_03104: u8 = 0;

#[export_name = "g_rc_box_alloc0"]
pub static PLACEHOLDER_03107: u8 = 0;

#[export_name = "g_rc_box_alloc"]
pub static PLACEHOLDER_03110: u8 = 0;

#[export_name = "g_rc_box_dup"]
pub static PLACEHOLDER_03113: u8 = 0;

#[export_name = "g_rc_box_get_size"]
pub static PLACEHOLDER_03116: u8 = 0;

#[export_name = "g_rc_box_release"]
pub static PLACEHOLDER_03119: u8 = 0;

#[export_name = "g_rc_box_release_full"]
pub static PLACEHOLDER_03122: u8 = 0;

#[export_name = "g_realloc"]
pub static PLACEHOLDER_03125: u8 = 0;

#[export_name = "g_realloc_n"]
pub static PLACEHOLDER_03128: u8 = 0;

#[export_name = "g_rec_mutex_clear"]
pub static PLACEHOLDER_03131: u8 = 0;

#[export_name = "g_rec_mutex_init"]
pub static PLACEHOLDER_03134: u8 = 0;

#[export_name = "g_rec_mutex_lock"]
pub static PLACEHOLDER_03137: u8 = 0;

#[export_name = "g_rec_mutex_trylock"]
pub static PLACEHOLDER_03140: u8 = 0;

#[export_name = "g_rec_mutex_unlock"]
pub static PLACEHOLDER_03143: u8 = 0;

#[export_name = "g_ref_count_compare"]
pub static PLACEHOLDER_03146: u8 = 0;

#[export_name = "g_ref_count_dec"]
pub static PLACEHOLDER_03149: u8 = 0;

#[export_name = "g_ref_count_inc"]
pub static PLACEHOLDER_03152: u8 = 0;

#[export_name = "g_ref_count_init"]
pub static PLACEHOLDER_03155: u8 = 0;

#[export_name = "g_ref_string_acquire"]
pub static PLACEHOLDER_03158: u8 = 0;

#[export_name = "g_ref_string_length"]
pub static PLACEHOLDER_03161: u8 = 0;

#[export_name = "g_ref_string_new"]
pub static PLACEHOLDER_03164: u8 = 0;

#[export_name = "g_ref_string_new_intern"]
pub static PLACEHOLDER_03167: u8 = 0;

#[export_name = "g_ref_string_new_len"]
pub static PLACEHOLDER_03170: u8 = 0;

#[export_name = "g_ref_string_release"]
pub static PLACEHOLDER_03173: u8 = 0;

#[export_name = "g_regex_check_replacement"]
pub static PLACEHOLDER_03176: u8 = 0;

#[export_name = "g_regex_error_quark"]
pub static PLACEHOLDER_03179: u8 = 0;

#[export_name = "g_regex_escape_nul"]
pub static PLACEHOLDER_03182: u8 = 0;

#[export_name = "g_regex_escape_string"]
pub static PLACEHOLDER_03185: u8 = 0;

#[export_name = "g_regex_get_capture_count"]
pub static PLACEHOLDER_03188: u8 = 0;

#[export_name = "g_regex_get_compile_flags"]
pub static PLACEHOLDER_03191: u8 = 0;

#[export_name = "g_regex_get_has_cr_or_lf"]
pub static PLACEHOLDER_03194: u8 = 0;

#[export_name = "g_regex_get_match_flags"]
pub static PLACEHOLDER_03197: u8 = 0;

#[export_name = "g_regex_get_max_backref"]
pub static PLACEHOLDER_03200: u8 = 0;

#[export_name = "g_regex_get_max_lookbehind"]
pub static PLACEHOLDER_03203: u8 = 0;

#[export_name = "g_regex_get_pattern"]
pub static PLACEHOLDER_03206: u8 = 0;

#[export_name = "g_regex_get_string_number"]
pub static PLACEHOLDER_03209: u8 = 0;

#[export_name = "g_regex_match"]
pub static PLACEHOLDER_03212: u8 = 0;

#[export_name = "g_regex_match_all"]
pub static PLACEHOLDER_03215: u8 = 0;

#[export_name = "g_regex_match_all_full"]
pub static PLACEHOLDER_03218: u8 = 0;

#[export_name = "g_regex_match_full"]
pub static PLACEHOLDER_03221: u8 = 0;

#[export_name = "g_regex_match_simple"]
pub static PLACEHOLDER_03224: u8 = 0;

#[export_name = "g_regex_new"]
pub static PLACEHOLDER_03227: u8 = 0;

#[export_name = "g_regex_ref"]
pub static PLACEHOLDER_03230: u8 = 0;

#[export_name = "g_regex_replace"]
pub static PLACEHOLDER_03233: u8 = 0;

#[export_name = "g_regex_replace_eval"]
pub static PLACEHOLDER_03236: u8 = 0;

#[export_name = "g_regex_replace_literal"]
pub static PLACEHOLDER_03239: u8 = 0;

#[export_name = "g_regex_split"]
pub static PLACEHOLDER_03242: u8 = 0;

#[export_name = "g_regex_split_full"]
pub static PLACEHOLDER_03245: u8 = 0;

#[export_name = "g_regex_split_simple"]
pub static PLACEHOLDER_03248: u8 = 0;

#[export_name = "g_regex_unref"]
pub static PLACEHOLDER_03251: u8 = 0;

#[export_name = "g_relation_count"]
pub static PLACEHOLDER_03254: u8 = 0;

#[export_name = "g_relation_delete"]
pub static PLACEHOLDER_03257: u8 = 0;

#[export_name = "g_relation_destroy"]
pub static PLACEHOLDER_03260: u8 = 0;

#[export_name = "g_relation_exists"]
pub static PLACEHOLDER_03263: u8 = 0;

#[export_name = "g_relation_index"]
pub static PLACEHOLDER_03266: u8 = 0;

#[export_name = "g_relation_insert"]
pub static PLACEHOLDER_03269: u8 = 0;

#[export_name = "g_relation_new"]
pub static PLACEHOLDER_03272: u8 = 0;

#[export_name = "g_relation_print"]
pub static PLACEHOLDER_03275: u8 = 0;

#[export_name = "g_relation_select"]
pub static PLACEHOLDER_03278: u8 = 0;

#[export_name = "g_reload_user_special_dirs_cache"]
pub static PLACEHOLDER_03281: u8 = 0;

#[export_name = "g_remove"]
pub static PLACEHOLDER_03284: u8 = 0;

#[export_name = "g_rename"]
pub static PLACEHOLDER_03287: u8 = 0;

#[export_name = "g_return_if_fail_warning"]
pub static PLACEHOLDER_03290: u8 = 0;

#[export_name = "g_rmdir"]
pub static PLACEHOLDER_03293: u8 = 0;

#[export_name = "g_rw_lock_clear"]
pub static PLACEHOLDER_03296: u8 = 0;

#[export_name = "g_rw_lock_init"]
pub static PLACEHOLDER_03299: u8 = 0;

#[export_name = "g_rw_lock_reader_lock"]
pub static PLACEHOLDER_03302: u8 = 0;

#[export_name = "g_rw_lock_reader_trylock"]
pub static PLACEHOLDER_03305: u8 = 0;

#[export_name = "g_rw_lock_reader_unlock"]
pub static PLACEHOLDER_03308: u8 = 0;

#[export_name = "g_rw_lock_writer_lock"]
pub static PLACEHOLDER_03311: u8 = 0;

#[export_name = "g_rw_lock_writer_trylock"]
pub static PLACEHOLDER_03314: u8 = 0;

#[export_name = "g_rw_lock_writer_unlock"]
pub static PLACEHOLDER_03317: u8 = 0;

#[export_name = "g_scanner_cur_line"]
pub static PLACEHOLDER_03320: u8 = 0;

#[export_name = "g_scanner_cur_position"]
pub static PLACEHOLDER_03323: u8 = 0;

#[export_name = "g_scanner_cur_token"]
pub static PLACEHOLDER_03326: u8 = 0;

#[export_name = "g_scanner_cur_value"]
pub static PLACEHOLDER_03329: u8 = 0;

#[export_name = "g_scanner_destroy"]
pub static PLACEHOLDER_03332: u8 = 0;

#[export_name = "g_scanner_eof"]
pub static PLACEHOLDER_03335: u8 = 0;

#[export_name = "g_scanner_error"]
pub static PLACEHOLDER_03338: u8 = 0;

#[export_name = "g_scanner_get_next_token"]
pub static PLACEHOLDER_03341: u8 = 0;

#[export_name = "g_scanner_input_file"]
pub static PLACEHOLDER_03344: u8 = 0;

#[export_name = "g_scanner_input_text"]
pub static PLACEHOLDER_03347: u8 = 0;

#[export_name = "g_scanner_lookup_symbol"]
pub static PLACEHOLDER_03350: u8 = 0;

#[export_name = "g_scanner_new"]
pub static PLACEHOLDER_03353: u8 = 0;

#[export_name = "g_scanner_peek_next_token"]
pub static PLACEHOLDER_03356: u8 = 0;

#[export_name = "g_scanner_scope_add_symbol"]
pub static PLACEHOLDER_03359: u8 = 0;

#[export_name = "g_scanner_scope_foreach_symbol"]
pub static PLACEHOLDER_03362: u8 = 0;

#[export_name = "g_scanner_scope_lookup_symbol"]
pub static PLACEHOLDER_03365: u8 = 0;

#[export_name = "g_scanner_scope_remove_symbol"]
pub static PLACEHOLDER_03368: u8 = 0;

#[export_name = "g_scanner_set_scope"]
pub static PLACEHOLDER_03371: u8 = 0;

#[export_name = "g_scanner_sync_file_offset"]
pub static PLACEHOLDER_03374: u8 = 0;

#[export_name = "g_scanner_unexp_token"]
pub static PLACEHOLDER_03377: u8 = 0;

#[export_name = "g_scanner_warn"]
pub static PLACEHOLDER_03380: u8 = 0;

#[export_name = "g_sequence_append"]
pub static PLACEHOLDER_03383: u8 = 0;

#[export_name = "g_sequence_foreach"]
pub static PLACEHOLDER_03386: u8 = 0;

#[export_name = "g_sequence_foreach_range"]
pub static PLACEHOLDER_03389: u8 = 0;

#[export_name = "g_sequence_free"]
pub static PLACEHOLDER_03392: u8 = 0;

#[export_name = "g_sequence_get"]
pub static PLACEHOLDER_03395: u8 = 0;

#[export_name = "g_sequence_get_begin_iter"]
pub static PLACEHOLDER_03398: u8 = 0;

#[export_name = "g_sequence_get_end_iter"]
pub static PLACEHOLDER_03401: u8 = 0;

#[export_name = "g_sequence_get_iter_at_pos"]
pub static PLACEHOLDER_03404: u8 = 0;

#[export_name = "g_sequence_get_length"]
pub static PLACEHOLDER_03407: u8 = 0;

#[export_name = "g_sequence_insert_before"]
pub static PLACEHOLDER_03410: u8 = 0;

#[export_name = "g_sequence_insert_sorted"]
pub static PLACEHOLDER_03413: u8 = 0;

#[export_name = "g_sequence_insert_sorted_iter"]
pub static PLACEHOLDER_03416: u8 = 0;

#[export_name = "g_sequence_is_empty"]
pub static PLACEHOLDER_03419: u8 = 0;

#[export_name = "g_sequence_iter_compare"]
pub static PLACEHOLDER_03422: u8 = 0;

#[export_name = "g_sequence_iter_get_position"]
pub static PLACEHOLDER_03425: u8 = 0;

#[export_name = "g_sequence_iter_get_sequence"]
pub static PLACEHOLDER_03428: u8 = 0;

#[export_name = "g_sequence_iter_is_begin"]
pub static PLACEHOLDER_03431: u8 = 0;

#[export_name = "g_sequence_iter_is_end"]
pub static PLACEHOLDER_03434: u8 = 0;

#[export_name = "g_sequence_iter_move"]
pub static PLACEHOLDER_03437: u8 = 0;

#[export_name = "g_sequence_iter_next"]
pub static PLACEHOLDER_03440: u8 = 0;

#[export_name = "g_sequence_iter_prev"]
pub static PLACEHOLDER_03443: u8 = 0;

#[export_name = "g_sequence_lookup"]
pub static PLACEHOLDER_03446: u8 = 0;

#[export_name = "g_sequence_lookup_iter"]
pub static PLACEHOLDER_03449: u8 = 0;

#[export_name = "g_sequence_move"]
pub static PLACEHOLDER_03452: u8 = 0;

#[export_name = "g_sequence_move_range"]
pub static PLACEHOLDER_03455: u8 = 0;

#[export_name = "g_sequence_new"]
pub static PLACEHOLDER_03458: u8 = 0;

#[export_name = "g_sequence_prepend"]
pub static PLACEHOLDER_03461: u8 = 0;

#[export_name = "g_sequence_range_get_midpoint"]
pub static PLACEHOLDER_03464: u8 = 0;

#[export_name = "g_sequence_remove"]
pub static PLACEHOLDER_03467: u8 = 0;

#[export_name = "g_sequence_remove_range"]
pub static PLACEHOLDER_03470: u8 = 0;

#[export_name = "g_sequence_search"]
pub static PLACEHOLDER_03473: u8 = 0;

#[export_name = "g_sequence_search_iter"]
pub static PLACEHOLDER_03476: u8 = 0;

#[export_name = "g_sequence_set"]
pub static PLACEHOLDER_03479: u8 = 0;

#[export_name = "g_sequence_sort"]
pub static PLACEHOLDER_03482: u8 = 0;

#[export_name = "g_sequence_sort_changed"]
pub static PLACEHOLDER_03485: u8 = 0;

#[export_name = "g_sequence_sort_changed_iter"]
pub static PLACEHOLDER_03488: u8 = 0;

#[export_name = "g_sequence_sort_iter"]
pub static PLACEHOLDER_03491: u8 = 0;

#[export_name = "g_sequence_swap"]
pub static PLACEHOLDER_03494: u8 = 0;

#[export_name = "g_set_application_name"]
pub static PLACEHOLDER_03497: u8 = 0;

#[export_name = "g_set_error"]
pub static PLACEHOLDER_03500: u8 = 0;

#[export_name = "g_set_error_literal"]
pub static PLACEHOLDER_03503: u8 = 0;

#[export_name = "g_set_prgname"]
pub static PLACEHOLDER_03506: u8 = 0;

#[export_name = "g_set_print_handler"]
pub static PLACEHOLDER_03509: u8 = 0;

#[export_name = "g_set_printerr_handler"]
pub static PLACEHOLDER_03512: u8 = 0;

#[export_name = "g_setenv"]
pub static PLACEHOLDER_03515: u8 = 0;

#[export_name = "g_shell_error_quark"]
pub static PLACEHOLDER_03518: u8 = 0;

#[export_name = "g_shell_parse_argv"]
pub static PLACEHOLDER_03521: u8 = 0;

#[export_name = "g_shell_quote"]
pub static PLACEHOLDER_03524: u8 = 0;

#[export_name = "g_shell_unquote"]
pub static PLACEHOLDER_03527: u8 = 0;

#[export_name = "g_slice_alloc0"]
pub static PLACEHOLDER_03530: u8 = 0;

#[export_name = "g_slice_alloc"]
pub static PLACEHOLDER_03533: u8 = 0;

#[export_name = "g_slice_copy"]
pub static PLACEHOLDER_03536: u8 = 0;

#[export_name = "g_slice_free1"]
pub static PLACEHOLDER_03539: u8 = 0;

#[export_name = "g_slice_free_chain_with_offset"]
pub static PLACEHOLDER_03542: u8 = 0;

#[export_name = "g_slice_get_config"]
pub static PLACEHOLDER_03545: u8 = 0;

#[export_name = "g_slice_get_config_state"]
pub static PLACEHOLDER_03548: u8 = 0;

#[export_name = "g_slice_set_config"]
pub static PLACEHOLDER_03551: u8 = 0;

#[export_name = "g_slist_alloc"]
pub static PLACEHOLDER_03554: u8 = 0;

#[export_name = "g_slist_append"]
pub static PLACEHOLDER_03557: u8 = 0;

#[export_name = "g_slist_concat"]
pub static PLACEHOLDER_03560: u8 = 0;

#[export_name = "g_slist_copy"]
pub static PLACEHOLDER_03563: u8 = 0;

#[export_name = "g_slist_copy_deep"]
pub static PLACEHOLDER_03566: u8 = 0;

#[export_name = "g_slist_delete_link"]
pub static PLACEHOLDER_03569: u8 = 0;

#[export_name = "g_slist_find"]
pub static PLACEHOLDER_03572: u8 = 0;

#[export_name = "g_slist_find_custom"]
pub static PLACEHOLDER_03575: u8 = 0;

#[export_name = "g_slist_foreach"]
pub static PLACEHOLDER_03578: u8 = 0;

#[export_name = "g_slist_free"]
pub static PLACEHOLDER_03581: u8 = 0;

#[export_name = "g_slist_free_1"]
pub static PLACEHOLDER_03584: u8 = 0;

#[export_name = "g_slist_free_full"]
pub static PLACEHOLDER_03587: u8 = 0;

#[export_name = "g_slist_index"]
pub static PLACEHOLDER_03590: u8 = 0;

#[export_name = "g_slist_insert"]
pub static PLACEHOLDER_03593: u8 = 0;

#[export_name = "g_slist_insert_before"]
pub static PLACEHOLDER_03596: u8 = 0;

#[export_name = "g_slist_insert_sorted"]
pub static PLACEHOLDER_03599: u8 = 0;

#[export_name = "g_slist_insert_sorted_with_data"]
pub static PLACEHOLDER_03602: u8 = 0;

#[export_name = "g_slist_last"]
pub static PLACEHOLDER_03605: u8 = 0;

#[export_name = "g_slist_length"]
pub static PLACEHOLDER_03608: u8 = 0;

#[export_name = "g_slist_nth"]
pub static PLACEHOLDER_03611: u8 = 0;

#[export_name = "g_slist_nth_data"]
pub static PLACEHOLDER_03614: u8 = 0;

#[export_name = "g_slist_pop_allocator"]
pub static PLACEHOLDER_03617: u8 = 0;

#[export_name = "g_slist_position"]
pub static PLACEHOLDER_03620: u8 = 0;

#[export_name = "g_slist_prepend"]
pub static PLACEHOLDER_03623: u8 = 0;

#[export_name = "g_slist_push_allocator"]
pub static PLACEHOLDER_03626: u8 = 0;

#[export_name = "g_slist_remove"]
pub static PLACEHOLDER_03629: u8 = 0;

#[export_name = "g_slist_remove_all"]
pub static PLACEHOLDER_03632: u8 = 0;

#[export_name = "g_slist_remove_link"]
pub static PLACEHOLDER_03635: u8 = 0;

#[export_name = "g_slist_reverse"]
pub static PLACEHOLDER_03638: u8 = 0;

#[export_name = "g_slist_sort"]
pub static PLACEHOLDER_03641: u8 = 0;

#[export_name = "g_slist_sort_with_data"]
pub static PLACEHOLDER_03644: u8 = 0;

#[export_name = "g_snprintf"]
pub static PLACEHOLDER_03647: u8 = 0;

#[export_name = "g_source_add_child_source"]
pub static PLACEHOLDER_03650: u8 = 0;

#[export_name = "g_source_add_poll"]
pub static PLACEHOLDER_03653: u8 = 0;

#[export_name = "g_source_add_unix_fd"]
pub static PLACEHOLDER_03656: u8 = 0;

#[export_name = "g_source_attach"]
pub static PLACEHOLDER_03659: u8 = 0;

#[export_name = "g_source_destroy"]
pub static PLACEHOLDER_03662: u8 = 0;

#[export_name = "g_source_get_can_recurse"]
pub static PLACEHOLDER_03665: u8 = 0;

#[export_name = "g_source_get_context"]
pub static PLACEHOLDER_03668: u8 = 0;

#[export_name = "g_source_get_current_time"]
pub static PLACEHOLDER_03671: u8 = 0;

#[export_name = "g_source_get_id"]
pub static PLACEHOLDER_03674: u8 = 0;

#[export_name = "g_source_get_name"]
pub static PLACEHOLDER_03677: u8 = 0;

#[export_name = "g_source_get_priority"]
pub static PLACEHOLDER_03680: u8 = 0;

#[export_name = "g_source_get_ready_time"]
pub static PLACEHOLDER_03683: u8 = 0;

#[export_name = "g_source_get_time"]
pub static PLACEHOLDER_03686: u8 = 0;

#[export_name = "g_source_is_destroyed"]
pub static PLACEHOLDER_03689: u8 = 0;

#[export_name = "g_source_modify_unix_fd"]
pub static PLACEHOLDER_03692: u8 = 0;

#[export_name = "g_source_new"]
pub static PLACEHOLDER_03695: u8 = 0;

#[export_name = "g_source_query_unix_fd"]
pub static PLACEHOLDER_03698: u8 = 0;

#[export_name = "g_source_ref"]
pub static PLACEHOLDER_03701: u8 = 0;

#[export_name = "g_source_remove"]
pub static PLACEHOLDER_03704: u8 = 0;

#[export_name = "g_source_remove_by_funcs_user_data"]
pub static PLACEHOLDER_03707: u8 = 0;

#[export_name = "g_source_remove_by_user_data"]
pub static PLACEHOLDER_03710: u8 = 0;

#[export_name = "g_source_remove_child_source"]
pub static PLACEHOLDER_03713: u8 = 0;

#[export_name = "g_source_remove_poll"]
pub static PLACEHOLDER_03716: u8 = 0;

#[export_name = "g_source_remove_unix_fd"]
pub static PLACEHOLDER_03719: u8 = 0;

#[export_name = "g_source_set_callback"]
pub static PLACEHOLDER_03722: u8 = 0;

#[export_name = "g_source_set_callback_indirect"]
pub static PLACEHOLDER_03725: u8 = 0;

#[export_name = "g_source_set_can_recurse"]
pub static PLACEHOLDER_03728: u8 = 0;

#[export_name = "g_source_set_dispose_function"]
pub static PLACEHOLDER_03731: u8 = 0;

#[export_name = "g_source_set_funcs"]
pub static PLACEHOLDER_03734: u8 = 0;

#[export_name = "g_source_set_name"]
pub static PLACEHOLDER_03737: u8 = 0;

#[export_name = "g_source_set_name_by_id"]
pub static PLACEHOLDER_03740: u8 = 0;

#[export_name = "g_source_set_priority"]
pub static PLACEHOLDER_03743: u8 = 0;

#[export_name = "g_source_set_ready_time"]
pub static PLACEHOLDER_03746: u8 = 0;

#[export_name = "g_source_set_static_name"]
pub static PLACEHOLDER_03749: u8 = 0;

#[export_name = "g_source_unref"]
pub static PLACEHOLDER_03752: u8 = 0;

#[export_name = "g_spaced_primes_closest"]
pub static PLACEHOLDER_03755: u8 = 0;

#[export_name = "g_spawn_async"]
pub static PLACEHOLDER_03758: u8 = 0;

#[export_name = "g_spawn_async_with_fds"]
pub static PLACEHOLDER_03761: u8 = 0;

#[export_name = "g_spawn_async_with_pipes"]
pub static PLACEHOLDER_03764: u8 = 0;

#[export_name = "g_spawn_async_with_pipes_and_fds"]
pub static PLACEHOLDER_03767: u8 = 0;

#[export_name = "g_spawn_check_exit_status"]
pub static PLACEHOLDER_03770: u8 = 0;

#[export_name = "g_spawn_check_wait_status"]
pub static PLACEHOLDER_03773: u8 = 0;

#[export_name = "g_spawn_close_pid"]
pub static PLACEHOLDER_03776: u8 = 0;

#[export_name = "g_spawn_command_line_async"]
pub static PLACEHOLDER_03779: u8 = 0;

#[export_name = "g_spawn_command_line_sync"]
pub static PLACEHOLDER_03782: u8 = 0;

#[export_name = "g_spawn_error_quark"]
pub static PLACEHOLDER_03785: u8 = 0;

#[export_name = "g_spawn_exit_error_quark"]
pub static PLACEHOLDER_03788: u8 = 0;

#[export_name = "g_spawn_sync"]
pub static PLACEHOLDER_03791: u8 = 0;

#[export_name = "g_sprintf"]
pub static PLACEHOLDER_03794: u8 = 0;

#[export_name = "g_stat"]
pub static PLACEHOLDER_03797: u8 = 0;

#[export_name = "g_static_mutex_free"]
pub static PLACEHOLDER_03800: u8 = 0;

#[export_name = "g_static_mutex_get_mutex_impl"]
pub static PLACEHOLDER_03803: u8 = 0;

#[export_name = "g_static_mutex_init"]
pub static PLACEHOLDER_03806: u8 = 0;

#[export_name = "g_static_private_free"]
pub static PLACEHOLDER_03809: u8 = 0;

#[export_name = "g_static_private_get"]
pub static PLACEHOLDER_03812: u8 = 0;

#[export_name = "g_static_private_init"]
pub static PLACEHOLDER_03815: u8 = 0;

#[export_name = "g_static_private_set"]
pub static PLACEHOLDER_03818: u8 = 0;

#[export_name = "g_static_rec_mutex_free"]
pub static PLACEHOLDER_03821: u8 = 0;

#[export_name = "g_static_rec_mutex_init"]
pub static PLACEHOLDER_03824: u8 = 0;

#[export_name = "g_static_rec_mutex_lock"]
pub static PLACEHOLDER_03827: u8 = 0;

#[export_name = "g_static_rec_mutex_lock_full"]
pub static PLACEHOLDER_03830: u8 = 0;

#[export_name = "g_static_rec_mutex_trylock"]
pub static PLACEHOLDER_03833: u8 = 0;

#[export_name = "g_static_rec_mutex_unlock"]
pub static PLACEHOLDER_03836: u8 = 0;

#[export_name = "g_static_rec_mutex_unlock_full"]
pub static PLACEHOLDER_03839: u8 = 0;

#[export_name = "g_static_rw_lock_free"]
pub static PLACEHOLDER_03842: u8 = 0;

#[export_name = "g_static_rw_lock_init"]
pub static PLACEHOLDER_03845: u8 = 0;

#[export_name = "g_static_rw_lock_reader_lock"]
pub static PLACEHOLDER_03848: u8 = 0;

#[export_name = "g_static_rw_lock_reader_trylock"]
pub static PLACEHOLDER_03851: u8 = 0;

#[export_name = "g_static_rw_lock_reader_unlock"]
pub static PLACEHOLDER_03854: u8 = 0;

#[export_name = "g_static_rw_lock_writer_lock"]
pub static PLACEHOLDER_03857: u8 = 0;

#[export_name = "g_static_rw_lock_writer_trylock"]
pub static PLACEHOLDER_03860: u8 = 0;

#[export_name = "g_static_rw_lock_writer_unlock"]
pub static PLACEHOLDER_03863: u8 = 0;

#[export_name = "g_stpcpy"]
pub static PLACEHOLDER_03866: u8 = 0;

#[export_name = "g_str_equal"]
pub static PLACEHOLDER_03869: u8 = 0;

#[export_name = "g_str_has_prefix"]
pub static PLACEHOLDER_03872: u8 = 0;

#[export_name = "g_str_has_suffix"]
pub static PLACEHOLDER_03875: u8 = 0;

#[export_name = "g_str_hash"]
pub static PLACEHOLDER_03878: u8 = 0;

#[export_name = "g_str_is_ascii"]
pub static PLACEHOLDER_03881: u8 = 0;

#[export_name = "g_str_match_string"]
pub static PLACEHOLDER_03884: u8 = 0;

#[export_name = "g_str_to_ascii"]
pub static PLACEHOLDER_03887: u8 = 0;

#[export_name = "g_str_tokenize_and_fold"]
pub static PLACEHOLDER_03890: u8 = 0;

#[export_name = "g_strcanon"]
pub static PLACEHOLDER_03893: u8 = 0;

#[export_name = "g_strcasecmp"]
pub static PLACEHOLDER_03896: u8 = 0;

#[export_name = "g_strchomp"]
pub static PLACEHOLDER_03899: u8 = 0;

#[export_name = "g_strchug"]
pub static PLACEHOLDER_03902: u8 = 0;

#[export_name = "g_strcmp0"]
pub unsafe extern "C" fn export_g_strcmp0(
    str1: *const crate::ffi::gchar,
    str2: *const crate::ffi::gchar,
) -> crate::ffi::gint {
    crate::runtime::g_strcmp0_impl(str1, str2)
}

#[export_name = "g_strcompress"]
pub static PLACEHOLDER_03907: u8 = 0;

#[export_name = "g_strconcat"]
pub static PLACEHOLDER_03910: u8 = 0;

#[export_name = "g_strdelimit"]
pub static PLACEHOLDER_03913: u8 = 0;

#[export_name = "g_strdown"]
pub static PLACEHOLDER_03916: u8 = 0;

#[export_name = "g_strdup"]
pub static PLACEHOLDER_03919: u8 = 0;

#[export_name = "g_strdup_printf"]
pub static PLACEHOLDER_03922: u8 = 0;

#[export_name = "g_strdup_vprintf"]
pub static PLACEHOLDER_03925: u8 = 0;

#[export_name = "g_strdupv"]
pub static PLACEHOLDER_03928: u8 = 0;

#[export_name = "g_strerror"]
pub static PLACEHOLDER_03931: u8 = 0;

#[export_name = "g_strescape"]
pub static PLACEHOLDER_03934: u8 = 0;

#[export_name = "g_strfreev"]
pub static PLACEHOLDER_03937: u8 = 0;

#[export_name = "g_string_append"]
pub static PLACEHOLDER_03940: u8 = 0;

#[export_name = "g_string_append_c"]
pub static PLACEHOLDER_03943: u8 = 0;

#[export_name = "g_string_append_len"]
pub static PLACEHOLDER_03946: u8 = 0;

#[export_name = "g_string_append_printf"]
pub static PLACEHOLDER_03949: u8 = 0;

#[export_name = "g_string_append_unichar"]
pub static PLACEHOLDER_03952: u8 = 0;

#[export_name = "g_string_append_uri_escaped"]
pub static PLACEHOLDER_03955: u8 = 0;

#[export_name = "g_string_append_vprintf"]
pub static PLACEHOLDER_03958: u8 = 0;

#[export_name = "g_string_ascii_down"]
pub static PLACEHOLDER_03961: u8 = 0;

#[export_name = "g_string_ascii_up"]
pub static PLACEHOLDER_03964: u8 = 0;

#[export_name = "g_string_assign"]
pub static PLACEHOLDER_03967: u8 = 0;

#[export_name = "g_string_chunk_clear"]
pub static PLACEHOLDER_03970: u8 = 0;

#[export_name = "g_string_chunk_free"]
pub static PLACEHOLDER_03973: u8 = 0;

#[export_name = "g_string_chunk_insert"]
pub static PLACEHOLDER_03976: u8 = 0;

#[export_name = "g_string_chunk_insert_const"]
pub static PLACEHOLDER_03979: u8 = 0;

#[export_name = "g_string_chunk_insert_len"]
pub static PLACEHOLDER_03982: u8 = 0;

#[export_name = "g_string_chunk_new"]
pub static PLACEHOLDER_03985: u8 = 0;

#[export_name = "g_string_down"]
pub static PLACEHOLDER_03988: u8 = 0;

#[export_name = "g_string_equal"]
pub static PLACEHOLDER_03991: u8 = 0;

#[export_name = "g_string_erase"]
pub static PLACEHOLDER_03994: u8 = 0;

#[export_name = "g_string_free"]
pub static PLACEHOLDER_03997: u8 = 0;

#[export_name = "g_string_free_and_steal"]
pub static PLACEHOLDER_04000: u8 = 0;

#[export_name = "g_string_free_to_bytes"]
pub static PLACEHOLDER_04003: u8 = 0;

#[export_name = "g_string_hash"]
pub static PLACEHOLDER_04006: u8 = 0;

#[export_name = "g_string_insert"]
pub static PLACEHOLDER_04009: u8 = 0;

#[export_name = "g_string_insert_c"]
pub static PLACEHOLDER_04012: u8 = 0;

#[export_name = "g_string_insert_len"]
pub static PLACEHOLDER_04015: u8 = 0;

#[export_name = "g_string_insert_unichar"]
pub static PLACEHOLDER_04018: u8 = 0;

#[export_name = "g_string_new"]
pub static PLACEHOLDER_04021: u8 = 0;

#[export_name = "g_string_new_len"]
pub static PLACEHOLDER_04024: u8 = 0;

#[export_name = "g_string_new_take"]
pub static PLACEHOLDER_04027: u8 = 0;

#[export_name = "g_string_overwrite"]
pub static PLACEHOLDER_04030: u8 = 0;

#[export_name = "g_string_overwrite_len"]
pub static PLACEHOLDER_04033: u8 = 0;

#[export_name = "g_string_prepend"]
pub static PLACEHOLDER_04036: u8 = 0;

#[export_name = "g_string_prepend_c"]
pub static PLACEHOLDER_04039: u8 = 0;

#[export_name = "g_string_prepend_len"]
pub static PLACEHOLDER_04042: u8 = 0;

#[export_name = "g_string_prepend_unichar"]
pub static PLACEHOLDER_04045: u8 = 0;

#[export_name = "g_string_printf"]
pub static PLACEHOLDER_04048: u8 = 0;

#[export_name = "g_string_replace"]
pub static PLACEHOLDER_04051: u8 = 0;

#[export_name = "g_string_set_size"]
pub static PLACEHOLDER_04054: u8 = 0;

#[export_name = "g_string_sized_new"]
pub static PLACEHOLDER_04057: u8 = 0;

#[export_name = "g_string_truncate"]
pub static PLACEHOLDER_04060: u8 = 0;

#[export_name = "g_string_up"]
pub static PLACEHOLDER_04063: u8 = 0;

#[export_name = "g_string_vprintf"]
pub static PLACEHOLDER_04066: u8 = 0;

#[export_name = "g_strip_context"]
pub static PLACEHOLDER_04069: u8 = 0;

#[export_name = "g_strjoin"]
pub static PLACEHOLDER_04072: u8 = 0;

#[export_name = "g_strjoinv"]
pub static PLACEHOLDER_04075: u8 = 0;

#[export_name = "g_strlcat"]
pub static PLACEHOLDER_04078: u8 = 0;

#[export_name = "g_strlcpy"]
pub static PLACEHOLDER_04081: u8 = 0;

#[export_name = "g_strncasecmp"]
pub static PLACEHOLDER_04084: u8 = 0;

#[export_name = "g_strndup"]
pub static PLACEHOLDER_04087: u8 = 0;

#[export_name = "g_strnfill"]
pub static PLACEHOLDER_04090: u8 = 0;

#[export_name = "g_strreverse"]
pub static PLACEHOLDER_04093: u8 = 0;

#[export_name = "g_strrstr"]
pub static PLACEHOLDER_04096: u8 = 0;

#[export_name = "g_strrstr_len"]
pub static PLACEHOLDER_04099: u8 = 0;

#[export_name = "g_strsignal"]
pub static PLACEHOLDER_04102: u8 = 0;

#[export_name = "g_strsplit"]
pub static PLACEHOLDER_04105: u8 = 0;

#[export_name = "g_strsplit_set"]
pub static PLACEHOLDER_04108: u8 = 0;

#[export_name = "g_strstr_len"]
pub static PLACEHOLDER_04111: u8 = 0;

#[export_name = "g_strtod"]
pub static PLACEHOLDER_04114: u8 = 0;

#[export_name = "g_strup"]
pub static PLACEHOLDER_04117: u8 = 0;

#[export_name = "g_strv_builder_add"]
pub static PLACEHOLDER_04120: u8 = 0;

#[export_name = "g_strv_builder_add_many"]
pub static PLACEHOLDER_04123: u8 = 0;

#[export_name = "g_strv_builder_addv"]
pub static PLACEHOLDER_04126: u8 = 0;

#[export_name = "g_strv_builder_end"]
pub static PLACEHOLDER_04129: u8 = 0;

#[export_name = "g_strv_builder_new"]
pub static PLACEHOLDER_04132: u8 = 0;

#[export_name = "g_strv_builder_ref"]
pub static PLACEHOLDER_04135: u8 = 0;

#[export_name = "g_strv_builder_take"]
pub static PLACEHOLDER_04138: u8 = 0;

#[export_name = "g_strv_builder_unref"]
pub static PLACEHOLDER_04141: u8 = 0;

#[export_name = "g_strv_contains"]
pub static PLACEHOLDER_04144: u8 = 0;

#[export_name = "g_strv_equal"]
pub static PLACEHOLDER_04147: u8 = 0;

#[export_name = "g_strv_length"]
pub static PLACEHOLDER_04150: u8 = 0;

#[export_name = "g_test_add_data_func"]
pub static PLACEHOLDER_04153: u8 = 0;

#[export_name = "g_test_add_data_func_full"]
pub static PLACEHOLDER_04156: u8 = 0;

#[export_name = "g_test_add_func"]
pub static PLACEHOLDER_04159: u8 = 0;

#[export_name = "g_test_add_vtable"]
pub static PLACEHOLDER_04162: u8 = 0;

#[export_name = "g_test_assert_expected_messages_internal"]
pub static PLACEHOLDER_04165: u8 = 0;

#[export_name = "g_test_bug"]
pub static PLACEHOLDER_04168: u8 = 0;

#[export_name = "g_test_bug_base"]
pub static PLACEHOLDER_04171: u8 = 0;

#[export_name = "g_test_build_filename"]
pub static PLACEHOLDER_04174: u8 = 0;

#[export_name = "g_test_case_free"]
pub static PLACEHOLDER_04177: u8 = 0;

#[export_name = "g_test_config_vars"]
pub static PLACEHOLDER_04180: u8 = 0;

#[export_name = "g_test_create_case"]
pub static PLACEHOLDER_04183: u8 = 0;

#[export_name = "g_test_create_suite"]
pub static PLACEHOLDER_04186: u8 = 0;

#[export_name = "g_test_disable_crash_reporting"]
pub static PLACEHOLDER_04189: u8 = 0;

#[export_name = "g_test_expect_message"]
pub static PLACEHOLDER_04192: u8 = 0;

#[export_name = "g_test_fail"]
pub static PLACEHOLDER_04195: u8 = 0;

#[export_name = "g_test_fail_printf"]
pub static PLACEHOLDER_04198: u8 = 0;

#[export_name = "g_test_failed"]
pub static PLACEHOLDER_04201: u8 = 0;

#[export_name = "g_test_get_dir"]
pub static PLACEHOLDER_04204: u8 = 0;

#[export_name = "g_test_get_filename"]
pub static PLACEHOLDER_04207: u8 = 0;

#[export_name = "g_test_get_path"]
pub static PLACEHOLDER_04210: u8 = 0;

#[export_name = "g_test_get_root"]
pub static PLACEHOLDER_04213: u8 = 0;

#[export_name = "g_test_incomplete"]
pub static PLACEHOLDER_04216: u8 = 0;

#[export_name = "g_test_incomplete_printf"]
pub static PLACEHOLDER_04219: u8 = 0;

#[export_name = "g_test_init"]
pub static PLACEHOLDER_04222: u8 = 0;

#[export_name = "g_test_log_buffer_free"]
pub static PLACEHOLDER_04225: u8 = 0;

#[export_name = "g_test_log_buffer_new"]
pub static PLACEHOLDER_04228: u8 = 0;

#[export_name = "g_test_log_buffer_pop"]
pub static PLACEHOLDER_04231: u8 = 0;

#[export_name = "g_test_log_buffer_push"]
pub static PLACEHOLDER_04234: u8 = 0;

#[export_name = "g_test_log_msg_free"]
pub static PLACEHOLDER_04237: u8 = 0;

#[export_name = "g_test_log_set_fatal_handler"]
pub static PLACEHOLDER_04240: u8 = 0;

#[export_name = "g_test_log_type_name"]
pub static PLACEHOLDER_04243: u8 = 0;

#[export_name = "g_test_maximized_result"]
pub static PLACEHOLDER_04246: u8 = 0;

#[export_name = "g_test_message"]
pub static PLACEHOLDER_04249: u8 = 0;

#[export_name = "g_test_minimized_result"]
pub static PLACEHOLDER_04252: u8 = 0;

#[export_name = "g_test_queue_destroy"]
pub static PLACEHOLDER_04255: u8 = 0;

#[export_name = "g_test_queue_free"]
pub static PLACEHOLDER_04258: u8 = 0;

#[export_name = "g_test_rand_double"]
pub static PLACEHOLDER_04261: u8 = 0;

#[export_name = "g_test_rand_double_range"]
pub static PLACEHOLDER_04264: u8 = 0;

#[export_name = "g_test_rand_int"]
pub static PLACEHOLDER_04267: u8 = 0;

#[export_name = "g_test_rand_int_range"]
pub static PLACEHOLDER_04270: u8 = 0;

#[export_name = "g_test_run"]
pub static PLACEHOLDER_04273: u8 = 0;

#[export_name = "g_test_run_suite"]
pub static PLACEHOLDER_04276: u8 = 0;

#[export_name = "g_test_set_nonfatal_assertions"]
pub static PLACEHOLDER_04279: u8 = 0;

#[export_name = "g_test_skip"]
pub static PLACEHOLDER_04282: u8 = 0;

#[export_name = "g_test_skip_printf"]
pub static PLACEHOLDER_04285: u8 = 0;

#[export_name = "g_test_subprocess"]
pub static PLACEHOLDER_04288: u8 = 0;

#[export_name = "g_test_suite_add"]
pub static PLACEHOLDER_04291: u8 = 0;

#[export_name = "g_test_suite_add_suite"]
pub static PLACEHOLDER_04294: u8 = 0;

#[export_name = "g_test_suite_free"]
pub static PLACEHOLDER_04297: u8 = 0;

#[export_name = "g_test_summary"]
pub static PLACEHOLDER_04300: u8 = 0;

#[export_name = "g_test_timer_elapsed"]
pub static PLACEHOLDER_04303: u8 = 0;

#[export_name = "g_test_timer_last"]
pub static PLACEHOLDER_04306: u8 = 0;

#[export_name = "g_test_timer_start"]
pub static PLACEHOLDER_04309: u8 = 0;

#[export_name = "g_test_trap_assertions"]
pub static PLACEHOLDER_04312: u8 = 0;

#[export_name = "g_test_trap_fork"]
pub static PLACEHOLDER_04315: u8 = 0;

#[export_name = "g_test_trap_has_passed"]
pub static PLACEHOLDER_04318: u8 = 0;

#[export_name = "g_test_trap_reached_timeout"]
pub static PLACEHOLDER_04321: u8 = 0;

#[export_name = "g_test_trap_subprocess"]
pub static PLACEHOLDER_04324: u8 = 0;

#[export_name = "g_test_trap_subprocess_with_envp"]
pub static PLACEHOLDER_04327: u8 = 0;

#[export_name = "g_thread_create"]
pub static PLACEHOLDER_04330: u8 = 0;

#[export_name = "g_thread_create_full"]
pub static PLACEHOLDER_04333: u8 = 0;

#[export_name = "g_thread_error_quark"]
pub static PLACEHOLDER_04336: u8 = 0;

#[export_name = "g_thread_exit"]
pub static PLACEHOLDER_04339: u8 = 0;

#[export_name = "g_thread_foreach"]
pub static PLACEHOLDER_04342: u8 = 0;

#[export_name = "g_thread_functions_for_glib_use"]
pub static PLACEHOLDER_04345: u8 = 0;

#[export_name = "g_thread_get_initialized"]
pub static PLACEHOLDER_04348: u8 = 0;

#[export_name = "g_thread_gettime"]
pub static PLACEHOLDER_04351: u8 = 0;

#[export_name = "g_thread_init_glib"]
pub static PLACEHOLDER_04354: u8 = 0;

#[export_name = "g_thread_join"]
pub unsafe extern "C" fn export_g_thread_join(
    thread: *mut crate::runtime::GThreadHandle,
) -> crate::ffi::gpointer {
    crate::runtime::g_thread_join_impl(thread)
}

#[export_name = "g_thread_new"]
pub unsafe extern "C" fn export_g_thread_new(
    name: *const crate::ffi::gchar,
    func: crate::runtime::GThreadFunc,
    data: crate::ffi::gpointer,
) -> *mut crate::runtime::GThreadHandle {
    crate::runtime::g_thread_new_impl(name, func, data)
}

#[export_name = "g_thread_pool_free"]
pub static PLACEHOLDER_04361: u8 = 0;

#[export_name = "g_thread_pool_get_max_idle_time"]
pub static PLACEHOLDER_04364: u8 = 0;

#[export_name = "g_thread_pool_get_max_threads"]
pub static PLACEHOLDER_04367: u8 = 0;

#[export_name = "g_thread_pool_get_max_unused_threads"]
pub static PLACEHOLDER_04370: u8 = 0;

#[export_name = "g_thread_pool_get_num_threads"]
pub static PLACEHOLDER_04373: u8 = 0;

#[export_name = "g_thread_pool_get_num_unused_threads"]
pub static PLACEHOLDER_04376: u8 = 0;

#[export_name = "g_thread_pool_move_to_front"]
pub static PLACEHOLDER_04379: u8 = 0;

#[export_name = "g_thread_pool_new"]
pub static PLACEHOLDER_04382: u8 = 0;

#[export_name = "g_thread_pool_new_full"]
pub static PLACEHOLDER_04385: u8 = 0;

#[export_name = "g_thread_pool_push"]
pub static PLACEHOLDER_04388: u8 = 0;

#[export_name = "g_thread_pool_set_max_idle_time"]
pub static PLACEHOLDER_04391: u8 = 0;

#[export_name = "g_thread_pool_set_max_threads"]
pub static PLACEHOLDER_04394: u8 = 0;

#[export_name = "g_thread_pool_set_max_unused_threads"]
pub static PLACEHOLDER_04397: u8 = 0;

#[export_name = "g_thread_pool_set_sort_function"]
pub static PLACEHOLDER_04400: u8 = 0;

#[export_name = "g_thread_pool_stop_unused_threads"]
pub static PLACEHOLDER_04403: u8 = 0;

#[export_name = "g_thread_pool_unprocessed"]
pub static PLACEHOLDER_04406: u8 = 0;

#[export_name = "g_thread_ref"]
pub static PLACEHOLDER_04409: u8 = 0;

#[export_name = "g_thread_self"]
pub static PLACEHOLDER_04412: u8 = 0;

#[export_name = "g_thread_set_priority"]
pub static PLACEHOLDER_04415: u8 = 0;

#[export_name = "g_thread_try_new"]
pub static PLACEHOLDER_04418: u8 = 0;

#[export_name = "g_thread_unref"]
pub static PLACEHOLDER_04421: u8 = 0;

#[export_name = "g_thread_use_default_impl"]
pub static PLACEHOLDER_04424: u8 = 0;

#[export_name = "g_thread_yield"]
pub static PLACEHOLDER_04427: u8 = 0;

#[export_name = "g_threads_got_initialized"]
pub static PLACEHOLDER_04430: u8 = 0;

#[export_name = "g_time_val_add"]
pub static PLACEHOLDER_04433: u8 = 0;

#[export_name = "g_time_val_from_iso8601"]
pub static PLACEHOLDER_04436: u8 = 0;

#[export_name = "g_time_val_to_iso8601"]
pub static PLACEHOLDER_04439: u8 = 0;

#[export_name = "g_time_zone_adjust_time"]
pub static PLACEHOLDER_04442: u8 = 0;

#[export_name = "g_time_zone_find_interval"]
pub static PLACEHOLDER_04445: u8 = 0;

#[export_name = "g_time_zone_get_abbreviation"]
pub static PLACEHOLDER_04448: u8 = 0;

#[export_name = "g_time_zone_get_identifier"]
pub static PLACEHOLDER_04451: u8 = 0;

#[export_name = "g_time_zone_get_offset"]
pub static PLACEHOLDER_04454: u8 = 0;

#[export_name = "g_time_zone_is_dst"]
pub static PLACEHOLDER_04457: u8 = 0;

#[export_name = "g_time_zone_new"]
pub static PLACEHOLDER_04460: u8 = 0;

#[export_name = "g_time_zone_new_identifier"]
pub static PLACEHOLDER_04463: u8 = 0;

#[export_name = "g_time_zone_new_local"]
pub static PLACEHOLDER_04466: u8 = 0;

#[export_name = "g_time_zone_new_offset"]
pub static PLACEHOLDER_04469: u8 = 0;

#[export_name = "g_time_zone_new_utc"]
pub static PLACEHOLDER_04472: u8 = 0;

#[export_name = "g_time_zone_ref"]
pub static PLACEHOLDER_04475: u8 = 0;

#[export_name = "g_time_zone_unref"]
pub static PLACEHOLDER_04478: u8 = 0;

#[export_name = "g_timeout_add"]
pub static PLACEHOLDER_04481: u8 = 0;

#[export_name = "g_timeout_add_full"]
pub static PLACEHOLDER_04484: u8 = 0;

#[export_name = "g_timeout_add_once"]
pub static PLACEHOLDER_04487: u8 = 0;

#[export_name = "g_timeout_add_seconds"]
pub static PLACEHOLDER_04490: u8 = 0;

#[export_name = "g_timeout_add_seconds_full"]
pub static PLACEHOLDER_04493: u8 = 0;

#[export_name = "g_timeout_add_seconds_once"]
pub static PLACEHOLDER_04496: u8 = 0;

#[export_name = "g_timeout_funcs"]
pub static PLACEHOLDER_04499: u8 = 0;

#[export_name = "g_timeout_source_new"]
pub static PLACEHOLDER_04502: u8 = 0;

#[export_name = "g_timeout_source_new_seconds"]
pub static PLACEHOLDER_04505: u8 = 0;

#[export_name = "g_timer_continue"]
pub static PLACEHOLDER_04508: u8 = 0;

#[export_name = "g_timer_destroy"]
pub static PLACEHOLDER_04511: u8 = 0;

#[export_name = "g_timer_elapsed"]
pub static PLACEHOLDER_04514: u8 = 0;

#[export_name = "g_timer_is_active"]
pub static PLACEHOLDER_04517: u8 = 0;

#[export_name = "g_timer_new"]
pub static PLACEHOLDER_04520: u8 = 0;

#[export_name = "g_timer_reset"]
pub static PLACEHOLDER_04523: u8 = 0;

#[export_name = "g_timer_start"]
pub static PLACEHOLDER_04526: u8 = 0;

#[export_name = "g_timer_stop"]
pub static PLACEHOLDER_04529: u8 = 0;

#[export_name = "g_trash_stack_height"]
pub static PLACEHOLDER_04532: u8 = 0;

#[export_name = "g_trash_stack_peek"]
pub static PLACEHOLDER_04535: u8 = 0;

#[export_name = "g_trash_stack_pop"]
pub static PLACEHOLDER_04538: u8 = 0;

#[export_name = "g_trash_stack_push"]
pub static PLACEHOLDER_04541: u8 = 0;

#[export_name = "g_tree_destroy"]
pub static PLACEHOLDER_04544: u8 = 0;

#[export_name = "g_tree_foreach"]
pub static PLACEHOLDER_04547: u8 = 0;

#[export_name = "g_tree_foreach_node"]
pub static PLACEHOLDER_04550: u8 = 0;

#[export_name = "g_tree_height"]
pub static PLACEHOLDER_04553: u8 = 0;

#[export_name = "g_tree_insert"]
pub static PLACEHOLDER_04556: u8 = 0;

#[export_name = "g_tree_insert_node"]
pub static PLACEHOLDER_04559: u8 = 0;

#[export_name = "g_tree_lookup"]
pub static PLACEHOLDER_04562: u8 = 0;

#[export_name = "g_tree_lookup_extended"]
pub static PLACEHOLDER_04565: u8 = 0;

#[export_name = "g_tree_lookup_node"]
pub static PLACEHOLDER_04568: u8 = 0;

#[export_name = "g_tree_lower_bound"]
pub static PLACEHOLDER_04571: u8 = 0;

#[export_name = "g_tree_new"]
pub static PLACEHOLDER_04574: u8 = 0;

#[export_name = "g_tree_new_full"]
pub static PLACEHOLDER_04577: u8 = 0;

#[export_name = "g_tree_new_with_data"]
pub static PLACEHOLDER_04580: u8 = 0;

#[export_name = "g_tree_nnodes"]
pub static PLACEHOLDER_04583: u8 = 0;

#[export_name = "g_tree_node_first"]
pub static PLACEHOLDER_04586: u8 = 0;

#[export_name = "g_tree_node_key"]
pub static PLACEHOLDER_04589: u8 = 0;

#[export_name = "g_tree_node_last"]
pub static PLACEHOLDER_04592: u8 = 0;

#[export_name = "g_tree_node_next"]
pub static PLACEHOLDER_04595: u8 = 0;

#[export_name = "g_tree_node_previous"]
pub static PLACEHOLDER_04598: u8 = 0;

#[export_name = "g_tree_node_value"]
pub static PLACEHOLDER_04601: u8 = 0;

#[export_name = "g_tree_ref"]
pub static PLACEHOLDER_04604: u8 = 0;

#[export_name = "g_tree_remove"]
pub static PLACEHOLDER_04607: u8 = 0;

#[export_name = "g_tree_remove_all"]
pub static PLACEHOLDER_04610: u8 = 0;

#[export_name = "g_tree_replace"]
pub static PLACEHOLDER_04613: u8 = 0;

#[export_name = "g_tree_replace_node"]
pub static PLACEHOLDER_04616: u8 = 0;

#[export_name = "g_tree_search"]
pub static PLACEHOLDER_04619: u8 = 0;

#[export_name = "g_tree_search_node"]
pub static PLACEHOLDER_04622: u8 = 0;

#[export_name = "g_tree_steal"]
pub static PLACEHOLDER_04625: u8 = 0;

#[export_name = "g_tree_traverse"]
pub static PLACEHOLDER_04628: u8 = 0;

#[export_name = "g_tree_unref"]
pub static PLACEHOLDER_04631: u8 = 0;

#[export_name = "g_tree_upper_bound"]
pub static PLACEHOLDER_04634: u8 = 0;

#[export_name = "g_try_malloc0"]
pub static PLACEHOLDER_04637: u8 = 0;

#[export_name = "g_try_malloc0_n"]
pub static PLACEHOLDER_04640: u8 = 0;

#[export_name = "g_try_malloc"]
pub static PLACEHOLDER_04643: u8 = 0;

#[export_name = "g_try_malloc_n"]
pub static PLACEHOLDER_04646: u8 = 0;

#[export_name = "g_try_realloc"]
pub static PLACEHOLDER_04649: u8 = 0;

#[export_name = "g_try_realloc_n"]
pub static PLACEHOLDER_04652: u8 = 0;

#[export_name = "g_tuples_destroy"]
pub static PLACEHOLDER_04655: u8 = 0;

#[export_name = "g_tuples_index"]
pub static PLACEHOLDER_04658: u8 = 0;

#[export_name = "g_ucs4_to_utf16"]
pub static PLACEHOLDER_04661: u8 = 0;

#[export_name = "g_ucs4_to_utf8"]
pub static PLACEHOLDER_04664: u8 = 0;

#[export_name = "g_unichar_break_type"]
pub static PLACEHOLDER_04667: u8 = 0;

#[export_name = "g_unichar_combining_class"]
pub static PLACEHOLDER_04670: u8 = 0;

#[export_name = "g_unichar_compose"]
pub static PLACEHOLDER_04673: u8 = 0;

#[export_name = "g_unichar_decompose"]
pub static PLACEHOLDER_04676: u8 = 0;

#[export_name = "g_unichar_digit_value"]
pub static PLACEHOLDER_04679: u8 = 0;

#[export_name = "g_unichar_fully_decompose"]
pub static PLACEHOLDER_04682: u8 = 0;

#[export_name = "g_unichar_get_mirror_char"]
pub static PLACEHOLDER_04685: u8 = 0;

#[export_name = "g_unichar_get_script"]
pub static PLACEHOLDER_04688: u8 = 0;

#[export_name = "g_unichar_isalnum"]
pub static PLACEHOLDER_04691: u8 = 0;

#[export_name = "g_unichar_isalpha"]
pub static PLACEHOLDER_04694: u8 = 0;

#[export_name = "g_unichar_iscntrl"]
pub static PLACEHOLDER_04697: u8 = 0;

#[export_name = "g_unichar_isdefined"]
pub static PLACEHOLDER_04700: u8 = 0;

#[export_name = "g_unichar_isdigit"]
pub static PLACEHOLDER_04703: u8 = 0;

#[export_name = "g_unichar_isgraph"]
pub static PLACEHOLDER_04706: u8 = 0;

#[export_name = "g_unichar_islower"]
pub static PLACEHOLDER_04709: u8 = 0;

#[export_name = "g_unichar_ismark"]
pub static PLACEHOLDER_04712: u8 = 0;

#[export_name = "g_unichar_isprint"]
pub static PLACEHOLDER_04715: u8 = 0;

#[export_name = "g_unichar_ispunct"]
pub static PLACEHOLDER_04718: u8 = 0;

#[export_name = "g_unichar_isspace"]
pub static PLACEHOLDER_04721: u8 = 0;

#[export_name = "g_unichar_istitle"]
pub static PLACEHOLDER_04724: u8 = 0;

#[export_name = "g_unichar_isupper"]
pub static PLACEHOLDER_04727: u8 = 0;

#[export_name = "g_unichar_iswide"]
pub static PLACEHOLDER_04730: u8 = 0;

#[export_name = "g_unichar_iswide_cjk"]
pub static PLACEHOLDER_04733: u8 = 0;

#[export_name = "g_unichar_isxdigit"]
pub static PLACEHOLDER_04736: u8 = 0;

#[export_name = "g_unichar_iszerowidth"]
pub static PLACEHOLDER_04739: u8 = 0;

#[export_name = "g_unichar_to_utf8"]
pub static PLACEHOLDER_04742: u8 = 0;

#[export_name = "g_unichar_tolower"]
pub static PLACEHOLDER_04745: u8 = 0;

#[export_name = "g_unichar_totitle"]
pub static PLACEHOLDER_04748: u8 = 0;

#[export_name = "g_unichar_toupper"]
pub static PLACEHOLDER_04751: u8 = 0;

#[export_name = "g_unichar_type"]
pub static PLACEHOLDER_04754: u8 = 0;

#[export_name = "g_unichar_validate"]
pub static PLACEHOLDER_04757: u8 = 0;

#[export_name = "g_unichar_xdigit_value"]
pub static PLACEHOLDER_04760: u8 = 0;

#[export_name = "g_unicode_canonical_decomposition"]
pub static PLACEHOLDER_04763: u8 = 0;

#[export_name = "g_unicode_canonical_ordering"]
pub static PLACEHOLDER_04766: u8 = 0;

#[export_name = "g_unicode_script_from_iso15924"]
pub static PLACEHOLDER_04769: u8 = 0;

#[export_name = "g_unicode_script_to_iso15924"]
pub static PLACEHOLDER_04772: u8 = 0;

#[export_name = "g_unix_error_quark"]
pub static PLACEHOLDER_04775: u8 = 0;

#[export_name = "g_unix_fd_add"]
pub static PLACEHOLDER_04778: u8 = 0;

#[export_name = "g_unix_fd_add_full"]
pub static PLACEHOLDER_04781: u8 = 0;

#[export_name = "g_unix_fd_source_funcs"]
pub static PLACEHOLDER_04784: u8 = 0;

#[export_name = "g_unix_fd_source_new"]
pub static PLACEHOLDER_04787: u8 = 0;

#[export_name = "g_unix_get_passwd_entry"]
pub static PLACEHOLDER_04790: u8 = 0;

#[export_name = "g_unix_open_pipe"]
pub static PLACEHOLDER_04793: u8 = 0;

#[export_name = "g_unix_set_fd_nonblocking"]
pub static PLACEHOLDER_04796: u8 = 0;

#[export_name = "g_unix_signal_add"]
pub static PLACEHOLDER_04799: u8 = 0;

#[export_name = "g_unix_signal_add_full"]
pub static PLACEHOLDER_04802: u8 = 0;

#[export_name = "g_unix_signal_funcs"]
pub static PLACEHOLDER_04805: u8 = 0;

#[export_name = "g_unix_signal_source_new"]
pub static PLACEHOLDER_04808: u8 = 0;

#[export_name = "g_unlink"]
pub static PLACEHOLDER_04811: u8 = 0;

#[export_name = "g_unsetenv"]
pub static PLACEHOLDER_04814: u8 = 0;

#[export_name = "g_uri_build"]
pub static PLACEHOLDER_04817: u8 = 0;

#[export_name = "g_uri_build_with_user"]
pub static PLACEHOLDER_04820: u8 = 0;

#[export_name = "g_uri_error_quark"]
pub static PLACEHOLDER_04823: u8 = 0;

#[export_name = "g_uri_escape_bytes"]
pub static PLACEHOLDER_04826: u8 = 0;

#[export_name = "g_uri_escape_string"]
pub static PLACEHOLDER_04829: u8 = 0;

#[export_name = "g_uri_get_auth_params"]
pub static PLACEHOLDER_04832: u8 = 0;

#[export_name = "g_uri_get_flags"]
pub static PLACEHOLDER_04835: u8 = 0;

#[export_name = "g_uri_get_fragment"]
pub static PLACEHOLDER_04838: u8 = 0;

#[export_name = "g_uri_get_host"]
pub static PLACEHOLDER_04841: u8 = 0;

#[export_name = "g_uri_get_password"]
pub static PLACEHOLDER_04844: u8 = 0;

#[export_name = "g_uri_get_path"]
pub static PLACEHOLDER_04847: u8 = 0;

#[export_name = "g_uri_get_port"]
pub static PLACEHOLDER_04850: u8 = 0;

#[export_name = "g_uri_get_query"]
pub static PLACEHOLDER_04853: u8 = 0;

#[export_name = "g_uri_get_scheme"]
pub static PLACEHOLDER_04856: u8 = 0;

#[export_name = "g_uri_get_user"]
pub static PLACEHOLDER_04859: u8 = 0;

#[export_name = "g_uri_get_userinfo"]
pub static PLACEHOLDER_04862: u8 = 0;

#[export_name = "g_uri_is_valid"]
pub static PLACEHOLDER_04865: u8 = 0;

#[export_name = "g_uri_join"]
pub static PLACEHOLDER_04868: u8 = 0;

#[export_name = "g_uri_join_with_user"]
pub static PLACEHOLDER_04871: u8 = 0;

#[export_name = "g_uri_list_extract_uris"]
pub static PLACEHOLDER_04874: u8 = 0;

#[export_name = "g_uri_params_iter_init"]
pub static PLACEHOLDER_04877: u8 = 0;

#[export_name = "g_uri_params_iter_next"]
pub static PLACEHOLDER_04880: u8 = 0;

#[export_name = "g_uri_parse"]
pub static PLACEHOLDER_04883: u8 = 0;

#[export_name = "g_uri_parse_params"]
pub static PLACEHOLDER_04886: u8 = 0;

#[export_name = "g_uri_parse_relative"]
pub static PLACEHOLDER_04889: u8 = 0;

#[export_name = "g_uri_parse_scheme"]
pub static PLACEHOLDER_04892: u8 = 0;

#[export_name = "g_uri_peek_scheme"]
pub static PLACEHOLDER_04895: u8 = 0;

#[export_name = "g_uri_ref"]
pub static PLACEHOLDER_04898: u8 = 0;

#[export_name = "g_uri_resolve_relative"]
pub static PLACEHOLDER_04901: u8 = 0;

#[export_name = "g_uri_split"]
pub static PLACEHOLDER_04904: u8 = 0;

#[export_name = "g_uri_split_network"]
pub static PLACEHOLDER_04907: u8 = 0;

#[export_name = "g_uri_split_with_user"]
pub static PLACEHOLDER_04910: u8 = 0;

#[export_name = "g_uri_to_string"]
pub static PLACEHOLDER_04913: u8 = 0;

#[export_name = "g_uri_to_string_partial"]
pub static PLACEHOLDER_04916: u8 = 0;

#[export_name = "g_uri_unescape_bytes"]
pub static PLACEHOLDER_04919: u8 = 0;

#[export_name = "g_uri_unescape_segment"]
pub static PLACEHOLDER_04922: u8 = 0;

#[export_name = "g_uri_unescape_string"]
pub static PLACEHOLDER_04925: u8 = 0;

#[export_name = "g_uri_unref"]
pub static PLACEHOLDER_04928: u8 = 0;

#[export_name = "g_usleep"]
pub static PLACEHOLDER_04931: u8 = 0;

#[export_name = "g_utf16_to_ucs4"]
pub static PLACEHOLDER_04934: u8 = 0;

#[export_name = "g_utf16_to_utf8"]
pub static PLACEHOLDER_04937: u8 = 0;

#[export_name = "g_utf8_casefold"]
pub static PLACEHOLDER_04940: u8 = 0;

#[export_name = "g_utf8_collate"]
pub static PLACEHOLDER_04943: u8 = 0;

#[export_name = "g_utf8_collate_key"]
pub static PLACEHOLDER_04946: u8 = 0;

#[export_name = "g_utf8_collate_key_for_filename"]
pub static PLACEHOLDER_04949: u8 = 0;

#[export_name = "g_utf8_find_next_char"]
pub static PLACEHOLDER_04952: u8 = 0;

#[export_name = "g_utf8_find_prev_char"]
pub static PLACEHOLDER_04955: u8 = 0;

#[export_name = "g_utf8_get_char"]
pub static PLACEHOLDER_04958: u8 = 0;

#[export_name = "g_utf8_get_char_validated"]
pub static PLACEHOLDER_04961: u8 = 0;

#[export_name = "g_utf8_make_valid"]
pub static PLACEHOLDER_04964: u8 = 0;

#[export_name = "g_utf8_normalize"]
pub static PLACEHOLDER_04967: u8 = 0;

#[export_name = "g_utf8_offset_to_pointer"]
pub static PLACEHOLDER_04970: u8 = 0;

#[export_name = "g_utf8_pointer_to_offset"]
pub static PLACEHOLDER_04973: u8 = 0;

#[export_name = "g_utf8_prev_char"]
pub static PLACEHOLDER_04976: u8 = 0;

#[export_name = "g_utf8_skip"]
pub static PLACEHOLDER_04979: u8 = 0;

#[export_name = "g_utf8_strchr"]
pub static PLACEHOLDER_04982: u8 = 0;

#[export_name = "g_utf8_strdown"]
pub static PLACEHOLDER_04985: u8 = 0;

#[export_name = "g_utf8_strlen"]
pub static PLACEHOLDER_04988: u8 = 0;

#[export_name = "g_utf8_strncpy"]
pub static PLACEHOLDER_04991: u8 = 0;

#[export_name = "g_utf8_strrchr"]
pub static PLACEHOLDER_04994: u8 = 0;

#[export_name = "g_utf8_strreverse"]
pub static PLACEHOLDER_04997: u8 = 0;

#[export_name = "g_utf8_strup"]
pub static PLACEHOLDER_05000: u8 = 0;

#[export_name = "g_utf8_substring"]
pub static PLACEHOLDER_05003: u8 = 0;

#[export_name = "g_utf8_to_ucs4"]
pub static PLACEHOLDER_05006: u8 = 0;

#[export_name = "g_utf8_to_ucs4_fast"]
pub static PLACEHOLDER_05009: u8 = 0;

#[export_name = "g_utf8_to_utf16"]
pub static PLACEHOLDER_05012: u8 = 0;

#[export_name = "g_utf8_truncate_middle"]
pub static PLACEHOLDER_05015: u8 = 0;

#[export_name = "g_utf8_validate"]
pub static PLACEHOLDER_05018: u8 = 0;

#[export_name = "g_utf8_validate_len"]
pub static PLACEHOLDER_05021: u8 = 0;

#[export_name = "g_utime"]
pub static PLACEHOLDER_05024: u8 = 0;

#[export_name = "g_uuid_string_is_valid"]
pub static PLACEHOLDER_05027: u8 = 0;

#[export_name = "g_uuid_string_random"]
pub static PLACEHOLDER_05030: u8 = 0;

#[export_name = "g_variant_builder_add"]
pub static PLACEHOLDER_05033: u8 = 0;

#[export_name = "g_variant_builder_add_parsed"]
pub static PLACEHOLDER_05036: u8 = 0;

#[export_name = "g_variant_builder_add_value"]
pub static PLACEHOLDER_05039: u8 = 0;

#[export_name = "g_variant_builder_clear"]
pub static PLACEHOLDER_05042: u8 = 0;

#[export_name = "g_variant_builder_close"]
pub static PLACEHOLDER_05045: u8 = 0;

#[export_name = "g_variant_builder_end"]
pub static PLACEHOLDER_05048: u8 = 0;

#[export_name = "g_variant_builder_init"]
pub static PLACEHOLDER_05051: u8 = 0;

#[export_name = "g_variant_builder_new"]
pub static PLACEHOLDER_05054: u8 = 0;

#[export_name = "g_variant_builder_open"]
pub static PLACEHOLDER_05057: u8 = 0;

#[export_name = "g_variant_builder_ref"]
pub static PLACEHOLDER_05060: u8 = 0;

#[export_name = "g_variant_builder_unref"]
pub static PLACEHOLDER_05063: u8 = 0;

#[export_name = "g_variant_byteswap"]
pub static PLACEHOLDER_05066: u8 = 0;

#[export_name = "g_variant_check_format_string"]
pub static PLACEHOLDER_05069: u8 = 0;

#[export_name = "g_variant_classify"]
pub static PLACEHOLDER_05072: u8 = 0;

#[export_name = "g_variant_compare"]
pub static PLACEHOLDER_05075: u8 = 0;

#[export_name = "g_variant_dict_clear"]
pub static PLACEHOLDER_05078: u8 = 0;

#[export_name = "g_variant_dict_contains"]
pub static PLACEHOLDER_05081: u8 = 0;

#[export_name = "g_variant_dict_end"]
pub static PLACEHOLDER_05084: u8 = 0;

#[export_name = "g_variant_dict_init"]
pub static PLACEHOLDER_05087: u8 = 0;

#[export_name = "g_variant_dict_insert"]
pub static PLACEHOLDER_05090: u8 = 0;

#[export_name = "g_variant_dict_insert_value"]
pub static PLACEHOLDER_05093: u8 = 0;

#[export_name = "g_variant_dict_lookup"]
pub static PLACEHOLDER_05096: u8 = 0;

#[export_name = "g_variant_dict_lookup_value"]
pub static PLACEHOLDER_05099: u8 = 0;

#[export_name = "g_variant_dict_new"]
pub static PLACEHOLDER_05102: u8 = 0;

#[export_name = "g_variant_dict_ref"]
pub static PLACEHOLDER_05105: u8 = 0;

#[export_name = "g_variant_dict_remove"]
pub static PLACEHOLDER_05108: u8 = 0;

#[export_name = "g_variant_dict_unref"]
pub static PLACEHOLDER_05111: u8 = 0;

#[export_name = "g_variant_dup_bytestring"]
pub static PLACEHOLDER_05114: u8 = 0;

#[export_name = "g_variant_dup_bytestring_array"]
pub static PLACEHOLDER_05117: u8 = 0;

#[export_name = "g_variant_dup_objv"]
pub static PLACEHOLDER_05120: u8 = 0;

#[export_name = "g_variant_dup_string"]
pub static PLACEHOLDER_05123: u8 = 0;

#[export_name = "g_variant_dup_strv"]
pub static PLACEHOLDER_05126: u8 = 0;

#[export_name = "g_variant_equal"]
pub static PLACEHOLDER_05129: u8 = 0;

#[export_name = "g_variant_format_string_scan"]
pub static PLACEHOLDER_05132: u8 = 0;

#[export_name = "g_variant_format_string_scan_type"]
pub static PLACEHOLDER_05135: u8 = 0;

#[export_name = "g_variant_get"]
pub static PLACEHOLDER_05138: u8 = 0;

#[export_name = "g_variant_get_boolean"]
pub static PLACEHOLDER_05141: u8 = 0;

#[export_name = "g_variant_get_byte"]
pub static PLACEHOLDER_05144: u8 = 0;

#[export_name = "g_variant_get_bytestring"]
pub static PLACEHOLDER_05147: u8 = 0;

#[export_name = "g_variant_get_bytestring_array"]
pub static PLACEHOLDER_05150: u8 = 0;

#[export_name = "g_variant_get_child"]
pub static PLACEHOLDER_05153: u8 = 0;

#[export_name = "g_variant_get_child_value"]
pub static PLACEHOLDER_05156: u8 = 0;

#[export_name = "g_variant_get_data"]
pub static PLACEHOLDER_05159: u8 = 0;

#[export_name = "g_variant_get_data_as_bytes"]
pub static PLACEHOLDER_05162: u8 = 0;

#[export_name = "g_variant_get_double"]
pub static PLACEHOLDER_05165: u8 = 0;

#[export_name = "g_variant_get_fixed_array"]
pub static PLACEHOLDER_05168: u8 = 0;

#[export_name = "g_variant_get_handle"]
pub static PLACEHOLDER_05171: u8 = 0;

#[export_name = "g_variant_get_int16"]
pub static PLACEHOLDER_05174: u8 = 0;

#[export_name = "g_variant_get_int32"]
pub static PLACEHOLDER_05177: u8 = 0;

#[export_name = "g_variant_get_int64"]
pub static PLACEHOLDER_05180: u8 = 0;

#[export_name = "g_variant_get_maybe"]
pub static PLACEHOLDER_05183: u8 = 0;

#[export_name = "g_variant_get_normal_form"]
pub static PLACEHOLDER_05186: u8 = 0;

#[export_name = "g_variant_get_objv"]
pub static PLACEHOLDER_05189: u8 = 0;

#[export_name = "g_variant_get_size"]
pub static PLACEHOLDER_05192: u8 = 0;

#[export_name = "g_variant_get_string"]
pub static PLACEHOLDER_05195: u8 = 0;

#[export_name = "g_variant_get_strv"]
pub static PLACEHOLDER_05198: u8 = 0;

#[export_name = "g_variant_get_type"]
pub static PLACEHOLDER_05201: u8 = 0;

#[export_name = "g_variant_get_type_string"]
pub static PLACEHOLDER_05204: u8 = 0;

#[export_name = "g_variant_get_uint16"]
pub static PLACEHOLDER_05207: u8 = 0;

#[export_name = "g_variant_get_uint32"]
pub static PLACEHOLDER_05210: u8 = 0;

#[export_name = "g_variant_get_uint64"]
pub static PLACEHOLDER_05213: u8 = 0;

#[export_name = "g_variant_get_va"]
pub static PLACEHOLDER_05216: u8 = 0;

#[export_name = "g_variant_get_variant"]
pub static PLACEHOLDER_05219: u8 = 0;

#[export_name = "g_variant_hash"]
pub static PLACEHOLDER_05222: u8 = 0;

#[export_name = "g_variant_is_container"]
pub static PLACEHOLDER_05225: u8 = 0;

#[export_name = "g_variant_is_floating"]
pub static PLACEHOLDER_05228: u8 = 0;

#[export_name = "g_variant_is_normal_form"]
pub static PLACEHOLDER_05231: u8 = 0;

#[export_name = "g_variant_is_object_path"]
pub static PLACEHOLDER_05234: u8 = 0;

#[export_name = "g_variant_is_of_type"]
pub static PLACEHOLDER_05237: u8 = 0;

#[export_name = "g_variant_is_signature"]
pub static PLACEHOLDER_05240: u8 = 0;

#[export_name = "g_variant_iter_copy"]
pub static PLACEHOLDER_05243: u8 = 0;

#[export_name = "g_variant_iter_free"]
pub static PLACEHOLDER_05246: u8 = 0;

#[export_name = "g_variant_iter_init"]
pub static PLACEHOLDER_05249: u8 = 0;

#[export_name = "g_variant_iter_loop"]
pub static PLACEHOLDER_05252: u8 = 0;

#[export_name = "g_variant_iter_n_children"]
pub static PLACEHOLDER_05255: u8 = 0;

#[export_name = "g_variant_iter_new"]
pub static PLACEHOLDER_05258: u8 = 0;

#[export_name = "g_variant_iter_next"]
pub static PLACEHOLDER_05261: u8 = 0;

#[export_name = "g_variant_iter_next_value"]
pub static PLACEHOLDER_05264: u8 = 0;

#[export_name = "g_variant_lookup"]
pub static PLACEHOLDER_05267: u8 = 0;

#[export_name = "g_variant_lookup_value"]
pub static PLACEHOLDER_05270: u8 = 0;

#[export_name = "g_variant_n_children"]
pub static PLACEHOLDER_05273: u8 = 0;

#[export_name = "g_variant_new"]
pub static PLACEHOLDER_05276: u8 = 0;

#[export_name = "g_variant_new_array"]
pub static PLACEHOLDER_05279: u8 = 0;

#[export_name = "g_variant_new_boolean"]
pub static PLACEHOLDER_05282: u8 = 0;

#[export_name = "g_variant_new_byte"]
pub static PLACEHOLDER_05285: u8 = 0;

#[export_name = "g_variant_new_bytestring"]
pub static PLACEHOLDER_05288: u8 = 0;

#[export_name = "g_variant_new_bytestring_array"]
pub static PLACEHOLDER_05291: u8 = 0;

#[export_name = "g_variant_new_dict_entry"]
pub static PLACEHOLDER_05294: u8 = 0;

#[export_name = "g_variant_new_double"]
pub static PLACEHOLDER_05297: u8 = 0;

#[export_name = "g_variant_new_fixed_array"]
pub static PLACEHOLDER_05300: u8 = 0;

#[export_name = "g_variant_new_from_bytes"]
pub static PLACEHOLDER_05303: u8 = 0;

#[export_name = "g_variant_new_from_data"]
pub static PLACEHOLDER_05306: u8 = 0;

#[export_name = "g_variant_new_handle"]
pub static PLACEHOLDER_05309: u8 = 0;

#[export_name = "g_variant_new_int16"]
pub static PLACEHOLDER_05312: u8 = 0;

#[export_name = "g_variant_new_int32"]
pub static PLACEHOLDER_05315: u8 = 0;

#[export_name = "g_variant_new_int64"]
pub static PLACEHOLDER_05318: u8 = 0;

#[export_name = "g_variant_new_maybe"]
pub static PLACEHOLDER_05321: u8 = 0;

#[export_name = "g_variant_new_object_path"]
pub static PLACEHOLDER_05324: u8 = 0;

#[export_name = "g_variant_new_objv"]
pub static PLACEHOLDER_05327: u8 = 0;

#[export_name = "g_variant_new_parsed"]
pub static PLACEHOLDER_05330: u8 = 0;

#[export_name = "g_variant_new_parsed_va"]
pub static PLACEHOLDER_05333: u8 = 0;

#[export_name = "g_variant_new_printf"]
pub static PLACEHOLDER_05336: u8 = 0;

#[export_name = "g_variant_new_signature"]
pub static PLACEHOLDER_05339: u8 = 0;

#[export_name = "g_variant_new_string"]
pub static PLACEHOLDER_05342: u8 = 0;

#[export_name = "g_variant_new_strv"]
pub static PLACEHOLDER_05345: u8 = 0;

#[export_name = "g_variant_new_take_string"]
pub static PLACEHOLDER_05348: u8 = 0;

#[export_name = "g_variant_new_tuple"]
pub static PLACEHOLDER_05351: u8 = 0;

#[export_name = "g_variant_new_uint16"]
pub static PLACEHOLDER_05354: u8 = 0;

#[export_name = "g_variant_new_uint32"]
pub static PLACEHOLDER_05357: u8 = 0;

#[export_name = "g_variant_new_uint64"]
pub static PLACEHOLDER_05360: u8 = 0;

#[export_name = "g_variant_new_va"]
pub static PLACEHOLDER_05363: u8 = 0;

#[export_name = "g_variant_new_variant"]
pub static PLACEHOLDER_05366: u8 = 0;

#[export_name = "g_variant_parse"]
pub static PLACEHOLDER_05369: u8 = 0;

#[export_name = "g_variant_parse_error_print_context"]
pub static PLACEHOLDER_05372: u8 = 0;

#[export_name = "g_variant_parse_error_quark"]
pub static PLACEHOLDER_05375: u8 = 0;

#[export_name = "g_variant_parser_get_error_quark"]
pub static PLACEHOLDER_05378: u8 = 0;

#[export_name = "g_variant_print"]
pub static PLACEHOLDER_05381: u8 = 0;

#[export_name = "g_variant_print_string"]
pub static PLACEHOLDER_05384: u8 = 0;

#[export_name = "g_variant_ref"]
pub static PLACEHOLDER_05387: u8 = 0;

#[export_name = "g_variant_ref_sink"]
pub static PLACEHOLDER_05390: u8 = 0;

#[export_name = "g_variant_serialised_byteswap"]
pub static PLACEHOLDER_05393: u8 = 0;

#[export_name = "g_variant_serialised_check"]
pub static PLACEHOLDER_05396: u8 = 0;

#[export_name = "g_variant_serialised_get_child"]
pub static PLACEHOLDER_05399: u8 = 0;

#[export_name = "g_variant_serialised_is_normal"]
pub static PLACEHOLDER_05402: u8 = 0;

#[export_name = "g_variant_serialised_n_children"]
pub static PLACEHOLDER_05405: u8 = 0;

#[export_name = "g_variant_serialiser_is_object_path"]
pub static PLACEHOLDER_05408: u8 = 0;

#[export_name = "g_variant_serialiser_is_signature"]
pub static PLACEHOLDER_05411: u8 = 0;

#[export_name = "g_variant_serialiser_is_string"]
pub static PLACEHOLDER_05414: u8 = 0;

#[export_name = "g_variant_serialiser_needed_size"]
pub static PLACEHOLDER_05417: u8 = 0;

#[export_name = "g_variant_serialiser_serialise"]
pub static PLACEHOLDER_05420: u8 = 0;

#[export_name = "g_variant_store"]
pub static PLACEHOLDER_05423: u8 = 0;

#[export_name = "g_variant_take_ref"]
pub static PLACEHOLDER_05426: u8 = 0;

#[export_name = "g_variant_type_checked_"]
pub static PLACEHOLDER_05429: u8 = 0;

#[export_name = "g_variant_type_copy"]
pub static PLACEHOLDER_05432: u8 = 0;

#[export_name = "g_variant_type_dup_string"]
pub static PLACEHOLDER_05435: u8 = 0;

#[export_name = "g_variant_type_element"]
pub static PLACEHOLDER_05438: u8 = 0;

#[export_name = "g_variant_type_equal"]
pub static PLACEHOLDER_05441: u8 = 0;

#[export_name = "g_variant_type_first"]
pub static PLACEHOLDER_05444: u8 = 0;

#[export_name = "g_variant_type_free"]
pub static PLACEHOLDER_05447: u8 = 0;

#[export_name = "g_variant_type_get_string_length"]
pub static PLACEHOLDER_05450: u8 = 0;

#[export_name = "g_variant_type_hash"]
pub static PLACEHOLDER_05453: u8 = 0;

#[export_name = "g_variant_type_info_assert_no_infos"]
pub static PLACEHOLDER_05456: u8 = 0;

#[export_name = "g_variant_type_info_element"]
pub static PLACEHOLDER_05459: u8 = 0;

#[export_name = "g_variant_type_info_get"]
pub static PLACEHOLDER_05462: u8 = 0;

#[export_name = "g_variant_type_info_get_type_string"]
pub static PLACEHOLDER_05465: u8 = 0;

#[export_name = "g_variant_type_info_member_info"]
pub static PLACEHOLDER_05468: u8 = 0;

#[export_name = "g_variant_type_info_n_members"]
pub static PLACEHOLDER_05471: u8 = 0;

#[export_name = "g_variant_type_info_query"]
pub static PLACEHOLDER_05474: u8 = 0;

#[export_name = "g_variant_type_info_query_depth"]
pub static PLACEHOLDER_05477: u8 = 0;

#[export_name = "g_variant_type_info_query_element"]
pub static PLACEHOLDER_05480: u8 = 0;

#[export_name = "g_variant_type_info_ref"]
pub static PLACEHOLDER_05483: u8 = 0;

#[export_name = "g_variant_type_info_unref"]
pub static PLACEHOLDER_05486: u8 = 0;

#[export_name = "g_variant_type_is_array"]
pub static PLACEHOLDER_05489: u8 = 0;

#[export_name = "g_variant_type_is_basic"]
pub static PLACEHOLDER_05492: u8 = 0;

#[export_name = "g_variant_type_is_container"]
pub static PLACEHOLDER_05495: u8 = 0;

#[export_name = "g_variant_type_is_definite"]
pub static PLACEHOLDER_05498: u8 = 0;

#[export_name = "g_variant_type_is_dict_entry"]
pub static PLACEHOLDER_05501: u8 = 0;

#[export_name = "g_variant_type_is_maybe"]
pub static PLACEHOLDER_05504: u8 = 0;

#[export_name = "g_variant_type_is_subtype_of"]
pub static PLACEHOLDER_05507: u8 = 0;

#[export_name = "g_variant_type_is_tuple"]
pub static PLACEHOLDER_05510: u8 = 0;

#[export_name = "g_variant_type_is_variant"]
pub static PLACEHOLDER_05513: u8 = 0;

#[export_name = "g_variant_type_key"]
pub static PLACEHOLDER_05516: u8 = 0;

#[export_name = "g_variant_type_n_items"]
pub static PLACEHOLDER_05519: u8 = 0;

#[export_name = "g_variant_type_new"]
pub static PLACEHOLDER_05522: u8 = 0;

#[export_name = "g_variant_type_new_array"]
pub static PLACEHOLDER_05525: u8 = 0;

#[export_name = "g_variant_type_new_dict_entry"]
pub static PLACEHOLDER_05528: u8 = 0;

#[export_name = "g_variant_type_new_maybe"]
pub static PLACEHOLDER_05531: u8 = 0;

#[export_name = "g_variant_type_new_tuple"]
pub static PLACEHOLDER_05534: u8 = 0;

#[export_name = "g_variant_type_next"]
pub static PLACEHOLDER_05537: u8 = 0;

#[export_name = "g_variant_type_peek_string"]
pub static PLACEHOLDER_05540: u8 = 0;

#[export_name = "g_variant_type_string_get_depth_"]
pub static PLACEHOLDER_05543: u8 = 0;

#[export_name = "g_variant_type_string_is_valid"]
pub static PLACEHOLDER_05546: u8 = 0;

#[export_name = "g_variant_type_string_scan"]
pub static PLACEHOLDER_05549: u8 = 0;

#[export_name = "g_variant_type_value"]
pub static PLACEHOLDER_05552: u8 = 0;

#[export_name = "g_variant_unref"]
pub static PLACEHOLDER_05555: u8 = 0;

#[export_name = "g_vasprintf"]
pub static PLACEHOLDER_05558: u8 = 0;

#[export_name = "g_vfprintf"]
pub static PLACEHOLDER_05561: u8 = 0;

#[export_name = "g_vprintf"]
pub static PLACEHOLDER_05564: u8 = 0;

#[export_name = "g_vsnprintf"]
pub static PLACEHOLDER_05567: u8 = 0;

#[export_name = "g_vsprintf"]
pub static PLACEHOLDER_05570: u8 = 0;

#[export_name = "g_warn_message"]
pub static PLACEHOLDER_05573: u8 = 0;

#[export_name = "glib__private__"]
pub static PLACEHOLDER_05576: u8 = 0;

#[export_name = "glib_binary_age"]
pub static PLACEHOLDER_05579: u8 = 0;

#[export_name = "glib_check_version"]
pub static PLACEHOLDER_05582: u8 = 0;

#[export_name = "glib_gettext"]
pub static PLACEHOLDER_05585: u8 = 0;

#[export_name = "glib_interface_age"]
pub static PLACEHOLDER_05588: u8 = 0;

#[export_name = "glib_major_version"]
pub static PLACEHOLDER_05591: u8 = 0;

#[export_name = "glib_mem_profiler_table"]
pub static PLACEHOLDER_05594: u8 = 0;

#[export_name = "glib_micro_version"]
pub static PLACEHOLDER_05597: u8 = 0;

#[export_name = "glib_minor_version"]
pub static PLACEHOLDER_05600: u8 = 0;

#[export_name = "glib_on_error_halt"]
pub static PLACEHOLDER_05603: u8 = 0;

#[export_name = "glib_pgettext"]
pub static PLACEHOLDER_05606: u8 = 0;
