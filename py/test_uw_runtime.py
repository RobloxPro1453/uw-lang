# -*- coding: utf-8 -*-
"""Unit tests for uw_runtime.py; run with Python 3 or the kit's Python 2.7:

    python py/test_uw_runtime.py
"""
import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import uw_runtime as uw  # noqa: E402

KEY = str.lower  # stand-in for the kit's convert_to_identifier outside the kit


class Runtime(unittest.TestCase):
    def setUp(self):
        self.shared = (0, 0, 0, [], [(1, "script_a")])  # one trigger object used by two entries
        self.lst = [
            ("alpha", 0, [(1, "script_x")], [self.shared]),
            ("Spear", 1, []),
            ("beta", 2, [], [self.shared]),
            ("spear", 3, []),
        ]
        self.before = repr(self.lst)

    def unchanged(self):
        self.assertEqual(repr(self.lst), self.before, "input list was mutated")

    def test_find_plain_and_occurrence(self):
        self.assertEqual(uw.find(self.lst, "alpha", KEY), 0)
        self.assertEqual(uw.find(self.lst, "spear#1", KEY), 1)
        self.assertEqual(uw.find(self.lst, "SPEAR#2", KEY), 3)

    def test_find_refuses_to_guess(self):
        for ref in ("spear", "spear#3", "spear#0", "spear#x", "gamma"):
            self.assertRaises(uw.UwRuntimeError, uw.find, self.lst, ref, KEY)

    def test_append_copies_only_the_changed_path(self):
        out = uw.append_ops(self.lst, "alpha", (3, 0, 4), [(2, "script_b")], KEY)
        self.unchanged()
        self.assertIsNot(out, self.lst)
        for i in (1, 2, 3):
            self.assertIs(out[i], self.lst[i])
        # The other user of the shared trigger keeps the original object, untouched.
        self.assertIs(out[2][3][0], self.shared)
        self.assertEqual(self.shared[4], [(1, "script_a")])
        self.assertEqual(out[0][3][0][4], [(1, "script_a"), (2, "script_b")])
        self.assertIsInstance(out[0], tuple)
        self.assertIsInstance(out[0][3], list)
        self.assertIsInstance(out[0][3][0], tuple)
        self.assertIsInstance(out[0][3][0][4], list)

    def test_prepend_and_replace_body(self):
        out = uw.prepend_ops(self.lst, "alpha", 2, [(9,)], KEY)
        self.assertEqual(out[0][2], [(9,), (1, "script_x")])
        out = uw.replace_body(self.lst, "spear#2", 2, [(7,)], KEY)
        self.assertEqual(out[3][2], [(7,)])
        self.assertIs(out[1], self.lst[1])
        self.unchanged()

    def test_ops_list_is_copied_not_shared(self):
        ops = [(5, "x")]
        out = uw.append_ops(self.lst, "alpha", 2, ops, KEY)
        ops.append((6,))
        self.assertEqual(out[0][2][-1], (5, "x"))

    def test_block_path_must_end_at_a_list(self):
        self.assertRaises(uw.UwRuntimeError, uw.append_ops, self.lst, "alpha", 1, [], KEY)
        self.assertRaises(uw.UwRuntimeError, uw.append_ops, self.lst, "alpha", 9, [], KEY)
        self.unchanged()

    def test_replace_field_and_entry(self):
        out = uw.replace_field(self.lst, "spear#1", 1, 42, KEY)
        self.assertEqual(out[1], ("Spear", 42, []))
        self.assertIs(out[3], self.lst[3])
        out = uw.replace_entry(self.lst, "beta", ("beta", 9), KEY)
        self.assertEqual(out[2], ("beta", 9))
        self.unchanged()

    def test_extend_rejects_name_clash(self):
        out = uw.extend(self.lst, [("gamma", 4)], KEY)
        self.assertEqual(out[-1], ("gamma", 4))
        self.assertEqual(len(out), 5)
        self.assertRaises(uw.UwRuntimeError, uw.extend, self.lst, [("ALPHA", 5)], KEY)
        self.assertRaises(uw.UwRuntimeError, uw.extend, self.lst, [("d", 1), ("D", 2)], KEY)
        self.unchanged()

    def test_unnamed_table_helpers(self):
        lst = ["a", "b", "c"]
        self.assertEqual(uw.insert_before(lst, 0, ["x"]), ["x", "a", "b", "c"])
        self.assertEqual(uw.insert_before(lst, 3, ["x"]), ["a", "b", "c", "x"])
        self.assertEqual(uw.insert_after(lst, 2, ["x"]), ["a", "b", "c", "x"])
        self.assertRaises(uw.UwRuntimeError, uw.insert_after, lst, 3, ["x"])
        self.assertRaises(uw.UwRuntimeError, uw.insert_before, lst, 4, ["x"])
        self.assertEqual(uw.remove_where(lst, lambda i, e: i == 1), ["a", "c"])
        self.assertEqual(lst, ["a", "b", "c"])


if __name__ == "__main__":
    unittest.main()
