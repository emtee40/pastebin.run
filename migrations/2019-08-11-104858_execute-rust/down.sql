-- SPDX-FileCopyrightText: 2023 Kamila Borowska <kamila@borowska.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

DELETE FROM wrappers WHERE language_id = (SELECT language_id FROM languages WHERE name = 'Rust');
