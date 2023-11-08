-- SPDX-FileCopyrightText: 2023 Kamila Borowska <kamila@borowska.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

TRUNCATE implementation_wrappers;
DELETE FROM implementations;

ALTER TABLE implementation_wrappers
    DROP CONSTRAINT implementation_wrappers_implementation_id_identifier_key,
    ADD CONSTRAINT implementation_wrappers_identifier_key
    UNIQUE (identifier);
