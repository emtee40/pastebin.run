-- SPDX-FileCopyrightText: 2023 Kamila Borowska <kamila@borowska.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE shared_wrappers RENAME TO wrappers;
DROP TABLE implementation_wrappers;
DROP TABLE implementations;
