-- SPDX-FileCopyrightText: 2023 Kamila Borowska <kamila@borowska.pw>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

UPDATE languages SET highlighter_mode = 'htmlmixed' WHERE highlighter_mode = 'html';
