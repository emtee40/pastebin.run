-- SPDX-FileCopyrightText: 2023 Kamila Borowska <kamila@borowska.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

ALTER TABLE languages DROP COLUMN hello_world_paste_id, ADD COLUMN hello_world text DEFAULT '';
