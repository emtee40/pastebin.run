-- SPDX-FileCopyrightText: 2023 Kamila Borowska <kamila@borowska.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE pastes
    ADD COLUMN stdin text NOT NULL DEFAULT '',
    ADD COLUMN exit_code integer,
    ADD COLUMN stdout text,
    ADD COLUMN stderr text;
