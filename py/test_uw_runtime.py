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

    def test_tag_all_prepends_to_every_block(self):
        lst = [("a", [], [(1,)]), ("b", 0, "not a block"), ("c", 0)]
        out = uw.tag_all(lst, 2, lambda e: [(9, e[0])])
        self.assertEqual(out[0], ("a", [], [(9, "a"), (1,)]))
        self.assertIs(out[1], lst[1])
        self.assertIs(out[2], lst[2])
        self.assertEqual(lst[0], ("a", [], [(1,)]))

    def test_unnamed_table_helpers(self):
        lst = ["a", "b", "c"]
        self.assertEqual(uw.insert_before(lst, 0, ["x"]), ["x", "a", "b", "c"])
        self.assertEqual(uw.insert_before(lst, 3, ["x"]), ["a", "b", "c", "x"])
        self.assertEqual(uw.insert_after(lst, 2, ["x"]), ["a", "b", "c", "x"])
        self.assertRaises(uw.UwRuntimeError, uw.insert_after, lst, 3, ["x"])
        self.assertRaises(uw.UwRuntimeError, uw.insert_before, lst, 4, ["x"])
        self.assertEqual(uw.remove_where(lst, lambda i, e: i == 1), ["a", "c"])
        self.assertEqual(lst, ["a", "b", "c"])

    def test_append_entry_keeps_the_existing_entries(self):
        """A simple trigger is `(check_interval, ops)`: unnamed, so it needs no name check."""
        lst = [(24, [(1,)]), (6, [(2,)])]
        out = uw.append_entry(lst, (1, [(3,)]))
        self.assertEqual(out, [(24, [(1,)]), (6, [(2,)]), (1, [(3,)])])
        self.assertIs(out[0], lst[0])
        self.assertEqual(len(lst), 2)


class Menus(unittest.TestCase):
    """Menu layout module_game_menus.py:20-32: (id, flags, text, mesh, enter ops, options)."""

    def setUp(self):
        self.menus = [
            ("camp", 0, "Old", "none", [], [
                ("wait", [], "Wait", [(1, "a")]),
                ("leave", [(2,)], "Leave", [], "Door"),
            ]),
            ("other", 0, "X", "none", [], []),
        ]
        self.before = repr(self.menus)

    def tearDown(self):
        self.assertEqual(repr(self.menus), self.before, "input list was mutated")

    def opts(self, menus):
        return [o[0] for o in menus[0][5]]

    def test_set_text(self):
        out = uw.set_text(self.menus, "camp", 2, "New", key=KEY)
        self.assertEqual(out[0][2], "New")
        self.assertIs(out[1], self.menus[1])

    def test_add_option_positions(self):
        new = ("uw_x", [], "X", [])
        self.assertEqual(self.opts(uw.add_option(self.menus, "camp", 5, new, key=KEY)), ["wait", "leave", "uw_x"])
        self.assertEqual(self.opts(uw.add_option(self.menus, "camp", 5, new, after="wait", key=KEY)), ["wait", "uw_x", "leave"])
        self.assertEqual(self.opts(uw.add_option(self.menus, "camp", 5, new, before="wait", key=KEY)), ["uw_x", "wait", "leave"])
        self.assertRaises(uw.UwRuntimeError, uw.add_option, self.menus, "camp", 5, ("wait", [], "W", []), key=KEY)
        self.assertRaises(uw.UwRuntimeError, uw.add_option, self.menus, "camp", 5, new, after="nope", key=KEY)

    def test_change_option_keeps_unnamed_parts(self):
        out = uw.change_option(self.menus, "camp", 5, "leave", text="Go", key=KEY)
        self.assertEqual(out[0][5][1], ("leave", [(2,)], "Go", [], "Door"))
        out = uw.change_option(self.menus, "camp", 5, "wait", condition=[(3,)], consequence=[], key=KEY)
        self.assertEqual(out[0][5][0], ("wait", [(3,)], "Wait", []))

    def test_remove_option(self):
        self.assertEqual(self.opts(uw.remove_option(self.menus, "camp", 5, "wait", key=KEY)), ["leave"])
        self.assertRaises(uw.UwRuntimeError, uw.remove_option, self.menus, "camp", 5, "nope", key=KEY)

    def test_add_entry(self):
        out = uw.add_entry(self.menus, ("uw_ask", 0, "Q", "none", [], []), key=KEY)
        self.assertEqual([m[0] for m in out], ["camp", "other", "uw_ask"])
        self.assertRaises(uw.UwRuntimeError, uw.add_entry, self.menus, ("camp", 0, "", "none", [], []), key=KEY)


class Dialogs(unittest.TestCase):
    """Dialog lines module_dialogs.py:15-44: (speaker, starting state, conditions, text, ending
    state, consequences[, voice-over]); the kit scans them top to bottom."""

    def setUp(self):
        # The kit's conversation-preparation line: never displayed, it fills $g_talk_troop and the
        # other globals in its condition block (module_dialogs.py:47-188).
        self.preparation = [None, "start", [(1, "store_conversation_troop", "$g_talk_troop"), (31, 1, 0)],
                            "Warning: This line is never displayed.", "close_window", []]
        self.dialogs = [
            self.preparation,
            [None, "start", [], "Do I know you?", "lord_meet_neutral", []],
            [None, "lord_meet_neutral", [], "A name.", "lord_intro", []],
        ]
        self.before = repr(self.dialogs)

    def tearDown(self):
        self.assertEqual(repr(self.dialogs), self.before, "input list was mutated")

    def test_lines_go_before_native_lines_of_the_same_state(self):
        lines = [[None, "start", [], "Who are you?", "uw_first_meeting_0", []]]
        out = uw.insert_before_state(self.dialogs, "start", lines)
        self.assertIsNot(out, self.dialogs)
        self.assertEqual([e[3] for e in out], [
            "Warning: This line is never displayed.",
            "Who are you?",
            "Do I know you?",
            "A name.",
        ])
        self.assertEqual(out[1], lines[0])
        # The preparation line stays at the head, so $g_talk_troop is written before the new line
        # is scanned; every entry the patch does not name stays the same object.
        self.assertIs(out[0], self.preparation)
        self.assertIs(out[2], self.dialogs[1])
        self.assertIs(out[3], self.dialogs[2])

    def test_a_state_native_never_plays_is_an_error(self):
        self.assertRaises(uw.UwRuntimeError, uw.insert_before_state, self.dialogs, "uw_none_0", [[]])


if __name__ == "__main__":
    unittest.main()
