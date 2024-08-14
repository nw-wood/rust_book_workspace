https://doc.rust-lang.org/book/

many hand written versions of the code examples from the rust book, and some copies 😉

```
v\    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.89s
     Running unittests src/lib.rs (target/debug/deps/_10_generic_types_traits_and_lifetimes-8d29ae1e784fba0c)

running 18 tests
test _10_tests::_0_show_summary ... ok
test _10_tests::_1_constraining_generics_with_impls ... ok
test _10_tests::_1_generics_in_struct_definitions ... ok
test _10_tests::_1_generics_in_method_definitions ... ok
test _10_tests::_1_multiple_generics ... ok
test _10_tests::_1_mixing_generics_with_a_method ... ok
test _10_tests::_2_conditional_method_implementation ... ok
test _10_tests::_2_defining_and_implementing_a_trait ... ok
test _10_tests::_2_overriding_a_default_trait_method ... ok
test _10_tests::_2_return_a_traited_type ... ok
test _10_tests::_2_using_default_implementations ... ok
test _10_tests::_2_using_trait_bound_syntax ... ok
test _10_tests::_2_using_traits_as_function_parameters ... ok
test _10_tests::_3_a_static_lifetime_example ... ok
test _10_tests::_3_generics_traits_and_lifetimes_buffet ... ok
test _10_tests::_3_lifetimes_defined_in_methods ... ok
test _10_tests::_3_lifetimes_in_a_structs_definition ... ok
test _10_tests::_3_lifetimes_in_functions ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_11_writing_automated_tests-39da51d7e555da44)

running 9 tests
test _11_tests::_1_testing_using_result ... ok
test _11_tests::_2_ignoring_a_test ... ignored
test _11_tests::_0_show_summary ... ok
test _11_tests::_1_a_test ... ok
test _11_tests::_1_test_where_panic_is_expectation - should panic ... ok
test _11_tests::_1_panic_test_with_expected_result - should panic ... ok
test _11_tests::_1_using_a_format_result ... ok
test _11_tests::_2_second_test ... ok
test _11_tests::_2_using_a_regular_assert ... ok

test result: ok. 8 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_12_building_a_command_line_program-da4319339078f5d1)

running 5 tests
test _12_tests::_3_handling_a_returned_error ... ignored
test _12_tests::_3_using_process_exit_to_kill_main ... ignored
test _12_tests::_0_show_summary ... ok
test _12_tests::_3_returning_errors_from_a_function ... ok
test _12_tests::_6_arbitrary_write_to_standard_error ... ok

test result: ok. 3 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_13_iterators_and_closures-452e698bb24c43fe)

running 12 tests
test _13_tests::_1_demanding_ownership_with_move ... ignored
test _13_tests::_0_show_summary ... ok
test _13_tests::_1_an_fnmut_as_opposed_to_fnonce ... ok
test _13_tests::_1_borrow_mutably ... ok
test _13_tests::_1_capturing_refs_or_moving_ownership ... ok
test _13_tests::_1_using_sort_by_key ... ok
test _13_tests::_2_capturing_the_env_with_a_closure ... ok
test _13_tests::_2_consuming_an_iterator_with_sum ... ok
test _13_tests::_2_creating_an_iter_from_an_iter_and_collecting ... ok
test _13_tests::_2_iterator_next_method_demo ... ok
test _13_tests::_2_lazy_ol_iterators ... ok
test _13_tests::_1_example_closure ... ok

test result: ok. 11 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 2.00s

     Running unittests src/lib.rs (target/debug/deps/_14_cargo_and_crates_io-4737d2aa140adad5)

running 1 test
test _14_tests::show_summary ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_15_smart_pointers-f4176cc1804ca9db)

running 10 tests
test _15_tests::_2_deref_coercion_in_action ... ok
test _15_tests::_1_using_a_box_t ... ok
test _15_tests::_0_show_summary ... ok
test _15_tests::_1_using_boxes_for_recursion ... ok
test _15_tests::_2_implementing_the_deref_trait ... ok
test _15_tests::_2_using_a_box_like_a_ref ... ok
test _15_tests::_3_dropping_values_early ... ok
test _15_tests::_5_refcell_and_testing_with_a_mock_object ... ok
test _15_tests::_6_avoiding_a_cycle_with_weak ... ok
test _15_tests::_3_implementing_the_drop_trait ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_16_fearless_concurrency-40c09a65157f4254)

running 9 tests
test _16_tests::_0_show_summary ... ok
test _16_tests::_1_spawning_a_thread ... ok
test _16_tests::_1_moving_values_into_threads ... ok
test _16_tests::_1_waiting_for_a_thread ... ok
test _16_tests::_3_using_a_mutex ... ok
test _16_tests::_2_sending_multiple_values_down_a_channel ... ok
test _16_tests::_2_cloning_the_transmitter ... ok
test _16_tests::_3_sharing_a_mutex_with_arc ... ok
test _16_tests::_2_using_channels_with_threads ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_17_object_oreinted_programming_in_rust-2446c5dcb227c398)

running 3 tests
test _17_tests::show_summary ... ok
test _17_tests::_2_common_behavior ... ok
test _17_tests::oop_design_pattern ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_18_patterns_and_matching-eb50d3c7b158aab3)

running 23 tests
test _18_tests::_0_show_summary ... ok
test _18_tests::_1_conditional_if_let_expr ... ok
test _18_tests::_1_pattern_for_destructuring_tuples ... ok
test _18_tests::_1_patterns_in_a_for_loop ... ok
test _18_tests::_1_destructuring_a_functions_params ... ok
test _18_tests::_1_simple_match_expr ... ok
test _18_tests::_1_while_let_conditional_loop ... ok
test _18_tests::_3_destructuring_enums ... ok
test _18_tests::_3_destructuring_an_entire_struct ... ok
test _18_tests::_3_destructuring_nested_enums ... ok
test _18_tests::_3_extra_conditionals_with_match_guards ... ok
test _18_tests::_3_ignoring_an_unused_with_underscore ... ok
test _18_tests::_3_ignoring_params_with_match_again ... ok
test _18_tests::_3_ignoring_params_with_underscores ... ok
test _18_tests::_3_ignore_remaining_with_dot_dot ... ok
test _18_tests::_3_matching_literals ... ok
test _18_tests::_3_making_more_assertions_with_match_guards ... ok
test _18_tests::_3_matching_named_variables ... ok
test _18_tests::_3_matching_with_struct_patterns ... ok
test _18_tests::_3_multi_patterning_syntax ... ok
test _18_tests::_3_range_matching_syntax ... ok
test _18_tests::_3_ranges_work_with_chars ... ok
test _18_tests::_3_using_a_temporary_binding_with_ats ... ok

test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_19_advanced_features-8d8bfe5a2248f2e1)

running 16 tests
test _19_tests::_1_dangerous_function_ex ... ok
test _19_tests::_1_declaring_a_raw_pointer ... ok
test _19_tests::_1_hello_static ... ok
test _19_tests::_1_mutable_static_example ... ok
test _19_tests::_2_disambiguating_by_trait ... ok
test _19_tests::_1_safe_abstraction_over_unsafe_code ... ok
test _19_tests::_2_fully_qualified_syntax ... ok
test _19_tests::_2_trait_constraint_example_with_super ... ok
test _19_tests::_3_using_a_type_alias ... ok
test _19_tests::_2_using_tuple_structs_to_wrap_for_traits ... ok
test _19_tests::_4_alternative_func_for_map ... ok
test _19_tests::_4_closure_return ... ok
test _19_tests::_4_iterating_a_vector_into_another ... ok
test _19_tests::_4_lesser_wizard_summoning ... ok
test _19_tests::_4_providing_a_function_to_another ... ok
test _19_tests::show_summary ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_1_getting_started-a9f6208ecb17d8f7)

running 1 test
test _1::_0_show_summary ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_20_building_a_multithreaded_web_server-10ab813fcd04ae5d)

running 1 test
test _20_tests::show_summary ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_21_appendix-2b9f774c2c30af19)

running 1 test
test _21_tests::show_summary ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_2_programming_a_guessing_game-4b2e6fddff419c9a)

running 2 tests
test _2::_0_show_summary ... ok
test _2::_0_run_guessing_game ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_3_common_programming_concepts-7e2de81f69f027b9)

running 17 tests
test _3::_2_booly_fooly ... ok
test _3::_0_show_summary ... ok
test _3::_1_shadow_length_test ... ok
test _3::_1_mutable_x ... ok
test _3::_1_shadow_x ... ok
test _3::_2_some_basic_operations ... ok
test _3::_2_some_floating_point_types ... ok
test _3::_2_the_array_type ... ok
test _3::_2_the_character_type ... ok
test _3::_2_the_tuple_type ... ok
test _3::_5_basic_for_loops ... ok
test _3::_3_functional_fun ... ok
test _3::_5_if_expressions ... ok
test _3::_5_if_in_let_expressions ... ok
test _3::_5_loop_and_break ... ok
test _3::_5_loop_labels ... ok
test _3::_5_while_conditional_loop ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_4_understanding_ownership-0dd90408f3151391)

running 14 tests
test _4::_1_borrowing_with_some_types ... ok
test _4::_0_show_summary ... ok
test _4::_1_cloning_instead_of_moving ... ok
test _4::_1_return_moved_value_from_function ... ok
test _4::_1_ownership_with_function_params ... ok
test _4::_1_return_s_with_len_as_tup ... ok
test _4::_1_stack_only_copying ... ok
test _4::_1_string_mutation ... ok
test _4::_2_example_reference_usage ... ok
test _4::_3_an_enumerated_iterator ... ok
test _4::_3_first_word_with_slices_instead ... ok
test _4::_2_mutable_references ... ok
test _4::_3_first_word_with_str_signature ... ok
test _4::_3_string_slices ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_5_using_structs_to_structure_related_data-c5f712b7f5c0c29e)

running 14 tests
test _5_tests::_0_show_summary ... ok
test _5_tests::_1_build_struct_with_func ... ok
test _5_tests::_1_build_with_field_init ... ok
test _5_tests::_1_first_mutable_struct ... ok
test _5_tests::_1_first_struct ... ok
test _5_tests::_1_ownership_of_struct_data ... ok
test _5_tests::_1_struct_update_syntax ... ok
test _5_tests::_1_tuple_struct_example ... ok
test _5_tests::_2_factoring_in_structs_in_program_and_derive ... ok
test _5_tests::_2_struct_program ... ok
test _5_tests::_2_struct_program_with_tuples ... ok
test _5_tests::_3_making_a_rectangle_with_associated_function ... ok
test _5_tests::_3_multiple_impl_blocks_and_mutable_ref_methods ... ok
test _5_tests::_3_rectangle_method_examples ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_6_enums_and_pattern_matching-e6f6d8046f1f6b33)

running 9 tests
test _6_tests::_0_show_summary ... ok
test _6_tests::_1_enums_with_varied_types_defined ... ok
test _6_tests::_1_the_option_enum ... ok
test _6_tests::_2_catch_alls_and_placeholders ... ok
test _6_tests::_1_using_an_enum_and_methods ... ok
test _6_tests::_2_matching_with_option_t ... ok
test _6_tests::_2_control_flow_with_match ... ok
test _6_tests::_3_concise_control_flow ... ok
test _6_tests::_3_less_concise_shorthand ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_7_packages_crates_and_modules-4d3ca893b3cf7caf)

running 6 tests
test _7_tests::_2_grouping_related_code_in_mods ... ok
test _7_tests::_0_show_summary ... ok
test _7_tests::_4_pub_use_example ... ok
test _7_tests::_3_exposing_paths_with_pub ... ok
test _7_tests::_4_use_as_another_name ... ok
test _7_tests::referring_to_declared_module_items ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_8_common_collections-359b5e5d6ac1e566)

running 17 tests
test _8_tests::_0_show_summary ... ok
test _8_tests::_1_creating_and_storing_in_a_vec ... ok
test _8_tests::_1_iterating_over_a_vec ... ok
test _8_tests::_1_reading_elements_of_a_vector ... ok
test _8_tests::_1_mutating_while_iterating ... ok
test _8_tests::_1_storing_enums_in_vec ... ok
test _8_tests::_2_concatenation_with_format ... ok
test _8_tests::_2_concatenation_with_string_refs ... ok
test _8_tests::_2_indexing_into_strings ... ok
test _8_tests::_2_iter_chars_instead_of_bytes ... ok
test _8_tests::_2_mutating_a_string ... ok
test _8_tests::_2_to_string_method ... ok
test _8_tests::_3_creating_accessing_hashmaps ... ok
test _8_tests::_3_hashmaps_iteration ... ok
test _8_tests::_3_hashmaps_with_ownership ... ok
test _8_tests::_3_insert_or_and_hashmaps ... ok
test _8_tests::_3_updating_a_hashmap_value ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/_9_error_handling-2baa03ede8a43ad3)

running 7 tests
test _9_tests::_0_show_summary ... ok
test _9_tests::_2_combined_knowledge_of_errors ... ok
test _9_tests::_2_error_propagation ... ok
test _9_tests::_2_using_expect ... ok
test _9_tests::_2_catching_an_error ... ok
test _9_tests::_2_failing_ungracefully ... ok
test _9_tests::_3_panicking_method_call ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/rust_book_utilities-b93972ab673309a8)

running 1 test
test rust_book_utilities_tests::show_summary ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
