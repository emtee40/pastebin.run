-- SPDX-FileCopyrightText: 2023 Kamila Borowska <kamila@borowska.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

UPDATE pastes
    SET language_id = (
        SELECT language_id
        FROM languages
        WHERE identifier = 'python'
    )
    WHERE language_id = (
        SELECT language_id
        FROM languages
        WHERE identifier = 'python2'
    );

DELETE FROM languages WHERE identifier = 'python2';
