-- SPDX-FileCopyrightText: 2023 Kamila Borowska <kamila@borowska.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE wrappers DROP COLUMN is_formatter;

DELETE FROM wrappers WHERE label = 'autopep8';
